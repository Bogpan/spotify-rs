use std::{collections::HashSet, fmt::Debug, time::Duration};

use chrono::{DateTime, Utc};
use oauth2::{
    basic::BasicTokenType, AccessToken, CsrfToken, PkceCodeVerifier, RefreshToken, TokenResponse,
};
use serde::{Deserialize, Serialize};

// Typestate trait definitions and implementations.
pub trait AuthenticationState: private::Sealed {}
impl AuthenticationState for Token {}
impl AuthenticationState for Unauthenticated {}

pub trait AuthFlow: private::Sealed + Debug {}
impl AuthFlow for AuthCodeFlow {}
impl AuthFlow for AuthCodePkceFlow {}
impl AuthFlow for ClientCredsFlow {}
impl AuthFlow for UnknownFlow {}

impl Debug for AuthCodeFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthCodeFlow")
            .field("csrf_token", &"[redacted]")
            .finish()
    }
}

impl Debug for AuthCodePkceFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthCodePkceFlow")
            .field("csrf_token", &"[redacted]")
            .field("pkce_verifier", &"[redacted]")
            .finish()
    }
}

pub trait Authorised: private::Sealed {}
impl Authorised for AuthCodeFlow {}
impl Authorised for AuthCodePkceFlow {}

// The only way to have an unknown flow is by creating the client from a
// refresh token, which is only available for authorised flows, thus this
// is authorised.
impl Authorised for UnknownFlow {}

// Make it so users of the crate can't implement the typestate traits for their
// own types (which might not work anyway).
mod private {
    pub trait Sealed {}

    impl Sealed for super::Token {}
    impl Sealed for super::Unauthenticated {}
    impl Sealed for super::AuthCodeFlow {}
    impl Sealed for super::AuthCodePkceFlow {}
    impl Sealed for super::ClientCredsFlow {}
    impl Sealed for super::UnknownFlow {}
}

/// A list of (unique) scopes. You don't usually have to interact
/// with it directly, the conversion should happen implicitly, with the exception of
/// the [`from_refresh_token`](Client::from_refresh_token) function.
///
/// In such cases, you should just call [into](Into::into) on your list of scopes.
#[derive(Clone, Debug, Default)]
pub struct Scopes(pub(crate) HashSet<oauth2::Scope>);

impl<I> From<I> for Scopes
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(value: I) -> Self {
        let scopes = value
            .into_iter()
            .map(|i| oauth2::Scope::new(i.into()))
            .collect();
        Self(scopes)
    }
}

impl Scopes {
    /// Create a list of scopes for an iterator of items implement Into<String>.
    ///
    /// This is just using the existing From implementation, but exists to make
    /// it clearer how to create the struct for users.
    pub fn new<I>(scopes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        Self::from(scopes)
    }

    fn inner_vec(self) -> Vec<oauth2::Scope> {
        self.0.into_iter().collect()
    }
}

/// An OAuth2 token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// The token used for authenticating every single request.
    pub(crate) access_token: AccessToken,
    /// The token used for requesting a new access token when the current one expires.
    pub(crate) refresh_token: Option<RefreshToken>,
    /// How long until the current token expires, in seconds.
    pub expires_in: u64,

    #[serde(default = "Utc::now")]
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

// Represents the state of a client that's not authenticated.
#[doc = include_str!("docs/internal_implementation_details.md")]
#[derive(Clone, Copy, Debug)]
pub struct Unauthenticated;

/// Represents the [Authorisation Code Flow](https://developer.spotify.com/documentation/web-api/tutorials/code-flow),
/// as defined in [OAuth2 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1).
///
/// Its use is recommended in cases where the `client_secret` can be safely stored,
/// such as a web app running on the server.
///
/// This flow requires user authorisation, and thus allows the app
/// to make requests on behalf of the user.
pub struct AuthCodeFlow {
    pub(crate) csrf_token: CsrfToken,
}

/// Represents the [Authorisation Code Flow with PKCE extension](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow),
/// as defined in [OAuth2 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1) and [OAuth2 RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636).
///
/// Its use is recommended in cases where storing the `client_secret` safely is
/// *not* possible, such as web apps running on the client, desktop and mobile apps.
///
/// This flow requires user authorisation, and thus allows the app
/// to make requests on behalf of the user.
pub struct AuthCodePkceFlow {
    pub(crate) csrf_token: CsrfToken,
    pub(crate) pkce_verifier: Option<PkceCodeVerifier>,
}

/// Represents the [Client Credentials Flow](https://developer.spotify.com/documentation/web-api/tutorials/client-credentials-flow),
/// as defined in [OAuth2 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749#section-4.4).
///
/// Its use is recommended for apps usually running in the backend, that don't
/// require accessing user information.
///
/// This flow does *not* require user authorisation, and thus does not permit
/// making requests on the behalf of the user, so it can't access user data.
#[derive(Clone, Copy, Debug)]
pub struct ClientCredsFlow;

/// Represents an unknown authentication flow, used when creating a
/// [`Client`](crate::client::Client) via methods like
/// [`from_refresh_token`](crate::client::Client::from_refresh_token).
#[derive(Clone, Copy, Debug)]
pub struct UnknownFlow;

impl Token {
    /// Create a new token, to be used with one of the [`from_access_token`](crate::client::Client::from_access_token) methods.
    pub fn new(
        access_token: impl Into<String>,
        refresh_token: Option<&str>,
        created_at: DateTime<Utc>,
        expires_in: u64,
        scopes: Option<Scopes>,
    ) -> Self {
        let access_token = AccessToken::new(access_token.into());
        let refresh_token = refresh_token.map(|t| RefreshToken::new(t.to_owned()));
        let expires_at =
            created_at + chrono::Duration::seconds(i64::try_from(expires_in).unwrap_or(i64::MAX));

        let scopes = scopes.map(|s| s.inner_vec());

        Self {
            access_token,
            refresh_token,
            expires_in,
            created_at,
            expires_at,
            token_type: BasicTokenType::Bearer,
            scopes,
        }
    }

    /// Get the current access token secret.
    pub fn secret(&self) -> &str {
        self.access_token.secret()
    }

    /// Get the current refresh token. Some auth flows may not provide a refresh token,
    /// in which case it will return `None`.
    pub fn refresh_secret(&self) -> Option<&str> {
        self.refresh_token.as_ref().map(|t| t.secret().as_str())
    }

    // Used to set the timestamp of a newly received token to the current time.
    pub(crate) fn set_timestamps(self) -> Self {
        let created_at = Utc::now();

        // `self.expires_in` is a u64, so if converting from a u64 fails, use
        // the max i64 value (unlikely to happen).
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
