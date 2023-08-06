use std::{fmt::Debug, marker::PhantomData};

use oauth2::{
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenType,
    },
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, RedirectUrl, StandardRevocableToken,
};
use reqwest::{Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::{
    auth::{
        AuthCodeGrantFlow, AuthCodeGrantPKCEFlow, AuthFlow, AuthenticationState, Authorisation,
        AuthorisationPKCE, Authorised, Scope, Token, UnAuthenticated,
    },
    error::{Error, SpotifyError},
    model::{
        album::{Album, Albums, PagedAlbums, SavedAlbum, SimplifiedAlbum},
        track::SimplifiedTrack,
        Page,
    },
    query::album::{AlbumQuery, AlbumTracksQuery, AlbumsQuery, NewReleaseQuery, SavedAlbumsQuery},
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

#[derive(Debug)]
pub struct Client<A: AuthenticationState, F: AuthFlow> {
    pub auto_refresh: bool,
    pub(crate) auth: A,
    pub(crate) oauth: OAuthClient,
    pub(crate) http: reqwest::Client,
    marker: PhantomData<F>,
}

impl<F: AuthFlow> Client<UnAuthenticated, F> {
    pub fn new(
        auth_flow: F,
        redirect_uri: RedirectUrl,
        auto_refresh: bool,
    ) -> Client<UnAuthenticated, F> {
        let oauth_client = OAuthClient::new(
            auth_flow.client_id(),
            auth_flow.client_secret(),
            AuthUrl::new("https://accounts.spotify.com/authorize".to_owned()).unwrap(),
            auth_flow.token_url(),
        )
        .set_redirect_uri(redirect_uri);

        Client {
            auto_refresh,
            auth: UnAuthenticated,
            oauth: oauth_client,
            http: reqwest::Client::new(),
            marker: PhantomData,
        }
    }
}

impl<F: AuthFlow> Client<Token, F> {
    pub fn access_token(&self) -> &str {
        self.auth.access_token.secret()
    }

    pub fn refresh_token(&self) -> Option<&str> {
        self.auth
            .refresh_token
            .as_ref()
            .map(|t| t.secret().as_str())
    }

    pub async fn request_refresh_token(&mut self) -> Result<()> {
        let Some(refresh_token) = &self.auth.refresh_token else {
            return Err(Error::RefreshUnavailable);
        };

        let token = self
            .oauth
            .exchange_refresh_token(refresh_token)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        self.auth = token;
        Ok(())
    }

    pub(crate) async fn request<Q: Serialize + Debug>(
        &mut self,
        method: Method,
        endpoint: &str,
        query: Option<Q>,
        json: Option<Value>,
    ) -> Result<RequestBuilder> {
        if self.auth.is_expired() {
            if self.auto_refresh {
                self.request_refresh_token().await?
            }

            return Err(Error::ExpiredToken);
        }

        let mut req = self
            .http
            .request(method, format!("https://api.spotify.com/v1{endpoint}"))
            .bearer_auth(self.auth.access_token.secret());

        if let Some(q) = query {
            req = req.query(&q);
        }

        if let Some(j) = json {
            req = req.json(&j);
        }

        Ok(req)
    }

    pub(crate) async fn get<Q: Serialize + Debug, T: DeserializeOwned + Debug>(
        &mut self,
        endpoint: &str,
        query: impl Into<Option<Q>>,
        json: impl Into<Option<Value>>,
    ) -> Result<T> {
        Ok(self
            .request(Method::GET, endpoint, query.into(), json.into())
            .await?
            .send()
            .await?
            .json()
            .await?)
    }

    pub(crate) async fn post<Q: Serialize + Debug>(
        &mut self,
        endpoint: &str,
        query: impl Into<Option<Q>>,
        json: impl Into<Option<Value>>,
    ) -> Result<()> {
        let res = self
            .request(Method::POST, endpoint, query.into(), json.into())
            .await?
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    pub(crate) async fn put<Q: Serialize + Debug>(
        &mut self,
        endpoint: &str,
        query: impl Into<Option<Q>>,
        json: impl Into<Option<Value>>,
    ) -> Result<()> {
        let res = self
            .request(Method::PUT, endpoint, query.into(), json.into())
            .await?
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    pub(crate) async fn delete<Q: Serialize + Debug>(
        &mut self,
        endpoint: &str,
        query: impl Into<Option<Q>>,
        json: impl Into<Option<Value>>,
    ) -> Result<()> {
        let res = self
            .request(Method::DELETE, endpoint, query.into(), json.into())
            .await?
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    pub async fn get_album(&mut self, query: AlbumQuery) -> Result<Album> {
        self.get(&format!("/albums/{}", query.album_id), query, None)
            .await
    }

    pub async fn get_albums(&mut self, query: AlbumsQuery) -> Result<Vec<Album>> {
        self.get("/albums", query, None)
            .await
            .map(|a: Albums| a.albums)
    }

    pub async fn get_album_tracks(
        &mut self,
        query: AlbumTracksQuery,
    ) -> Result<Page<SimplifiedTrack>> {
        self.get(&format!("/albums/{}/tracks", query.album_id), query, None)
            .await
    }

    pub async fn get_new_releases(
        &mut self,
        query: NewReleaseQuery,
    ) -> Result<Page<SimplifiedAlbum>> {
        self.get("/browse/new-releases/", query, None)
            .await
            .map(|a: PagedAlbums| a.albums)
    }
}

impl<F: AuthFlow + Authorised> Client<Token, F> {
    pub async fn get_saved_albums(&mut self, query: SavedAlbumsQuery) -> Result<Page<SavedAlbum>> {
        self.get("/me/albums", query, None).await
    }

    pub async fn save_albums(&mut self, album_ids: &[&str]) -> Result<()> {
        self.put::<()>("/me/albums/", None, json!({ "ids": album_ids }))
            .await
    }

    pub async fn remove_saved_albums(&mut self, album_ids: &[&str]) -> Result<()> {
        self.delete::<()>("/me/albums/", None, json!({ "ids": album_ids }))
            .await
    }

    pub async fn check_saved_albums(&mut self, album_ids: &[&str]) -> Result<Vec<bool>> {
        self.get("/me/albums/contains", [("ids", album_ids.join(","))], None)
            .await
    }
}

impl Client<UnAuthenticated, AuthCodeGrantPKCEFlow> {
    pub fn get_authorisation<I: IntoIterator>(&self, scopes: I) -> AuthorisationPKCE
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

        AuthorisationPKCE {
            url: auth_url,
            csrf_token,
            pkce_verifier,
        }
    }

    pub async fn request_token(
        self,
        auth: AuthorisationPKCE,
        auth_code: AuthorizationCode,
        csrf_state: &str,
    ) -> Result<Client<Token, AuthCodeGrantPKCEFlow>> {
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

        Ok(Client {
            auto_refresh: self.auto_refresh,
            auth: token,
            oauth: self.oauth,
            http: self.http,
            marker: PhantomData,
        })
    }
}

impl Client<UnAuthenticated, AuthCodeGrantFlow> {
    pub fn get_authorisation<I: IntoIterator>(&self, scopes: I) -> Authorisation
    where
        I::Item: Into<Scope>,
    {
        let (auth_url, csrf_token) = self
            .oauth
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter().map(|i| i.into().0))
            .url();

        Authorisation {
            url: auth_url,
            csrf_token,
        }
    }

    pub async fn request_token(
        self,
        auth: Authorisation,
        auth_code: AuthorizationCode,
        csrf_state: &str,
    ) -> Result<Client<Token, AuthCodeGrantFlow>> {
        if csrf_state != auth.csrf_token.secret() {
            return Err(Error::InvalidStateParameter);
        }

        let token = self
            .oauth
            .exchange_code(auth_code)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        Ok(Client {
            auto_refresh: self.auto_refresh,
            auth: token,
            oauth: self.oauth,
            http: self.http,
            marker: PhantomData,
        })
    }
}
