use oauth2::basic::BasicErrorResponseType;
use serde::Deserialize;
use snafu::prelude::*;

/// A convenience result type that uses [`enum@Error`] by default.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    /// The client has not yet been authenticated.
    NotAuthenticated,

    /// The access token has expired and auto-refresh is turned off.
    ExpiredToken,

    /// The (CSRF) state parameter supplied is not the same as the one initially generated and sent to the server.
    ///
    /// Learn more about CSRF [here](https://datatracker.ietf.org/doc/html/rfc6749#section-10.12).
    #[snafu(display(
        "The supplied (CSRF) state parameter is not the same as the one sent to the authorisation server. Learn more about CSRF here: https://datatracker.ietf.org/doc/html/rfc6749#section-10.12"
    ))]
    InvalidStateParameter,

    /// The access token has expired and refreshing it is not possible in the current authorisation flow.
    RefreshUnavailable,

    // There are no remaining pages left, either before or after the current one.
    NoRemainingPages,

    /// An error that indicates that an internal error occurred and the
    /// client had no PKCE verifier when authenticating.
    ///
    /// This error *should not* occur realistically.
    #[snafu(display(
        "Internal error: the client's PKCE verifier was missing when authenticating."
    ))]
    InvalidClientState,

    /// The returned data is not valid valid UTF-8.
    InvalidResponse,

    /// An error returned by Spotify.
    #[snafu(display("Error returned by the Spotify API: {status} {description}"))]
    Spotify {
        /// The HTTP status code of the error.
        status: u16,
        /// A description of the error, as returned by Spotify.
        description: String,
    },

    #[snafu(display("An error ocurred during the authentication process."))]
    Authentication {
        source: OauthError,
    },

    /// An error related to parsing items.
    #[snafu(display("{description}"))]
    Parse {
        description: String,
    },

    /// An error that resulted from deserializing data, either as a [model](crate::model) type,
    /// or as a `Nil`.
    #[snafu(display("An error occurred during deserialization."))]
    Deserialization {
        source: serde_json::Error,
        body: String,
    },

    /// An HTTP error, as returned from the underlying HTTP client.
    #[snafu(display("An HTTP error occurred."))]
    Http {
        source: reqwest::Error,
    },
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

// Error encountered when requesting an OAuth2 access token.
type OauthError = oauth2::RequestTokenError<
    oauth2::reqwest::Error<reqwest::Error>,
    oauth2::StandardErrorResponse<BasicErrorResponseType>,
>;

// Enables the use of the `?` operator.
impl From<OauthError> for Error {
    fn from(source: OauthError) -> Self {
        Self::Authentication { source }
    }
}

// Enables the use of the `?` operator.
impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Self::Http { source }
    }
}

// Enables the use of the `?` operator.
impl From<SpotifyError> for Error {
    fn from(value: SpotifyError) -> Self {
        Self::Spotify {
            status: value.error.status,
            description: value.error.message,
        }
    }
}
