[![crates.io](https://img.shields.io/crates/v/spotify-rs)](https://crates.io/crates/spotify-rs)
[![docs.rs](https://img.shields.io/docsrs/spotify-rs)](https://docs.rs/spotify-rs)

# spotify-rs
spotify-rs is a Rust wrapper for the Spotify API.

It has full API coverage and supports all the authorisation flows (except for the implicit grant flow).

Usage example:
```rs
use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};
# use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This should match the redirect URI you set in your app's settings
    let redirect_url = RedirectUrl::new("redirect_url".to_owned())?;
    let auto_refresh = true;
    let scopes = vec!["user-library-read", "playlist-read-private"];
    let auth_code_flow = AuthCodeFlow::new("client_id", "client_secret", scopes);

    // Redirect the user to this URL to get the auth code and CSRF token
    let (client, url) = AuthCodeClient::new(auth_code_flow, redirect_url, auto_refresh);

    // They will then have to be redirected to the `redirect_url` you specified,
    // with those two parameters present in the URL

    // Finally, exchange the auth code for an access token
    let mut spotify = client.authenticate("auth_code", "csrf_token").await?;

    // Get an album with the specified ID (requires no scopes to be set)
    let album = spotify.album("album_id").get().await?;

    // The `album` method returns a builder with optional parameters you can set
    // For example, this sets the market to "GB".
    let album_gb = spotify.album("album_id").market("GB").get().await?;

    // Get 5 of the current user's playlists (requires the playlist-read-private scope)
    let user_playlists = spotify.current_user_playlists().limit(5).get().await?;

    Ok(())
}
```

## License
spotify-rs is dual-licensed under [Apache 2.0](https://github.com/Bogpan/spotify-rs/blob/main/LICENSE-APACHE) and [MIT](https://github.com/Bogpan/spotify-rs/blob/main/LICENSE-MIT) terms.
