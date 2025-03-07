use std::time::Duration;

use chrono::{DateTime, Utc};
use oauth2::{
    basic::BasicTokenType, AccessToken, ClientId, ClientSecret, CsrfToken, PkceCodeVerifier,
    RefreshToken, Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};

pub trait AuthenticationState: private::Sealed {}
impl AuthenticationState for Token {}
impl AuthenticationState for UnAuthenticated {}

pub trait AuthFlow: private::Sealed {
    fn client_id(&self) -> ClientId;
    fn client_secret(&self) -> Option<ClientSecret>;
    fn scopes(self) -> Option<Vec<oauth2::Scope>>;
}

pub trait Refreshable: private::Sealed {}
impl Refreshable for AuthCodeFlow {}
impl Refreshable for AuthCodePkceFlow {}

pub trait Authorised: private::Sealed {}
impl Authorised for AuthCodeFlow {}
impl Authorised for AuthCodePkceFlow {}

pub trait Verifier: private::Sealed {}
impl Verifier for NoVerifier {}
impl Verifier for CsrfVerifier {}
impl Verifier for PkceVerifier {}

mod private {
    use super::{
        AuthCodeFlow, AuthCodePkceFlow, ClientCredsFlow, CsrfVerifier, NoVerifier, PkceVerifier,
        Token, UnAuthenticated,
    };

    pub trait Sealed {}

    impl Sealed for Token {}
    impl Sealed for UnAuthenticated {}
    impl Sealed for AuthCodeFlow {}
    impl Sealed for AuthCodePkceFlow {}
    impl Sealed for ClientCredsFlow {}
    impl Sealed for NoVerifier {}
    impl Sealed for CsrfVerifier {}
    impl Sealed for PkceVerifier {}
}

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

#[derive(Clone, Debug)]
pub struct AuthCodeFlow {
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Debug)]
pub struct AuthCodePkceFlow {
    pub client_id: String,
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Debug)]
pub struct ClientCredsFlow {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct NoVerifier;

#[derive(Debug, Clone)]
pub struct CsrfVerifier(pub(crate) CsrfToken);

#[derive(Debug)]
pub struct PkceVerifier {
    pub(crate) csrf_token: CsrfToken,
    pub(crate) pkce_verifier: PkceCodeVerifier,
}

impl AuthFlow for AuthCodeFlow {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.client_id.clone())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        Some(ClientSecret::new(self.client_secret.clone()))
    }

    fn scopes(self) -> Option<Vec<oauth2::Scope>> {
        Some(self.scopes)
    }
}

impl AuthFlow for AuthCodePkceFlow {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.client_id.clone())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        None
    }

    fn scopes(self) -> Option<Vec<oauth2::Scope>> {
        Some(self.scopes)
    }
}

impl AuthFlow for ClientCredsFlow {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.client_id.clone())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        Some(ClientSecret::new(self.client_secret.clone()))
    }

    fn scopes(self) -> Option<Vec<oauth2::Scope>> {
        None
    }
}

impl Token {
    pub(crate) fn set_timestamps(self) -> Self {
        let created_at = Utc::now();

        // `self.expires_in` is a u64, so if converting from a u64 fails, use the max i64 value (unlikely to happen)
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
        Utc::now() >= self.expires_at
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

impl AuthCodeFlow {
    pub fn new<I>(client_id: impl Into<String>, client_secret: impl Into<String>, scopes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            scopes: scopes.into_iter().map(|s| Scope::new(s.into())).collect(),
        }
    }
}

impl AuthCodePkceFlow {
    pub fn new<I>(client_id: impl Into<String>, scopes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        Self {
            client_id: client_id.into(),
            scopes: scopes.into_iter().map(|s| Scope::new(s.into())).collect(),
        }
    }
}

impl ClientCredsFlow {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}
