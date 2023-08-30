use std::time::Duration;

use chrono::{DateTime, Utc};
use oauth2::{
    basic::BasicTokenType, AccessToken, ClientId, ClientSecret, CsrfToken, PkceCodeVerifier,
    RefreshToken, TokenResponse, TokenUrl,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub trait AuthFlow {
    fn client_id(&self) -> ClientId;
    fn client_secret(&self) -> Option<ClientSecret>;
    fn token_url(&self) -> Option<TokenUrl> {
        Some(TokenUrl::new("https://accounts.spotify.com/api/token".to_owned()).unwrap())
    }
}

pub trait AuthenticationState {}
impl AuthenticationState for Token {}
impl AuthenticationState for UnAuthenticated {}

pub trait Authorised {}
impl Authorised for AuthCodeGrantPKCEFlow {}
impl Authorised for AuthCodeGrantFlow {}

/// A Spotify token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// The token used for authenticating every single request.
    pub access_token: AccessToken,
    /// The token used for requesting a new access token when the current one expires.
    pub refresh_token: Option<RefreshToken>,
    /// How long until the current token expires, in seconds.
    pub expires_in: u64,

    #[serde(skip)]
    /// The UTC date and time when the token was created.
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    /// The UTC date and time when the token will expire.
    pub expires_at: DateTime<Utc>,

    #[serde(deserialize_with = "oauth2::helpers::deserialize_untagged_enum_case_insensitive")]
    pub(crate) token_type: BasicTokenType,
    #[serde(rename = "scope")]
    #[serde(deserialize_with = "oauth2::helpers::deserialize_space_delimited_vec")]
    #[serde(serialize_with = "oauth2::helpers::serialize_space_delimited_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) scopes: Option<Vec<oauth2::Scope>>,
}

#[derive(Clone, Copy, Debug)]
pub struct UnAuthenticated;

/// Authorisation code flow with PKCE is the recommended authorization flow
/// if you’re implementing authorization in any other type of application
/// where the client secret can’t be safely stored.
#[derive(Clone, Debug)]
pub struct AuthCodeGrantPKCEFlow {
    pub client_id: String,
}
#[derive(Clone, Debug)]

/// The authorisation code flow is suitable for long-running applications
/// (e.g. web and mobile apps) where the user grants permission only once.
///
/// If you’re using the authorisation code flow in any type of application
/// where the client secret can't be safely stored, then you should use the PKCE extension.
pub struct AuthCodeGrantFlow {
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Clone, Debug)]

/// The Client Credentials flow is used in server-to-server authentication.
///
/// Since this flow does not include authorisation, only endpoints that
/// do not access user information can be accessed.
pub struct ClientCredsGrantFlow {
    pub client_id: String,
    pub client_secret: String,
}

/// Holds the URL, CSRF token and PKCE verifier.
#[derive(Debug)]
pub struct AuthorisationPKCE {
    pub url: Url,
    pub(crate) csrf_token: CsrfToken,
    pub(crate) pkce_verifier: PkceCodeVerifier,
}

/// Holds the URL, and CSRF token..
#[derive(Debug)]
pub struct Authorisation {
    pub url: Url,
    pub(crate) csrf_token: CsrfToken,
}

/// A Spotify [scope](https://developer.spotify.com/documentation/web-api/concepts/scopes).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scope(pub(crate) oauth2::Scope);

impl Token {
    pub(crate) fn set_timestamps(self) -> Self {
        let created_at = Utc::now();
        let expires_at = created_at
            + chrono::Duration::seconds(i64::try_from(self.expires_in).unwrap_or(i64::MAX));

        Self {
            created_at,
            expires_at,
            ..self
        }
    }

    /// Returns `true` if the access token has expired.
    pub fn is_expired(&self) -> bool {
        self.created_at >= self.expires_at
    }

    /// Returns `true` if a refresh token is present.
    pub fn is_refreshable(&self) -> bool {
        self.refresh_token.is_some()
    }
}

impl TokenResponse<BasicTokenType> for Token {
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    fn token_type(&self) -> &BasicTokenType {
        &self.token_type
    }

    fn expires_in(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.expires_in))
    }

    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    fn scopes(&self) -> Option<&Vec<oauth2::Scope>> {
        self.scopes.as_ref()
    }
}

impl AuthFlow for AuthCodeGrantPKCEFlow {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.client_id.clone())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        None
    }
}

impl AuthFlow for AuthCodeGrantFlow {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.client_id.clone())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        Some(ClientSecret::new(self.client_secret.clone()))
    }
}

impl AuthFlow for ClientCredsGrantFlow {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.client_id.clone())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        Some(ClientSecret::new(self.client_secret.clone()))
    }
}

impl From<&str> for Scope {
    fn from(value: &str) -> Self {
        Scope(oauth2::Scope::new(value.to_owned()))
    }
}

impl From<String> for Scope {
    fn from(value: String) -> Self {
        Scope(oauth2::Scope::new(value))
    }
}
