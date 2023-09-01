//! spotify-rs is a Rust wrapper for the Spotify API. It has full API coverage
//! and supports all the authorisation flows (except for the implicit grant flow).
//!
//! # Getting Started
//! First, you'll need to
//! [create an app](https://developer.spotify.com/documentation/web-api/tutorials/getting-started#create-an-app)
//! on Spotify's developer [dashboard](https://developer.spotify.com/dashboard).
//!
//! You will need to set a redirect URI, which you'll need to pass to spotify-rs
//! and is recommended you use. You'll also need the client ID and possibly the
//! client secret of your app, depending on the authorisation flow you're going to use.
//!
//! To use the Spotify API, you'll need to authenticate using your client credentials,
//! and if you want to access user resources, you'll need to go through one of the authorisation flows.
//!
//! # Authorisation
//! You will need to set some scopes, redirect the user to a generated URL, which will
//! redirect them again to your app's *redirect URI*, which will contain a code that allows
//! your app to be authorised.
//!
//! spotify-rs supports 3 of the 4 OAuth2 authorisation flows the API supports:
//! the authorisation code flow, authorisation code with PKCE flow and the client credentials flow.
//!
//! The [implicit grant flow](https://developer.spotify.com/documentation/web-api/tutorials/implicit-flow)
//! is not supported for 2 reasons:
//! - it returns the access token in the URL, which is insecure and leaves your app vulnerable to all kinds of attacks;
//! - doesn't support refreshing the access token.
//!
//! The auth flow you should use depends on the use case:
//! - the authorisation code flow is recommended for long-running applications
//! where you can safely store the client secret (e.g. web and mobile apps)
//! - the authorisation code with PKCE flow is recommended for long-running applications
//! where you *can't* safely store the client secret (e.g. desktop apps and single page web apps)
//! - the client credentials flow doesn't include authorisation, thus letting you only access public information
//!
//! Below is an example for each auth flow:
//! ## Authorisation Code Flow
//! ```no_run
//! use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};
//! # use std::error::Error;
//!
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!      // This should match the redirect URI you set in your app's settings
//!     let redirect_url = RedirectUrl::new("redirect_url".to_owned())?;
//!     let auto_refresh = true;
//!     let scopes = vec!["user-library-read", "playlist-read-private"];
//!     let auth_code_flow = AuthCodeFlow::new("client_id", "client_secret", scopes);
//!
//!     // Redirect the user to this URL to get the auth code and CSRF token
//!     let (client, url) = AuthCodeClient::new(auth_code_flow, redirect_url, auto_refresh);
//!
//!     // They will then have to be redirected to the `redirect_url` you specified,
//!     // with those two parameters present in the URL
//!
//!     // Finally, exchange the auth code for an access token
//!     let mut spotify = client.authenticate("auth_code", "csrf_token").await?;
//!
//!     // Get an album with the specified ID (requires no scopes to be set)
//!     let album = spotify.album("album_id").get().await?;
//!
//!     // The `album` method returns a builder with optional parameters you can set
//!     // For example, this sets the market to "GB".
//!     let album_gb = spotify.album("album_id").market("GB").get().await?;
//!
//!     // Get 5 of the current user's playlists (requires the playlist-read-private scope)
//!     let user_playlists = spotify.current_user_playlists().limit(5).get().await?;
//!
//!     Ok(())
//! }
//! ```
//! The Authorisation Code Flow with PKCE is the same, except you would need to use
//! [`AuthCodePkceFlow`] and [`AuthCodeClient`].
//!
//! The list of available scopes can be found [here](https://developer.spotify.com/documentation/web-api/concepts/scopes).
//!
//! You can see all of the available optional parameters in the [`Builder`] documentation.
//! They show up after each `impl Builder<'_, F, SomeEndpoint`, where `SomeEndpoint`
//! represents the endpoint you're calling.
//!
//! The auth code and CSRF token can be obtained by parsing the URL the user was redirected
//! to from the `url` returned from `.get_authorisation`.
//!
//! That could be achieved by simply having the user copy and paste the URL into your app,
//! or, the recommended approach, by having a server listening at your `redirect_url` and
//! sending the auth code and CSRF token to the main app when the user is redirected to said URL.
//!
//! ## Client Credentials Flow
//! ```no_run
//! use spotify_rs::{ClientCredsClient, ClientCredsFlow, RedirectUrl};
//! # use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let auth_flow = ClientCredsFlow::new("client_id", "client_secret");
//!
//!     // Create an authenticate the client
//!     let mut spotify = ClientCredsClient::authenticate(auth_flow).await?;
//!
//!     let album = spotify.album("album_id").get().await?;
//!
//!     Ok(())
//! }
//! ```
//! This flow doesn't require anything besides the client credentials,
//! but you cannot access any user information.
//!
//! You can see all of the available optional parameters in the [`Builder`] documentation.
//! They show up after each `impl Builder<'_, F, SomeEndpoint`, where `SomeEndpoint`
//! represents the endpoint you're calling.
//!
//! # Automatic Token Refreshing
//! If `auto_refresh` is set to `true` when creating the client, on every request
//! the client will check if the token is about to expire. If the token is close
//! to expiring, it will refresh the token for you.
//!
//! If you disable this feature, you'll have to refresh the token yourself using [`Client::request_refresh_token()`].
//!
//! [`AuthCodePkceFlow`]: auth::AuthCodePkceFlow
//! [`Builder`]: endpoint::Builder
//! [`Client::request_refresh_token()`]: client::Client::request_refresh_token()

pub mod auth;
pub mod client;
pub mod endpoint;
mod error;
pub mod model;

use client::Body;
use serde::{Deserialize, Deserializer};

pub(crate) fn query_list<T: AsRef<str>>(list: &[T]) -> String {
    list.iter()
        .map(|i| i.as_ref())
        .collect::<Vec<&str>>()
        .join(",")
}

pub(crate) fn body_list<T: AsRef<str>>(name: &str, list: &[T]) -> Body<serde_json::Value> {
    let list: Vec<_> = list.iter().map(|i| i.as_ref()).collect();
    Body::Json(serde_json::json!({ name: list }))
}

pub use auth::{AuthCodeFlow, AuthCodePkceFlow, ClientCredsFlow};
pub use client::{AuthCodeClient, AuthCodePkceClient, ClientCredsClient};
pub use error::{Error, Result as SpotifyResult};
pub use oauth2::RedirectUrl;

/// Represents an empty API response.
pub struct Nil;

impl<'de> Deserialize<'de> for Nil {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Nil)
    }
}
