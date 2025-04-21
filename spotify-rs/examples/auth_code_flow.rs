//! This shows a simple way to use OAuth2 with the auth code flow. The code also
//! works for the auth code with PKCE flow, expect for a line change (indicated by a comment).
//! This means user details can be accessed.

use std::sync::{Arc, Mutex};

use reqwest::Url;
use rouille::{Response, Server};
use spotify_rs::{model::user::TimeRange, AuthCodeClient, RedirectUrl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file.
    dotenvy::from_path("spotify-rs/examples/.env")?;

    // Get the client ID, client secret and redirect URI from environment variables.
    let client_id = dotenvy::var("CLIENT_ID")?;
    let client_secret = dotenvy::var("CLIENT_SECRET")?;
    let redirect_uri = dotenvy::var("REDIRECT_URI")?;

    // This is required for the rouille server.
    let redirect_url = Url::parse(&redirect_uri).unwrap();
    let redirect_url_host = format!(
        "{}:{}",
        redirect_url.host_str().unwrap(),
        redirect_url.port().unwrap()
    );

    // Set the scopes of the app.
    let scopes = vec!["user-top-read", "user-follow-read"];

    // Create a channel to send data from the server to the rest of the program.
    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    tokio::spawn(async move {
        // Whether or not the data has been sent yet.
        let sent = Arc::new(Mutex::new(false));
        let sent2 = sent.clone();

        // Create a rouille server.
        let server = Server::new(redirect_url_host, move |request| {
            // Get the URL the user is redirected to from Spotify's OAuth page.
            let url = Url::parse(&format!("http://{}", request.raw_url())).unwrap();

            // Get the URL queries (e.g. https:localhost:3000?code=something&csrf_state=abcd))
            let mut queries: Vec<_> = url.query_pairs().into_owned().collect();
            let auth_code = queries.remove(0).1;
            let csrf_state = queries.remove(0).1;

            // Send the code and CSRF state to use in authenticating the client.
            tx.send((auth_code, csrf_state)).unwrap();

            // Mark the fact that the code and CSRF state have been sent.
            *sent2.lock().unwrap() = true;

            // Return this at localhost:3000;
            Response::html("<h1>You may close this page</h1><script>window.close()</script>")
        })
        .unwrap();

        // While the data hasn't been sent, keep running the server.
        while !*sent.lock().unwrap() {
            server.poll();
        }
    });

    // Whether or not to automatically refresh the token once it expires.
    let auto_refresh = false;

    let (client, url) = AuthCodeClient::new(
        client_id,
        client_secret,
        scopes,
        RedirectUrl::new(redirect_uri)?,
        auto_refresh,
    );

    // Alternatively, for the Auth Code PKCE flow (the rest of the code doesn't change):
    // let (client, url) = AuthCodePkceClient::new(
    //     client_id,
    //     scopes,
    //     RedirectUrl::new(redirect_uri)?,
    //     auto_refresh,
    // );

    println!("Navigate to {url} to complete the OAuth process.");

    let (auth_code, csrf_state) = rx.recv().unwrap();

    let spotify = client.authenticate(auth_code, csrf_state).await?;

    // Get the user's top tracks in the short term.
    let top_tracks = spotify_rs::current_user_top_tracks()
        .limit(5)
        .time_range(TimeRange::ShortTerm)
        .get(&spotify)
        .await?;

    // Get a Vec<Option<Track>>.
    let tracks = top_tracks.items;

    for (i, track) in tracks.iter().enumerate() {
        if let Some(t) = track {
            println!("Top track #{}: {}", i + 1, t.name)
        }
    }

    let followed_artists = spotify_rs::followed_artists()
        .limit(10)
        .get(&spotify)
        .await?;

    let artists = followed_artists
        .filtered_items()
        .into_iter()
        .map(|a| a.name)
        .collect::<Vec<_>>()
        .join(", ");

    println!("Followed artists: {artists}");

    Ok(())
}
