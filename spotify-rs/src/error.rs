use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

use oauth2::basic::BasicErrorResponseType;
use serde::Deserialize;
use thiserror::Error;

use crate::Token;

/// A convenience result type that uses [`enum@Error`] by default.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error returned by the [`Client`](crate::client::Client) in [`Result`].
#[derive(Clone, Debug, Error)]
pub enum Error {
    /// Error that occured during authentication.
    #[error("An error occured during authentication: {description}")]
    Authentication {
        /// The kind of the authentication error, that dictates roughly where
        /// the error originates.
        kind: AuthErrorKind,
        /// A description of the error.
        description: String,
    },

    /// The client has not yet been authenticated.
    #[error("The client has not been authenticated.")]
    NotAuthenticated,

    /// The token has expired and auto-refresh is turned off.
    #[error("The access token has expired and auto-refresh is turned off.")]
    ExpiredToken,

    /// The (CSRF) state parameter supplied is not the same as the one initially generated and sent to the server.
    ///
    /// Learn more about CSRF [here](https://datatracker.ietf.org/doc/html/rfc6749#section-10.12).
    #[error(
        "The supplied (CSRF) state parameter is not the same as the one sent to the authorisation server. Learn more about CSRF here: https://datatracker.ietf.org/doc/html/rfc6749#section-10.12"
    )]
    InvalidStateParameter,

    /// The access token has expired and refreshing it is not possible in the current authorisation flow.
    #[error("The access token has has expired and refreshing it is not possible in the current authorisation flow.")]
    RefreshUnavailable,

    /// An HTTP error, as returned from the underlying HTTP client.
    #[error("{0}")]
    Http(String),

    /// A deserialization error, as returned by serde.
    #[error("{0}")]
    Deserialization(String),

    /// An error returned by Spotify.
    #[error("Error returned by the Spotify API: {status} {description}")]
    Spotify {
        /// The HTTP status code of the error.
        status: u16,
        /// A description of the error.
        description: String,
    },

    /// An error related to parsing items.
    #[error("{description}")]
    Parse {
        /// A description of the error.
        description: String,
    },

    /// An error that occurs when the lock holding the client's token
    /// has been poisoned.
    #[error("The lock holding the Client's Token was poisoned: {0}")]
    PoisonedLock(String),

    /// An error that indicates that an internal error occurred and the
    /// client had no PKCE verifier when authenticating.
    ///
    /// This error *should not* occur realistically, but it exists just
    /// in case.
    #[error("Internal error: the client's PKCE verifier was missing when authenticating.")]
    InvalidClientState,

    // Rename the error and rewrite the description
    #[error("There are no remaining next/previous pages to get.")]
    NoRemainingPages,
}

#[derive(Deserialize)]
pub(crate) struct SpotifyError {
    error: Details,
}

#[derive(Deserialize)]
struct Details {
    status: u16,
    message: String,
}

/// The authentication error kind.
#[derive(Clone, Copy, Debug)]
pub enum AuthErrorKind {
    /// Error response returned by the authorisation server.
    ServerResponse,
    /// An error occurred while sending the request or receiving the response.
    Request,
    /// Error parsing the server response.
    Parse,
    /// Other types of errors (e.g. unexpected server response).
    Unknown,
}

// Error encountered when requesting an OAuth2 access token.
type OauthError = oauth2::RequestTokenError<
    oauth2::reqwest::Error<reqwest::Error>,
    oauth2::StandardErrorResponse<BasicErrorResponseType>,
>;

// Enable the use of the `?` operator.
impl From<OauthError> for Error {
    fn from(value: OauthError) -> Self {
        match value {
            OauthError::ServerResponse(res) => {
                let additional = match res.error_description() {
                    Some(desc) => format!(": {desc}"),
                    None => ".".to_owned(),
                };

                let  base = match res.error() {
                    BasicErrorResponseType::InvalidClient => "Client authentication failed",
                    BasicErrorResponseType::InvalidGrant =>  "The provided authorization grant or refresh token may be invalid, expired or revoked",
                    BasicErrorResponseType::InvalidRequest => "The request is invalid or malformed",
                    BasicErrorResponseType::InvalidScope => "The requested scope is invalid, unknown, malformed, or exceeds the scope granted by the resource owner",
                    BasicErrorResponseType::UnauthorizedClient => "The authenticated client is not authorized to use this authorization grant type",
                    BasicErrorResponseType::UnsupportedGrantType => "The authorization grant type is not supported by the authorization server",
                    BasicErrorResponseType::Extension(desc) => desc,
                };

                let description = format!("{base}{additional}");

                Error::Authentication {
                    kind: AuthErrorKind::ServerResponse,
                    description,
                }
            }
            OauthError::Request(err) => Error::Authentication { kind: AuthErrorKind::Request, description: format!("An error occured while sending the request or receiving the response from the authentication server: {err}") },
            OauthError::Parse(err, _) => Error::Authentication { kind: AuthErrorKind::Parse, description: format!("Failed to parse server response: {err}") },
            OauthError::Other(err) => Error::Authentication { kind: AuthErrorKind::Unknown, description: format!("An unknown error occured: {err}") },
        }
    }
}

// Enable the use of the `?` operator.
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value.to_string())
    }
}

// Enable the use of the `?` operator.
impl From<serde::de::value::Error> for Error {
    fn from(value: serde::de::value::Error) -> Self {
        Self::Deserialization(value.to_string())
    }
}

// Enable the use of the `?` operator.
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialization(value.to_string())
    }
}

// Enable the use of the `?` operator.
impl From<SpotifyError> for Error {
    fn from(value: SpotifyError) -> Self {
        Self::Spotify {
            status: value.error.status,
            description: value.error.message,
        }
    }
}

// Enable the use of the `?` operator.
impl From<PoisonError<RwLockReadGuard<'_, Token>>> for Error {
    fn from(value: PoisonError<RwLockReadGuard<'_, Token>>) -> Self {
        Self::PoisonedLock(value.to_string())
    }
}

// Enable the use of the `?` operator.
impl From<PoisonError<RwLockWriteGuard<'_, Token>>> for Error {
    fn from(value: PoisonError<RwLockWriteGuard<'_, Token>>) -> Self {
        Self::PoisonedLock(value.to_string())
    }
}
