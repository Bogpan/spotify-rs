// #![warn(missing_docs)]
//! spotify-rs is a Rust wrapper for the Spotify API. It has full API coverage
//! and supports all the authorisation flows (except for the implicit grant flow).
//!
//! # Getting Started
//! First, you'll need to
//! [create an app](https://developer.spotify.com/documentation/web-api/tutorials/getting-started#create-an-app)
//! on Spotify's [developer dashboard](https://developer.spotify.com/dashboard).
//!
//! There you will need to set a redirect URL.
//! You'll need to get the client ID, and possibly the client secret and redirect URL,
//! depending on the authorisation flow you're going to use.
//!
//! There are two concepts: authenticating - that is, "logging the app in", using your
//! client ID and secret - and authorisation - which means having a user grant your app
//! access to their account.
//!
//! Depending on your chosen auth flow, there is either one step or two required to get you
//! up and running.
//!
//! # Authorisation
//! You will need to set your scopes, redirect the user to a URL returned by spotify-rs, which will
//! redirect them *again* to your app's *redirect URL*, which will contain a code that allows
//! your app to be authorised.
//!
//! spotify-rs supports 3 of the 4 OAuth2 authorisation flows the API makes available:
//! the authorisation code flow, authorisation code with PKCE flow and the client credentials flow.
//!
//! The [implicit grant flow](https://developer.spotify.com/documentation/web-api/tutorials/implicit-flow)
//! is not supported for 2 reasons:
//! - it returns the access token in the URL, which is insecure and leaves your app vulnerable to all kinds of attacks;
//! - it doesn't support refreshing the access token.
//!
//! The auth flow you should use depends on the use case:
//! - the authorisation code flow is recommended for long-running applications
//!     where you can safely store the client secret (e.g. web and mobile apps)
//! - the authorisation code with PKCE flow is recommended for long-running applications
//!     where you *can't* safely store the client secret (e.g. desktop apps and single page web apps)
//! - the client credentials flow doesn't include authorisation, thus letting you only access public information
//!
//! Below is an example for each auth flow:
//! ## Authorisation Code Flow
//! ```no_run
//! use spotify_rs::{AuthCodeClient, RedirectUrl};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Your application scopes
//!     let scopes = vec![
//!         "user-read-private",
//!         "playlist-modify-public",
//!         "playlist-modify-private",
//!         "playlist-read-private",
//!         "playlist-read-collaborative",
//!     ];
//!
//!     // This should match the redirect URL you set in your app's settings
//!     // (on the Spotify API dashboard)
//!     let redirect_uri = RedirectUrl::new("your_redirect_url".to_owned())?;;
//!     
//!     // Whether or not to automatically refresh the token when it expires.
//!     let auto_refresh = true;
//!
//!     // You will need to redirect the user to this URL.
//!     let (client, url) = AuthCodeClient::new(
//!         "client_id",
//!         "client_secret",
//!         scopes,
//!         redirect_uri,
//!         auto_refresh,
//!     );
//!     
//!     // After the user was redirected to `url`, they will be redirected *again*, to
//!     // your `redirect_uri`, with the "auth_code" and "csrf_state" parameters in the URL.
//!     // You will need to get those parameters from the URL.
//!
//!     // Finally, you will be able to authenticate the client.
//!     let spotify = client.authenticate("auth_code", "csrf_state").await?;
//!
//!     // Get an album with the specified ID.
//!     let album = spotify_rs::album("album_id").get(&spotify).await?;
//!     println!("The name of the album is: {}", album.name);
//!     
//!     // The `album` method returns a builder with optional parameters you can set
//!     // For example, this sets the market to "GB".
//!     let album_gb = spotify_rs::album("album_id")
//!         .market("GB")
//!         .get(&spotify)
//!         .await?;
//!     println!("The popularity of the album is {}", album_gb.popularity);
//!     
//!     // This gets 5 playlists of the user that authorised the app
//!     // (it requires the playlist-read-private scope).
//!     let user_playlists = spotify_rs::current_user_playlists()
//!         .limit(5)
//!         .get(&spotify)
//!         .await?;
//!     let result_count = user_playlists.items.len();
//!     println!("The API returned {} playlists.", result_count);
//!     
//!     Ok(())
//! }
//! ```
//! The Authorisation Code Flow with PKCE is the same, except you would need to use
//! [`AuthCodePkceClient`] instead of [`AuthCodeClient`].
//!
//! A of available scopes can be found [here](https://developer.spotify.com/documentation/web-api/concepts/scopes).
//!
//! The auth code and CSRF token can be obtained by parsing the URL the user was
//! redirected to (the redirect URL, as set in the API dashboard and when creating the client).
//!
//! Please note that the redirect URL you pass to [`authenticate`] *must* match
//! the redirect URL you set in the Spotify API developer dashboard.
//!
//! That could be achieved by simply having the user copy and paste the URL into
//! your app, or, for example, by having a server listening at your `redirect_url`
//! and sending the auth code and CSRF token to the main app when the user is
//! redirected to said URL.
//!
//! For examples, check out the examples directory.
//!
//! ## Client Credentials Flow
//! ```no_run
//! use spotify_rs::{ClientCredsClient, RedirectUrl};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let spotify = ClientCredsClient::authenticate("client_id", "client_secret").await?;
//!
//!     // Get an album with the specified ID.
//!     let album = spotify_rs::album("album_id").get(&spotify).await?;
//!     println!("The name of the album is: {}", album.name);
//!     
//!     // The `album` method returns a builder with optional parameters you can set
//!     // For example, this sets the market to "GB".
//!     let album_gb = spotify_rs::album("album_id")
//!         .market("GB")
//!         .get(&spotify)
//!         .await?;
//!     println!("The popularity of the album is {}", album_gb.popularity);
//!     
//!     Ok(())
//! }
//! ```
//! This flow doesn't require anything besides the client credentials,
//! but you cannot access any user information.
//!
//! # Automatic Token Refreshing
//! If `auto_refresh` is set to `true` when creating the client, on every request
//! the client will check if the token is about to expire. If the token is close
//! to expiring, it will refresh the token for you.
//!
//! *Note: this means that if the token has expired, the `RwLock` holding the [`Token`]*
//! *will be acquired in order to change the token.*
//!
//! If you disable this feature, you'll have to refresh the token yourself using [`request_refresh_token`].
//!
//! [`Token`]: auth::Token
//! [`AuthCodeFlow`]: auth::AuthCodeFlow
//! [`AuthCodePkceFlow`]: auth::AuthCodePkceFlow
//! [`request_refresh_token`]: client::Client::request_refresh_token()
//! [`authenticate`]: client::Client::authenticate()

mod auth;
/// Struct and methods for constructing and authenticating [`Clients`](crate::client::Client).
pub mod client;
/// Functions and builders for all the Spotify endpoints.
pub mod endpoint;
mod error;
/// Mappings of objects received from the Spotify API.
pub mod model;

use client::Body;
use serde::{Deserialize, Deserializer};

pub use auth::{AuthCodePkceFlow, ClientCredsFlow, Token, Unauthenticated};
pub use client::{AuthCodeClient, AuthCodePkceClient, ClientCredsClient};
pub use error::{Error, Result as SpotifyResult};
pub use oauth2::RedirectUrl;

#[doc(hidden)]
pub use endpoint::{
    album::{
        album, album_tracks, albums, check_saved_albums, new_releases, remove_saved_albums,
        save_albums, saved_albums,
    },
    artist::{artist, artist_albums, artist_top_tracks, artists, get_related_artists},
    audiobook::{
        audiobook, audiobook_chapters, audiobooks, chapter, chapters, check_saved_audiobooks,
        remove_saved_audiobooks, save_audiobooks, saved_audiobooks,
    },
    category::{browse_categories, browse_category},
    genres::get_genre_seeds,
    markets::get_available_markets,
    player::{
        add_item_to_queue, get_available_devices, get_currently_playing_track, get_playback_state,
        get_user_queue, pause_playback, recently_played_tracks, seek_to_position,
        set_playback_volume, set_repeat_mode, skip_to_next, skip_to_previous, start_playback,
        toggle_playback_shuffle, transfer_playback,
    },
    playlist::{
        add_items_to_playlist, add_playlist_image, category_playlists, change_playlist_details,
        create_playlist, current_user_playlists, featured_playlists, get_playlist_image, playlist,
        playlist_items, remove_playlist_items, update_playlist_items, user_playlists,
    },
    search::search,
    show::{
        check_saved_episodes, check_saved_shows, episode, episodes, remove_saved_episodes,
        remove_saved_shows, save_episodes, save_shows, saved_episodes, saved_shows, show,
        show_episodes, shows,
    },
    track::{
        check_saved_tracks, get_track_audio_analysis, get_track_audio_features,
        get_tracks_audio_features, recommendations, remove_saved_tracks, save_tracks, saved_tracks,
        track, tracks,
    },
    user::{
        check_if_current_user_follow_playlist, check_if_user_follows_artists,
        check_if_user_follows_users, current_user_top_artists, current_user_top_tracks,
        follow_artists, follow_playlist, follow_users, followed_artists, get_current_user_profile,
        get_user, unfollow_artists, unfollow_playlist, unfollow_users,
    },
};

// Function meant to create a URL query list from &[T].
pub(crate) fn query_list<T: AsRef<str>>(list: &[T]) -> String {
    list.iter()
        .map(|i| i.as_ref())
        .collect::<Vec<&str>>()
        .join(",")
}

// Function meant to create a request body list from &[T].
pub(crate) fn body_list<T: AsRef<str>>(name: &str, list: &[T]) -> Body<serde_json::Value> {
    let list: Vec<&str> = list.iter().map(|i| i.as_ref()).collect();
    Body::Json(serde_json::json!({ name: list }))
}

/// Represents an empty API response.
pub struct Nil;

// Used to deserialize an empty API response.
// This is also necessary for when Spotify API endpoints that should
// normally return empty responses, instead return (useless) non-JSON responses.
impl<'de> Deserialize<'de> for Nil {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Nil)
    }
}
