//! A simple application that uses the Client Credentials flow to authenticate
//! and prints the name of an album and its artists.

use spotify_rs::ClientCredsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file.
    dotenvy::from_path("spotify-rs/examples/.env")?;

    // Get the client ID and client secret from environment variables.
    let client_id = dotenvy::var("CLIENT_ID")?;
    let client_secret = dotenvy::var("CLIENT_SECRET")?;

    // Create the Spotify client.
    let spotify = ClientCredsClient::authenticate(client_id, client_secret).await?;

    // Get the album with that ID, in the "GB" (Great Britain) market.
    let album = spotify_rs::album("78bpIziExqiI9qztvNFlQu")
        .market("GB")
        .get(&spotify)
        .await?;

    // Turn the list of artists into a list of their names.
    let artist_names: Vec<String> = album.artists.into_iter().map(|a| a.name).collect();

    println!(
        "The name of the album is {}, by {}",
        album.name,
        artist_names.join(", ")
    );

    Ok(())
}
