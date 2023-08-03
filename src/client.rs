use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use oauth2::{
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenType,
    },
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, RedirectUrl, StandardRevocableToken,
};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::{
    auth::{AuthCodeGrantPKCEFlow, AuthFlow, Authorisation, Authorised, Scope, Token},
    error::{Error, SpotifyError},
    model::{
        album::{Album, Albums, PagedAlbums, SavedAlbum, SimplifiedAlbum},
        track::SimplifiedTrack,
        Page,
    },
    Result,
};

pub(crate) type OAuthClient = oauth2::Client<
    BasicErrorResponse,
    Token,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
>;

fn query_map<K: Hash + Eq, V, const N: usize>(queries: [Option<(K, V)>; N]) -> HashMap<K, V> {
    HashMap::from_iter(queries.into_iter().flatten())
}

#[derive(Debug)]
pub struct Client<F: AuthFlow> {
    pub auto_refresh: bool,
    pub(crate) token: Option<Token>,
    pub(crate) oauth: OAuthClient,
    pub(crate) http: reqwest::Client,
    marker: PhantomData<F>,
}

impl<F: AuthFlow + Sync> Client<F> {
    pub fn new(auth_flow: F, redirect_uri: RedirectUrl, auto_refresh: bool) -> Client<F> {
        let oauth_client = OAuthClient::new(
            auth_flow.client_id(),
            auth_flow.client_secret(),
            AuthUrl::new("https://accounts.spotify.com/authorize".to_owned()).unwrap(),
            auth_flow.token_url(),
        )
        .set_redirect_uri(redirect_uri);

        Client {
            auto_refresh,
            token: None,
            oauth: oauth_client,
            http: reqwest::Client::new(),
            marker: PhantomData,
        }
    }

    pub fn access_token(&self) -> Option<&String> {
        self.token.as_ref().map(|t| t.access_token.secret())
    }

    pub fn refresh_token(&self) -> Option<&String> {
        self.token
            .as_ref()
            .and_then(|t| t.refresh_token.as_ref())
            .map(|t| t.secret())
    }

    async fn request_refresh(&mut self) -> std::result::Result<(), Error> {
        let Some(token) = &self.token else {
            return Err(Error::NotAuthenticated)
        };

        let Some(refresh_token) = &token.refresh_token else {
            return Err(Error::RefreshUnavailable);
        };

        let token = self
            .oauth
            .exchange_refresh_token(refresh_token)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        self.token = Some(token);
        Ok(())
    }

    async fn request<Q: Serialize + ?Sized>(
        &mut self,
        method: Method,
        endpoint: &str,
        query: Option<&Q>,
        json: &Option<Value>,
    ) -> Result<RequestBuilder> {
        let Some(token) = &self.token else {
            return Err(Error::NotAuthenticated)
        };

        if token.is_expired() {
            if self.auto_refresh {
                self.request_refresh().await?
            }

            return Err(Error::ExpiredToken);
        }

        let mut req = self
            .http
            .request(method, format!("https://api.spotify.com/v1{endpoint}"))
            .bearer_auth(token.access_token.secret());

        if let Some(q) = query {
            req = req.query(q);
        }

        if let Some(j) = json {
            req = req.json(j);
        }

        Ok(req)
    }

    async fn get<Q: Serialize + ?Sized, T: DeserializeOwned>(
        &mut self,
        endpoint: &str,
        query: Option<&Q>,
        json: Option<Value>,
    ) -> Result<T> {
        Ok(self
            .request(Method::GET, endpoint, query, &json)
            .await?
            .send()
            .await?
            .json()
            .await?)
    }

    async fn post<Q: Serialize + ?Sized>(
        &mut self,
        endpoint: &str,
        query: Option<&Q>,
        json: Option<Value>,
    ) -> Result<()> {
        let res = self
            .request(Method::POST, endpoint, query, &json)
            .await?
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    async fn put<Q: Serialize + ?Sized>(
        &mut self,
        endpoint: &str,
        query: Option<&Q>,
        json: Option<Value>,
    ) -> Result<()> {
        let res = self
            .request(Method::PUT, endpoint, query, &json)
            .await?
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    async fn delete<Q: Serialize + ?Sized>(
        &mut self,
        endpoint: &str,
        query: Option<&Q>,
        json: Option<Value>,
    ) -> Result<()> {
        let res = self
            .request(Method::DELETE, endpoint, query, &json)
            .await?
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    pub async fn get_album(&mut self, album_id: &str, market: Option<&str>) -> Result<Album> {
        let market = market.map(|m| [("market", m)]);
        self.get(&format!("/albums/{album_id}"), market.as_ref(), None)
            .await
    }

    pub async fn get_albums(
        &mut self,
        album_ids: &[&str],
        market: Option<&str>,
    ) -> Result<Vec<Album>> {
        let mut query = HashMap::from([("ids", album_ids.join(","))]);

        if let Some(market) = market {
            query.insert("market", market.to_owned());
        }

        self.get("/albums/", Some(&query), None)
            .await
            .map(|a: Albums| a.albums)
    }

    pub async fn get_album_tracks(
        &mut self,
        album_id: &str,
        market: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Page<SimplifiedTrack>> {
        let limit = limit.map(|l| ("limit", l.to_string()));
        let offset = offset.map(|o| ("offset", o.to_string()));
        let market = market.map(|m| ("market", m.to_string()));

        self.get(
            &format!("/albums/{album_id}/tracks"),
            Some(&query_map([market, offset, limit])),
            None,
        )
        .await
    }

    pub async fn get_new_releases(
        &mut self,
        country: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Page<SimplifiedAlbum>> {
        let country = country.map(|m| ("country", m.to_string()));
        let limit = limit.map(|l| ("limit", l.to_string()));
        let offset = offset.map(|o| ("offset", o.to_string()));

        self.get(
            "/browse/new-releases/",
            Some(&query_map([country, limit, offset])),
            None,
        )
        .await
        .map(|a: PagedAlbums| a.albums)
    }
}

impl<F: AuthFlow + Authorised + Sync> Client<F> {
    pub async fn get_saved_albums(
        &mut self,
        limit: Option<u32>,
        offset: Option<u32>,
        market: Option<&str>,
    ) -> Result<Page<SavedAlbum>> {
        let limit = limit.map(|l| ("limit", l.to_string()));
        let offset = offset.map(|o| ("offset", o.to_string()));
        let market = market.map(|m| ("market", m.to_string()));

        self.get(
            "/me/albums",
            Some(&query_map([limit, offset, market])),
            None,
        )
        .await
    }

    pub async fn save_albums(&mut self, album_ids: &[&str]) -> Result<()> {
        self.put::<()>("/me/albums/", None, Some(json!({ "ids": album_ids })))
            .await
    }

    pub async fn remove_saved_albums(&mut self, album_ids: &[&str]) -> Result<()> {
        self.delete::<()>("/me/albums/", None, Some(json!({ "ids": album_ids })))
            .await
    }

    pub async fn check_saved_albums(&mut self, album_ids: &[&str]) -> Result<Vec<bool>> {
        self.get(
            "/me/albums/contains",
            Some(&[("ids", album_ids.join(","))]),
            None,
        )
        .await
    }
}

impl Client<AuthCodeGrantPKCEFlow> {
    pub fn get_authorisation<I: IntoIterator>(&self, scopes: I) -> Authorisation
    where
        I::Item: Into<Scope>,
    {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .oauth
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter().map(|i| i.into().0))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Authorisation {
            url: auth_url,
            csrf_token,
            pkce_verifier,
        }
    }

    pub async fn request_token(
        &mut self,
        auth: Authorisation,
        auth_code: AuthorizationCode,
        csrf_state: &str,
    ) -> Result<Token> {
        if csrf_state != auth.csrf_token.secret() {
            return Err(Error::InvalidStateParameter);
        }

        let token = self
            .oauth
            .exchange_code(auth_code)
            .set_pkce_verifier(auth.pkce_verifier)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        self.token = Some(token.clone());
        Ok(token)
    }

    pub async fn request_token_refresh(&mut self) -> Result<Token> {
        let Some(token) = &self.token else {
            return Err(Error::NotAuthenticated)
        };

        let Some(refresh_token) = &token.refresh_token else {
            return Err(Error::RefreshUnavailable);
        };

        let token = self
            .oauth
            .exchange_refresh_token(refresh_token)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        self.token = Some(token.clone());
        Ok(token)
    }
}
