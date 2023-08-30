use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};

use serde::Deserialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by the client in a custom [`Result`].
#[derive(Clone, Debug, Error)]
pub enum Error {
    /// Error that occured during authentication.
    #[error("An error occured during authentication: {description}")]
    Authentication { kind: Kind, description: String },

    /// The token has expired and auto-refresh is turned off.
    #[error("The access token has expired and auto-refresh is turned off.")]
    ExpiredToken,

    /// HTTP error returned from the underlying HTTP client.
    #[error("{0}")]
    Http(String),

    /// The (CSRF) state parameter supplied is not the same as the one initially generated and sent to the server.
    ///
    /// Learn more about CSRF [here](https://datatracker.ietf.org/doc/html/rfc6749#section-10.12).
    #[error(
        "The supplied (CSRF) state parameter is not the same as the one sent to the authorisation server. Learn more about CSRF here: https://datatracker.ietf.org/doc/html/rfc6749#section-10.12"
    )]
    InvalidStateParameter,

    /// The client has not yet been authenticated.
    #[error("The client has not been authenticated.")]
    NotAuthenticated,

    /// The access token has expired and refreshing it is not possible in the current authorisation flow.
    #[error("The access token has has expired and refreshing it is not available in the current authorisation flow.")]
    RefreshUnavailable,

    /// An error returned from Spotify.
    #[error("Error returned from the Spotify API: {status} {message}")]
    Spotify { status: u16, message: String },
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
pub enum Kind {
    /// Error response returned by the authorisation server.
    ServerResponse,
    /// An error occurred while sending the request or receiving the response..
    Request,
    /// Error parsing the server response.
    Parse,
    /// Other types of errors (e.g. unexpected server response).
    Unknown,
}

impl
    From<
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    > for Error
{
    fn from(
        value: RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ) -> Self {
        match value {
            RequestTokenError::ServerResponse(res) => {
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
                    kind: Kind::ServerResponse,
                    description,
                }
            }
            RequestTokenError::Request(err) => Error::Authentication { kind: Kind::Request, description: format!("An error occured while sending the request or receiving the response from the authentication server: {err}") },
            RequestTokenError::Parse(err, _) => Error::Authentication { kind: Kind::Parse, description: format!("Failed to parse server response: {err}") },
            RequestTokenError::Other(err) => Error::Authentication { kind: Kind::Unknown, description: format!("An unknown error occured: {err}") },
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value.to_string())
    }
}

impl From<SpotifyError> for Error {
    fn from(value: SpotifyError) -> Self {
        Self::Spotify {
            status: value.error.status,
            message: value.error.message,
        }
    }
}
