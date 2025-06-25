use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use oauth2::{
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenType,
    },
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, StandardRevocableToken, TokenUrl,
};
use reqwest::{header::CONTENT_LENGTH, Method, Url};
use serde::{
    de::{value::BytesDeserializer, DeserializeOwned, IntoDeserializer},
    Serialize,
};
use tracing::info;

use crate::{
    auth::{
        AuthCodeFlow, AuthCodePkceFlow, AuthFlow, AuthenticationState, ClientCredsFlow, Scopes,
        Token, Unauthenticated, UnknownFlow,
    },
    error::{Error, Result, SpotifyError},
};

const AUTHORISATION_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
pub(crate) const API_URL: &str = "https://api.spotify.com/v1";

pub(crate) type OAuthClient = oauth2::Client<
    BasicErrorResponse,
    Token,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
>;

/// A client created using the Authorisation Code Flow.
pub type AuthCodeClient<A> = Client<A, AuthCodeFlow>;

/// A client created using the Authorisation Code with PKCE Flow.
pub type AuthCodePkceClient<A> = Client<A, AuthCodePkceFlow>;

/// A client created using the Client Credentials Flow.
pub type ClientCredsClient<A> = Client<A, ClientCredsFlow>;

#[doc(hidden)]
#[derive(Debug)]
pub(crate) enum Body<P: Serialize = ()> {
    Json(P),
    File(Vec<u8>),
}

/// The client which handles the authentication and all the Spotify API requests.
///
/// It is recommended to use one of the following: [`AuthCodeClient`], [`AuthCodePkceClient`]
/// or [`ClientCredsClient`], depending on the chosen authentication flow.
#[derive(Clone, Debug)]
pub struct Client<A: AuthenticationState, F: AuthFlow> {
    /// Dictates whether or not the client will request a new token when the
    /// current one is about the expire.
    ///
    /// It will check if the token has expired in every request.
    pub auto_refresh: bool,
    // This is used for the typestate pattern, to differentiate an authenticated
    // client from an unauthenticated one, but it also holds the Token.
    pub(crate) auth_state: Arc<RwLock<A>>,
    // This is used for the typestate pattern to differentiate between different
    // authorisation flows, as well as hold the CSRF/PKCE verifiers.
    pub(crate) auth_flow: F,
    // The OAuth2 client.
    pub(crate) oauth: OAuthClient,
    // The HTTP client.
    pub(crate) http: reqwest::Client,
}

impl Client<Token, UnknownFlow> {
    /// Create a new authenticated and authorised client from a refresh token.
    ///
    /// This method will fail if the refresh token is invalid or a new one cannot be obtained.
    pub async fn from_refresh_token(
        client_id: impl Into<String>,
        client_secret: Option<&str>,
        scopes: Option<Scopes>,
        auto_refresh: bool,
        refresh_token: String,
    ) -> Result<Self> {
        let client_id = ClientId::new(client_id.into());
        let client_secret = client_secret.map(|s| ClientSecret::new(s.to_owned()));

        let oauth_client = OAuthClient::new(
            client_id,
            client_secret,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let refresh_token = RefreshToken::new(refresh_token);
        let mut req = oauth_client.exchange_refresh_token(&refresh_token);

        if let Some(scopes) = scopes {
            req = req.add_scopes(scopes.0);
        }

        let mut token = req.request_async(async_http_client).await?.set_timestamps();
        if token.refresh_token.is_none() {
            // "When a refresh token is not returned, continue using the existing token."
            // https://developer.spotify.com/documentation/web-api/tutorials/refreshing-tokens
            token.refresh_token = Some(refresh_token);
        }

        Ok(Self {
            auto_refresh,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow: UnknownFlow,
            oauth: oauth_client,
            http: reqwest::Client::new(),
        })
    }
}

impl<F: AuthFlow> Client<Token, F> {
    /// Get a reference to the client's token.
    ///
    /// Please note that the [RwLock] used here is **not** async-aware, and thus
    /// the read/write guard should not be held across await points.
    pub fn token(&self) -> Arc<RwLock<Token>> {
        self.auth_state.clone()
    }

    /// Get the access token secret as an owned (cloned) string.
    /// If you only need a reference, you can use [`token`](Self::token)
    /// yourself and get a reference from the returned [RwLock].
    ///
    /// This method will fail if the `RwLock` that holds the token has
    /// been poisoned.
    pub fn access_token(&self) -> Result<String> {
        let token = self
            .auth_state
            .read()
            .expect("The lock holding the token has been poisoned.");

        Ok(token.access_token.secret().clone())
    }

    /// Get the refresh token secret as an owned (cloned) string.
    /// If you only need a reference, you can use [`token`](Self::token)
    /// yourself and get a reference from the returned [RwLock].
    ///
    /// This method will fail if the `RwLock` that holds the token has
    /// been poisoned.
    pub fn refresh_token(&self) -> Result<Option<String>> {
        let token = self
            .auth_state
            .read()
            .expect("The lock holding the token has been poisoned.");

        let refresh_token = token.refresh_token.as_ref().map(|t| t.secret().clone());

        Ok(refresh_token)
    }

    /// Exchange the refresh token for a new access token and updates it in the client.
    /// Only some auth flows allow for token refreshing.
    pub async fn exchange_refresh_token(&self) -> Result<()> {
        let refresh_token = {
            let lock = self.auth_state.read().unwrap_or_else(|e| e.into_inner());

            let Some(refresh_token) = &lock.refresh_token else {
                return Err(Error::RefreshUnavailable);
            };

            refresh_token.clone()
        };

        let mut token = self
            .oauth
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await?
            .set_timestamps();
        if token.refresh_token.is_none() {
            token.refresh_token = Some(refresh_token);
        }

        let mut lock = self
            .auth_state
            .write()
            .expect("The lock holding the token has been poisoned.");
        *lock = token;
        Ok(())
    }

    pub(crate) async fn request<P: Serialize + Debug, T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: String,
        query: Option<P>,
        body: Option<Body<P>>,
    ) -> Result<T> {
        let (token_expired, secret) = {
            let lock = self
                .auth_state
                .read()
                .expect("The lock holding the token has been poisoned.");

            (lock.is_expired(), lock.access_token.secret().to_owned())
        };

        if token_expired {
            if self.auto_refresh {
                info!("The token has expired, attempting to refresh...");

                self.exchange_refresh_token().await?;

                let lock = self
                    .auth_state
                    .read()
                    .expect("The lock holding the token has been poisoned.");

                info!("The token has been successfully refreshed. The new token will expire in {} seconds", lock.expires_in);
            } else {
                info!("The token has expired, automatic refresh is disabled.");
                return Err(Error::ExpiredToken);
            }
        }

        let mut req = {
            self.http
                .request(method, format!("{API_URL}{endpoint}"))
                .bearer_auth(secret)
        };

        if let Some(q) = query {
            req = req.query(&q);
        }

        if let Some(b) = body {
            match b {
                Body::Json(j) => req = req.json(&j),
                Body::File(f) => req = req.body(f),
            }
        } else {
            // Used because Spotify wants a Content-Length header for the PUT /audiobooks/me endpoint even though there is no body
            // If not supplied, it will return an error in the form of HTML (not JSON), which I believe to be an issue on their end.
            // No other endpoints so far behave this way.
            req = req.header(CONTENT_LENGTH, 0);
        }

        let req = req.build()?;
        info!(headers = ?req.headers(), "{} request sent to {}", req.method(), req.url());

        let res = self.http.execute(req).await?;

        if res.status().is_success() {
            let bytes = res.bytes().await?;

            // Try to deserialize from bytes of JSON text;
            let deserialized = serde_json::from_slice::<T>(&bytes).or_else(|e| {
                // if the previous operation fails, try deserializing straight
                // from the bytes, which works for Nil.
                let de: BytesDeserializer<'_, serde::de::value::Error> =
                    bytes.as_ref().into_deserializer();

                // This line also converts the serde::de::value::Error to a serde_json::Error
                // to make it clearer to the end user that deserialization failed.
                T::deserialize(de).map_err(|_| e)
            });
            // .context(DeserializationSnafu { body });

            match deserialized {
                Ok(content) => Ok(content),
                Err(err) => {
                    let body = std::str::from_utf8(&bytes).map_err(|_| Error::InvalidResponse)?;

                    tracing::error!(
                        %body,
                        "Failed to deserialize the response body into an object or Nil."
                    );

                    Err(Error::Deserialization {
                        source: err,
                        body: body.to_owned(),
                    })
                }
            }
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    pub(crate) async fn get<P: Serialize + Debug, T: DeserializeOwned>(
        &self,
        endpoint: String,
        query: impl Into<Option<P>>,
    ) -> Result<T> {
        self.request(Method::GET, endpoint, query.into(), None)
            .await
    }

    pub(crate) async fn post<P: Serialize + Debug, T: DeserializeOwned>(
        &self,
        endpoint: String,
        body: impl Into<Option<Body<P>>>,
    ) -> Result<T> {
        self.request(Method::POST, endpoint, None, body.into())
            .await
    }

    pub(crate) async fn put<P: Serialize + Debug, T: DeserializeOwned>(
        &self,
        endpoint: String,
        body: impl Into<Option<Body<P>>>,
    ) -> Result<T> {
        self.request(Method::PUT, endpoint, None, body.into()).await
    }

    pub(crate) async fn delete<P: Serialize + Debug, T: DeserializeOwned>(
        &self,
        endpoint: String,
        body: impl Into<Option<Body<P>>>,
    ) -> Result<T> {
        self.request(Method::DELETE, endpoint, None, body.into())
            .await
    }
}

impl AuthCodeClient<Unauthenticated> {
    /// Create a new client and generate an authorisation URL
    ///
    /// You must redirect the user to the returned URL, which in turn redirects them to
    /// the `redirect_uri` you provided, along with a `code` and `state` parameter in the URl.
    ///
    /// They are required for the next step in the auth process.
    pub fn new<S>(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        scopes: S,
        redirect_uri: RedirectUrl,
        auto_refresh: bool,
    ) -> (Self, Url)
    where
        S: Into<Scopes>,
    {
        let client_id = ClientId::new(client_id.into());
        let client_secret = Some(ClientSecret::new(client_secret.into()));

        let oauth = OAuthClient::new(
            client_id,
            client_secret,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        )
        .set_redirect_uri(redirect_uri);

        let (auth_url, csrf_token) = oauth
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into().0)
            .url();

        (
            Client {
                auto_refresh,
                auth_state: Arc::new(RwLock::new(Unauthenticated)),
                auth_flow: AuthCodeFlow { csrf_token },
                oauth,
                http: reqwest::Client::new(),
            },
            auth_url,
        )
    }

    /// This will exchange the `auth_code` for a token which will allow the client
    /// to make requests.
    ///
    /// `csrf_state` is used for CSRF protection.
    pub async fn authenticate(
        self,
        auth_code: impl Into<String>,
        csrf_state: impl AsRef<str>,
    ) -> Result<Client<Token, AuthCodeFlow>> {
        let auth_code = auth_code.into().trim().to_owned();
        let csrf_state = csrf_state.as_ref().trim();

        if csrf_state != self.auth_flow.csrf_token.secret() {
            return Err(Error::InvalidStateParameter);
        }

        let token = self
            .oauth
            .exchange_code(AuthorizationCode::new(auth_code))
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        Ok(Client {
            auto_refresh: self.auto_refresh,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow: self.auth_flow,
            oauth: self.oauth,
            http: self.http,
        })
    }
}

impl AuthCodePkceClient<Unauthenticated> {
    /// Create a new client and generate an authorisation URL
    ///
    /// You must redirect the user to the received URL, which in turn redirects them to
    /// the redirect URI you provided, along with a `code` and `state` parameter in the URl.
    ///
    /// They are required for the next step in the auth process.
    pub fn new<T, S>(
        client_id: T,
        scopes: S,
        redirect_uri: RedirectUrl,
        auto_refresh: bool,
    ) -> (Self, Url)
    where
        T: Into<String>,
        S: Into<Scopes>,
    {
        let client_id = ClientId::new(client_id.into());

        let oauth = OAuthClient::new(
            client_id,
            None,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        )
        .set_redirect_uri(redirect_uri);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = oauth
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into().0)
            .set_pkce_challenge(pkce_challenge)
            .url();

        (
            Client {
                auto_refresh,
                auth_state: Arc::new(RwLock::new(Unauthenticated)),
                auth_flow: AuthCodePkceFlow {
                    csrf_token,
                    pkce_verifier: Some(pkce_verifier),
                },
                oauth,
                http: reqwest::Client::new(),
            },
            auth_url,
        )
    }

    /// This will exchange the `auth_code` for a token which will allow the client
    /// to make requests.
    ///
    /// `csrf_state` is used for CSRF protection.
    pub async fn authenticate(
        mut self,
        auth_code: impl Into<String>,
        csrf_state: impl AsRef<str>,
    ) -> Result<Client<Token, AuthCodePkceFlow>> {
        let auth_code = auth_code.into().trim().to_owned();
        let csrf_state = csrf_state.as_ref().trim();

        if csrf_state != self.auth_flow.csrf_token.secret() {
            return Err(Error::InvalidStateParameter);
        }

        let Some(pkce_verifier) = self.auth_flow.pkce_verifier.take() else {
            // This should never be reached realistically, but an error
            // will be thrown and log issued just in case.
            tracing::error!(client = ?self, "No PKCE code verifier present when authenticating the client.");
            return Err(Error::InvalidClientState);
        };

        let token = self
            .oauth
            .exchange_code(AuthorizationCode::new(auth_code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        Ok(Client {
            auto_refresh: self.auto_refresh,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow: self.auth_flow,
            oauth: self.oauth,
            http: self.http,
        })
    }
}

impl ClientCredsClient<Unauthenticated> {
    /// This will exchange the client credentials for an access token used
    /// to make requests.
    ///
    /// This authentication method doesn't allow for token refreshing or to access
    /// user resources.
    pub async fn authenticate(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Result<ClientCredsClient<Token>> {
        let client_id = ClientId::new(client_id.into());
        let client_secret = Some(ClientSecret::new(client_secret.into()));

        let oauth = OAuthClient::new(
            client_id,
            client_secret,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let token = oauth
            .exchange_client_credentials()
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        Ok(Client {
            auto_refresh: false,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow: ClientCredsFlow,
            oauth,
            http: reqwest::Client::new(),
        })
    }
}

impl AuthCodeClient<Token> {
    /// Create a new authenticated client from an access token.
    /// This client will be able to access user data.
    ///
    /// This method will fail if the access token is invalid (a request will
    /// be sent to check the token).
    pub async fn from_access_token(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        auto_refresh: bool,
        token: Token,
    ) -> Result<Self> {
        let client_id = ClientId::new(client_id.into());
        // client_secret.map(|s| ClientSecret::new(s.to_owned()));
        let client_secret = Some(ClientSecret::new(client_secret.into()));

        let oauth_client = OAuthClient::new(
            client_id,
            client_secret,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let http = reqwest::Client::new();

        // This is just a bogus request to check if the token is valid.
        let res = http
            .get(format!("{API_URL}/markets"))
            .bearer_auth(token.secret())
            .header(CONTENT_LENGTH, 0)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(res.json::<SpotifyError>().await?.into());
        }

        let auth_flow = AuthCodeFlow {
            csrf_token: CsrfToken::new("not needed".to_owned()),
        };

        let auto_refresh = auto_refresh && token.refresh_token.is_some();

        Ok(Self {
            auto_refresh,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow,
            oauth: oauth_client,
            http,
        })
    }
}

impl AuthCodePkceClient<Token> {
    /// Create a new authenticated client from an access token.
    /// This client will be able to access user data.
    ///
    /// This method will fail if the access token is invalid (a request will
    /// be sent to check the token).
    pub async fn from_access_token(
        client_id: impl Into<String>,
        auto_refresh: bool,
        token: Token,
    ) -> Result<Self> {
        let client_id = ClientId::new(client_id.into());

        let oauth_client = OAuthClient::new(
            client_id,
            None,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let http = reqwest::Client::new();

        // This is just a bogus request to check if the token is valid.
        let res = http
            .get(format!("{API_URL}/recommendations/available-genre-seeds"))
            .bearer_auth(token.secret())
            .header(CONTENT_LENGTH, 0)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(res.json::<SpotifyError>().await?.into());
        }

        let auth_flow = AuthCodePkceFlow {
            csrf_token: CsrfToken::new("not needed".to_owned()),
            pkce_verifier: None,
        };

        let auto_refresh = auto_refresh && token.refresh_token.is_some();

        Ok(Self {
            auto_refresh,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow,
            oauth: oauth_client,
            http,
        })
    }
}

impl ClientCredsClient<Token> {
    /// Create a new authenticated client from an access token.
    /// This client will not be able to access user data.
    ///
    /// This method will fail if the access token is invalid (a request will
    /// be sent to check the token).
    pub async fn from_access_token(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        token: Token,
    ) -> Result<Self> {
        let client_id = ClientId::new(client_id.into());
        let client_secret = Some(ClientSecret::new(client_secret.into()));

        let oauth_client = OAuthClient::new(
            client_id,
            client_secret,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let http = reqwest::Client::new();

        // This is just a bogus request to check if the token is valid.
        let res = http
            .get(format!("{API_URL}/recommendations/available-genre-seeds"))
            .bearer_auth(token.secret())
            .header(CONTENT_LENGTH, 0)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(res.json::<SpotifyError>().await?.into());
        }

        Ok(Self {
            auto_refresh: false,
            auth_state: Arc::new(RwLock::new(token)),
            auth_flow: ClientCredsFlow,
            oauth: oauth_client,
            http,
        })
    }
}
