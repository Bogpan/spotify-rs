use std::marker::PhantomData;

use base64::{engine::general_purpose, Engine};
use oauth2::{
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenType,
    },
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, StandardRevocableToken, TokenUrl,
};
use reqwest::{header::CONTENT_LENGTH, Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::{
    auth::{
        AuthCodeFlow, AuthCodePkceFlow, AuthFlow, AuthenticationState, Authorised, ClientCredsFlow,
        CsrfVerifier, NoVerifier, PkceVerifier, Token, UnAuthenticated, Verifier,
    },
    body_list,
    endpoint::{
        album::*,
        artist::ArtistEndpoint,
        audiobook::*,
        category::{BrowseCategoriesEndpoint, BrowseCategoryEndpoint},
        player::*,
        playlist::*,
        search::SearchEndpoint,
        show::*,
        track::*,
        user::*,
        Builder, Endpoint,
    },
    error::{Error, Result, SpotifyError},
    model::{
        artist::{Artist, Artists},
        audio::{AudioAnalysis, AudioFeatures, AudioFeaturesResult},
        market::Markets,
        player::{Device, Devices, PlaybackState, Queue},
        recommendation::Genres,
        search::Item,
        user::{User, UserItemType},
        Image,
    },
    query_list, Nil,
};

const AUTHORISATION_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub(crate) type OAuthClient = oauth2::Client<
    BasicErrorResponse,
    Token,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
>;

/// A client created using the Authorisation Code Flow.
pub type AuthCodeClient<V = NoVerifier> = Client<UnAuthenticated, AuthCodeFlow, V>;

/// A client created using the Authorisation Code with PKCE Flow.
pub type AuthCodePkceClient<V = NoVerifier> = Client<UnAuthenticated, AuthCodePkceFlow, V>;

/// A client created using the Client Credentials Flow.
pub type ClientCredsClient<V = NoVerifier> = Client<UnAuthenticated, ClientCredsFlow, V>;

#[doc(hidden)]
pub(crate) enum Body<P: Serialize = ()> {
    Json(P),
    File(Vec<u8>),
}

/// The client which handles the authentication and all the Spotify API requests.
///
/// It is recommended to use one of the following: [`AuthCodeClient`], [`AuthCodePkceClient`] or [`ClientCredsClient`],
/// depending on the chosen auth flow.
#[derive(Debug)]
pub struct Client<A: AuthenticationState, F: AuthFlow, V: Verifier> {
    /// Dictates whether or not the client will request a new token when the
    /// current one is about the expire.
    ///
    /// It will check if the token has expired in every request.
    pub auto_refresh: bool,
    pub(crate) auth: A,
    pub(crate) oauth: OAuthClient,
    pub(crate) http: reqwest::Client,
    pub(crate) verifier: V,
    marker: PhantomData<F>,
}

impl Client<UnAuthenticated, AuthCodeFlow, CsrfVerifier> {
    /// Create a new client and generate an authorisation URL
    ///
    /// You must redirect the user to the returned URL, which in turn redirects them to
    /// the `redirect_uri` you provided, along with a `code` and `state` parameter in the URl.
    ///
    /// They are required for the next step in the auth process.
    pub fn new(
        AuthCodeFlow {
            client_id,
            client_secret,
            scopes,
        }: AuthCodeFlow,
        redirect_uri: RedirectUrl,
        auto_refresh: bool,
    ) -> (Self, Url) {
        let oauth = OAuthClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        )
        .set_redirect_uri(redirect_uri);

        let (auth_url, csrf_token) = oauth
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .url();

        (
            Client {
                auto_refresh,
                auth: UnAuthenticated,
                oauth,
                http: reqwest::Client::new(),
                verifier: CsrfVerifier(csrf_token),
                marker: PhantomData,
            },
            auth_url,
        )
    }
}

impl Client<UnAuthenticated, AuthCodePkceFlow, PkceVerifier> {
    /// Create a new client and generate an authorisation URL
    ///
    /// You must redirect the user to the received URL, which in turn redirects them to
    /// the redirect URI you provided, along with a `code` and `state` parameter in the URl.
    ///
    /// They are required for the next step in the auth process.
    pub fn new(
        AuthCodePkceFlow { client_id, scopes }: AuthCodePkceFlow,
        redirect_uri: RedirectUrl,
        auto_refresh: bool,
    ) -> (Self, Url) {
        let oauth = OAuthClient::new(
            ClientId::new(client_id),
            None,
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        )
        .set_redirect_uri(redirect_uri);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = oauth
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .set_pkce_challenge(pkce_challenge)
            .url();

        (
            Client {
                auto_refresh,
                auth: UnAuthenticated,
                oauth,
                http: reqwest::Client::new(),
                verifier: PkceVerifier {
                    csrf_token,
                    pkce_verifier,
                },
                marker: PhantomData,
            },
            auth_url,
        )
    }
}

impl<F: AuthFlow> Client<Token, F, NoVerifier> {
    /// Create a new authenticated and authorised client from a refresh token.
    /// It's still required to specify an auth flow.
    ///
    /// This method will fail if the refresh token is invalid or a new one cannot be obtained.
    pub async fn from_refresh_token(
        auth_flow: F,
        auto_refresh: bool,
        refresh_token: String,
    ) -> Result<Client<Token, F, NoVerifier>> {
        let oauth_client = OAuthClient::new(
            auth_flow.client_id(),
            auth_flow.client_secret(),
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let refresh_token = RefreshToken::new(refresh_token);

        let mut req = oauth_client.exchange_refresh_token(&refresh_token);

        if let Some(scopes) = auth_flow.scopes() {
            req = req.add_scopes(scopes);
        }

        let token = req.request_async(async_http_client).await?.set_timestamps();

        Ok(Client {
            auto_refresh,
            auth: token,
            oauth: oauth_client,
            http: reqwest::Client::new(),
            verifier: NoVerifier,
            marker: PhantomData,
        })
    }
}

impl<F: AuthFlow, V: Verifier> Client<Token, F, V> {
    /// Get the current access token.
    pub fn access_token(&self) -> &str {
        self.auth.access_token.secret()
    }

    /// Get the current refresh token. Some auth flows may not provide a refresh token,
    /// in which case it's `None`.
    pub fn refresh_token(&self) -> Option<&str> {
        self.auth
            .refresh_token
            .as_ref()
            .map(|t| t.secret().as_str())
    }

    /// Request a new refresh token and updates it in the client.
    /// Only some auth flows allow for token refreshing.
    pub async fn request_refresh_token(&mut self) -> Result<()> {
        let Some(refresh_token) = &self.auth.refresh_token else {
            return Err(Error::RefreshUnavailable);
        };

        let token = self
            .oauth
            .exchange_refresh_token(refresh_token)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        self.auth = token;
        Ok(())
    }

    pub(crate) async fn request<P: Serialize, T: DeserializeOwned>(
        &mut self,
        method: Method,
        endpoint: String,
        query: Option<P>,
        body: Option<Body<P>>,
    ) -> Result<T> {
        if self.auth.is_expired() {
            if self.auto_refresh {
                self.request_refresh_token().await?;
            } else {
                return Err(Error::ExpiredToken);
            }
        }

        let mut req = self
            .http
            .request(method, format!("https://api.spotify.com/v1{endpoint}"))
            .bearer_auth(self.auth.access_token.secret());

        if let Some(q) = query {
            req = req.query(&q);
        }

        if let Some(b) = body {
            match b {
                Body::Json(j) => req = req.json(&j),
                Body::File(f) => req = req.body(f),
            }
        } else {
            // Used because Spotify wants a Content-Length header for the PUT /audiobooks/me endpoint even though there is no body
            // If not supplied, it will return an error in the form of HTML (not JSON), which I believe to be an issue on their end.
            // No other endpoints so far behave this way.
            req = req.header(CONTENT_LENGTH, 0);
        }

        let res = req.send().await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(res.json::<SpotifyError>().await?.into())
        }
    }

    pub(crate) async fn get<P: Serialize, T: DeserializeOwned>(
        &mut self,
        endpoint: String,
        query: impl Into<Option<P>>,
    ) -> Result<T> {
        self.request(Method::GET, endpoint, query.into(), None)
            .await
    }

    pub(crate) async fn post<P: Serialize, T: DeserializeOwned>(
        &mut self,
        endpoint: String,
        body: impl Into<Option<Body<P>>>,
    ) -> Result<T> {
        self.request(Method::POST, endpoint, None, body.into())
            .await
    }

    pub(crate) async fn put<P: Serialize, T: DeserializeOwned>(
        &mut self,
        endpoint: String,
        body: impl Into<Option<Body<P>>>,
    ) -> Result<T> {
        self.request(Method::PUT, endpoint, None, body.into()).await
    }

    pub(crate) async fn delete<P: Serialize, T: DeserializeOwned>(
        &mut self,
        endpoint: String,
        body: impl Into<Option<Body<P>>>,
    ) -> Result<T> {
        self.request(Method::DELETE, endpoint, None, body.into())
            .await
    }

    fn builder<E: Endpoint>(&mut self, endpoint: E) -> Builder<'_, F, V, E> {
        Builder {
            spotify: self,
            endpoint,
        }
    }

    pub fn album(&mut self, id: impl Into<String>) -> Builder<'_, F, V, AlbumEndpoint> {
        self.builder(AlbumEndpoint {
            id: id.into(),
            market: None,
        })
    }

    pub fn albums<T: AsRef<str>>(&mut self, ids: &[T]) -> Builder<'_, F, V, AlbumsEndpoint> {
        self.builder(AlbumsEndpoint {
            ids: query_list(ids),
            market: None,
        })
    }

    pub fn album_tracks(
        &mut self,
        album_id: impl Into<String>,
    ) -> Builder<'_, F, V, AlbumTracksEndpoint> {
        self.builder(AlbumTracksEndpoint {
            id: album_id.into(),
            ..Default::default()
        })
    }

    pub fn new_releases(&mut self) -> Builder<'_, F, V, NewReleasesEndpoint> {
        self.builder(NewReleasesEndpoint::default())
    }

    pub fn artist(&mut self, id: impl Into<String>) -> Builder<'_, F, V, ArtistEndpoint> {
        self.builder(ArtistEndpoint { id: id.into() })
    }

    pub async fn get_artists<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Vec<Artist>> {
        self.get("/artists".to_owned(), [("ids", query_list(ids))])
            .await
            .map(|a: Artists| a.artists)
    }

    pub fn audiobook(&mut self, id: impl Into<String>) -> Builder<'_, F, V, AudiobookEndpoint> {
        self.builder(AudiobookEndpoint {
            id: id.into(),
            market: None,
        })
    }

    pub fn audiobooks<T: AsRef<str>>(
        &mut self,
        ids: &[T],
    ) -> Builder<'_, F, V, AudiobooksEndpoint> {
        self.builder(AudiobooksEndpoint {
            ids: query_list(ids),
            market: None,
        })
    }

    pub fn audiobook_chapters(
        &mut self,
        audiobook_id: impl Into<String>,
    ) -> Builder<'_, F, V, AudiobookChaptersEndpoint> {
        self.builder(AudiobookChaptersEndpoint {
            id: audiobook_id.into(),
            ..Default::default()
        })
    }

    pub fn browse_category(
        &mut self,
        id: impl Into<String>,
    ) -> Builder<'_, F, V, BrowseCategoryEndpoint> {
        self.builder(BrowseCategoryEndpoint {
            id: id.into(),
            ..Default::default()
        })
    }

    pub fn browse_categories(&mut self) -> Builder<'_, F, V, BrowseCategoriesEndpoint> {
        self.builder(BrowseCategoriesEndpoint::default())
    }

    /// *Note: Spotify's API returns `500 Server error`.*
    pub fn chapter(&mut self, id: impl Into<String>) -> Builder<'_, F, V, ChapterEndpoint> {
        self.builder(ChapterEndpoint {
            id: id.into(),
            market: None,
        })
    }

    /// *Note: Spotify's API returns `500 Server error`.*
    pub fn chapters<T: AsRef<str>>(&mut self, ids: &[T]) -> Builder<'_, F, V, ChaptersEndpoint> {
        self.builder(ChaptersEndpoint {
            ids: query_list(ids),
            market: None,
        })
    }

    pub fn episode(&mut self, id: impl Into<String>) -> Builder<'_, F, V, EpisodeEndpoint> {
        self.builder(EpisodeEndpoint {
            id: id.into(),
            market: None,
        })
    }

    pub fn episodes<T: AsRef<str>>(&mut self, ids: &[T]) -> Builder<'_, F, V, EpisodesEndpoint> {
        self.builder(EpisodesEndpoint {
            ids: query_list(ids),
            market: None,
        })
    }

    pub async fn get_genre_seeds(&mut self) -> Result<Vec<String>> {
        self.get::<(), _>("/recommendations/available-genre-seeds".to_owned(), None)
            .await
            .map(|g: Genres| g.genres)
    }

    pub async fn get_available_markets(&mut self) -> Result<Vec<String>> {
        self.get::<(), _>("/markets".to_owned(), None)
            .await
            .map(|m: Markets| m.markets)
    }

    pub fn playlist(&mut self, id: impl Into<String>) -> Builder<'_, F, V, PlaylistEndpoint> {
        self.builder(PlaylistEndpoint {
            id: id.into(),
            ..Default::default()
        })
    }

    pub fn change_playlist_details(
        &mut self,
        id: impl Into<String>,
    ) -> Builder<'_, F, V, ChangePlaylistDetailsEndpoint> {
        self.builder(ChangePlaylistDetailsEndpoint {
            id: id.into(),
            ..Default::default()
        })
    }

    pub fn playlist_items(
        &mut self,
        id: impl Into<String>,
    ) -> Builder<'_, F, V, PlaylistItemsEndpoint> {
        self.builder(PlaylistItemsEndpoint {
            id: id.into(),
            ..Default::default()
        })
    }

    pub fn update_playlist_items(
        &mut self,
        id: impl Into<String>,
        range_start: u32,
        insert_before: u32,
    ) -> Builder<'_, F, V, UpdatePlaylistItemsEndpoint> {
        self.builder(UpdatePlaylistItemsEndpoint {
            id: id.into(),
            range_start,
            insert_before,
            ..Default::default()
        })
    }

    pub fn add_items_to_playlist<T: ToString>(
        &mut self,
        id: impl Into<String>,
        item_uris: &[T],
    ) -> Builder<'_, F, V, AddPlaylistItemsEndpoint> {
        self.builder(AddPlaylistItemsEndpoint {
            id: id.into(),
            uris: item_uris.iter().map(ToString::to_string).collect(),
            position: None,
        })
    }

    pub fn remove_playlist_items<T: AsRef<str>>(
        &mut self,
        id: impl Into<String>,
        item_uris: &[T],
    ) -> Builder<'_, F, V, RemovePlaylistItemsEndpoint> {
        let tracks = item_uris
            .iter()
            .map(|u| json!({ "uri": u.as_ref() }))
            .collect();

        self.builder(RemovePlaylistItemsEndpoint {
            id: id.into(),
            tracks,
            snapshot_id: None,
        })
    }

    pub fn user_playlists(
        &mut self,
        user_id: impl Into<String>,
    ) -> Builder<'_, F, V, UserPlaylistsEndpoint> {
        self.builder(UserPlaylistsEndpoint {
            id: user_id.into(),
            ..Default::default()
        })
    }

    pub fn create_playlist(
        &mut self,
        user_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Builder<'_, F, V, CreatePlaylistEndpoint> {
        self.builder(CreatePlaylistEndpoint {
            user_id: user_id.into(),
            name: name.into(),
            ..Default::default()
        })
    }

    pub fn featured_playlists(&mut self) -> Builder<'_, F, V, FeaturedPlaylistsEndpoint> {
        self.builder(FeaturedPlaylistsEndpoint::default())
    }

    pub fn category_playlists(
        &mut self,
        category_id: impl Into<String>,
    ) -> Builder<'_, F, V, CategoryPlaylistsEndpoint> {
        self.builder(CategoryPlaylistsEndpoint {
            id: category_id.into(),
            ..Default::default()
        })
    }

    pub async fn get_playlist_image(&mut self, id: impl Into<String>) -> Result<Vec<Image>> {
        self.get::<(), _>(format!("/playlists/{}/images", id.into()), None)
            .await
    }

    pub async fn add_playlist_image(&mut self, id: impl Into<String>, image: &[u8]) -> Result<Nil> {
        let encoded_image = general_purpose::STANDARD.encode(image).into_bytes();
        let body = <Body>::File(encoded_image);

        self.put(format!("/playlists/{}/images", id.into()), body)
            .await
    }

    pub fn search(
        &mut self,
        query: impl Into<String>,
        item_types: &[Item],
    ) -> Builder<'_, F, V, SearchEndpoint> {
        let r#type = query_list(item_types);

        self.builder(SearchEndpoint {
            query: query.into(),
            r#type,
            ..Default::default()
        })
    }

    pub fn show(&mut self, id: impl Into<String>) -> Builder<'_, F, V, ShowEndpoint> {
        self.builder(ShowEndpoint {
            id: id.into(),
            market: None,
        })
    }

    pub fn shows<T: AsRef<str>>(&mut self, ids: &[T]) -> Builder<'_, F, V, ShowsEndpoint> {
        self.builder(ShowsEndpoint {
            ids: query_list(ids),
            market: None,
        })
    }

    pub fn show_episodes(
        &mut self,
        show_id: impl Into<String>,
    ) -> Builder<'_, F, V, ShowEpisodesEndpoint> {
        self.builder(ShowEpisodesEndpoint {
            show_id: show_id.into(),
            ..Default::default()
        })
    }

    pub fn track(&mut self, id: impl Into<String>) -> Builder<'_, F, V, TrackEndpoint> {
        self.builder(TrackEndpoint {
            id: id.into(),
            market: None,
        })
    }

    pub fn tracks<T: AsRef<str>>(&mut self, ids: &[T]) -> Builder<'_, F, V, TracksEndpoint> {
        self.builder(TracksEndpoint {
            ids: query_list(ids),
            market: None,
        })
    }

    pub async fn get_track_audio_features(
        &mut self,
        id: impl Into<String>,
    ) -> Result<AudioFeatures> {
        self.get::<(), _>(format!("/audio-features/{}", id.into()), None)
            .await
    }

    pub async fn get_tracks_audio_features<T: AsRef<str>>(
        &mut self,
        ids: &[T],
    ) -> Result<Vec<AudioFeatures>> {
        self.get("/audio-features".to_owned(), [("ids", query_list(ids))])
            .await
            .map(|a: AudioFeaturesResult| a.audio_features)
    }

    pub async fn get_track_audio_analysis(
        &mut self,
        id: impl Into<String>,
    ) -> Result<AudioAnalysis> {
        self.get::<(), _>(format!("/audio-analysis/{}", id.into()), None)
            .await
    }

    pub fn recommendations<S: SeedType, T: AsRef<str>>(
        &mut self,
        seed: Seed<T, S>,
    ) -> Builder<'_, F, V, RecommendationsEndpoint<S>> {
        let (seed_artists, seed_genres, seed_tracks) = match seed {
            Seed::Artists(ids, _) => (Some(query_list(ids)), None, None),
            Seed::Genres(genres, _) => (None, Some(query_list(genres)), None),
            Seed::Tracks(ids, _) => (None, None, Some(query_list(ids))),
        };

        self.builder(RecommendationsEndpoint {
            seed_artists,
            seed_genres,
            seed_tracks,
            limit: None,
            market: None,
            features: None,
            marker: PhantomData,
        })
    }

    pub async fn get_user(&mut self, id: impl Into<String>) -> Result<User> {
        self.get::<(), _>(format!("/users/{}", id.into()), None)
            .await
    }

    pub async fn check_if_users_follow_playlist<T: AsRef<str>>(
        &mut self,
        playlist_id: impl Into<String>,
        user_ids: &[T],
    ) -> Result<Vec<bool>> {
        self.get(
            format!("/playlists/{}/followers/contains", playlist_id.into()),
            [("ids", query_list(user_ids))],
        )
        .await
    }
}

impl<F: AuthFlow + Authorised, V: Verifier> Client<Token, F, V> {
    pub fn saved_albums(&mut self) -> Builder<'_, F, V, SavedAlbumsEndpoint> {
        self.builder(SavedAlbumsEndpoint::default())
    }

    pub async fn save_albums<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.put("/me/albums".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn remove_saved_albums<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.delete("/me/albums".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn check_saved_albums<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Vec<bool>> {
        self.get("/me/albums/contains".to_owned(), [("ids", query_list(ids))])
            .await
    }

    pub fn saved_audiobooks(&mut self) -> Builder<'_, F, V, SavedAudiobooksEndpoint> {
        self.builder(SavedAudiobooksEndpoint::default())
    }

    pub async fn save_audiobooks<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.put::<(), _>(format!("/me/audiobooks?ids={}", query_list(ids)), None)
            .await
    }

    pub async fn remove_saved_audiobooks<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.delete::<(), _>(format!("/me/audiobooks?ids={}", query_list(ids)), None)
            .await
    }

    pub async fn check_saved_audiobooks<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Vec<bool>> {
        self.get(
            "/me/audiobooks/contains".to_owned(),
            [("ids", query_list(ids))],
        )
        .await
    }

    pub fn saved_episodes(&mut self) -> Builder<'_, F, V, SavedEpisodesEndpoint> {
        self.builder(SavedEpisodesEndpoint::default())
    }

    pub async fn save_episodes<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.put("/me/episodes".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn remove_saved_episodes<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.delete("/me/episodes".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn check_saved_episodes<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Vec<bool>> {
        self.get::<(), _>(
            format!("/me/episodes/contains?ids={}", query_list(ids)),
            None,
        )
        .await
    }

    pub fn current_user_playlists(&mut self) -> Builder<'_, F, V, CurrentUserPlaylistsEndpoint> {
        self.builder(CurrentUserPlaylistsEndpoint::default())
    }

    pub fn saved_shows(&mut self) -> Builder<'_, F, V, SavedShowsEndpoint> {
        self.builder(SavedShowsEndpoint::default())
    }

    pub async fn save_shows<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.put("/me/shows".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn remove_saved_shows<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.delete("/me/shows".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn check_saved_shows<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Vec<bool>> {
        self.get("/me/shows/contains".to_owned(), [("ids", query_list(ids))])
            .await
    }

    pub fn saved_tracks(&mut self) -> Builder<'_, F, V, SavedTracksEndpoint> {
        self.builder(SavedTracksEndpoint::default())
    }

    pub async fn save_tracks<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.put("/me/tracks".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn remove_saved_tracks<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Nil> {
        self.delete("/me/tracks".to_owned(), body_list("ids", ids))
            .await
    }

    pub async fn check_saved_tracks<T: AsRef<str>>(&mut self, ids: &[T]) -> Result<Vec<bool>> {
        self.get("/me/tracks/contains".to_owned(), [("ids", query_list(ids))])
            .await
    }

    pub async fn get_current_user_profile(&mut self) -> Result<User> {
        self.get::<(), _>("/me".to_owned(), None).await
    }

    pub fn current_user_top_items(
        &mut self,
        r#type: UserItemType,
    ) -> Builder<'_, F, V, UserTopItemsEndpoint> {
        self.builder(UserTopItemsEndpoint {
            r#type,
            ..Default::default()
        })
    }

    pub fn follow_playlist(
        &mut self,
        id: impl Into<String>,
    ) -> Builder<'_, F, V, FollowPlaylistBuilder> {
        self.builder(FollowPlaylistBuilder {
            id: id.into(),
            public: None,
        })
    }

    pub async fn unfollow_playlist(&mut self, id: impl Into<String>) -> Result<Nil> {
        self.delete::<(), _>(format!("/playlists/{}/followers", id.into()), None)
            .await
    }

    pub fn followed_artists(&mut self) -> Builder<'_, F, V, FollowedArtistsBuilder> {
        // Currently only the "artist" type is supported, so it's hardcoded.
        self.builder(FollowedArtistsBuilder {
            r#type: "artist".to_owned(),
            ..Default::default()
        })
    }

    pub fn follow_artists<T: AsRef<str>>(
        &mut self,
        ids: &[T],
    ) -> Builder<'_, F, V, FollowUserOrArtistEndpoint> {
        self.builder(FollowUserOrArtistEndpoint {
            r#type: "artist".to_owned(),
            ids: ids.iter().map(|i| i.as_ref().to_owned()).collect(),
        })
    }

    pub fn follow_users<T: AsRef<str>>(
        &mut self,
        ids: &[T],
    ) -> Builder<'_, F, V, FollowUserOrArtistEndpoint> {
        self.builder(FollowUserOrArtistEndpoint {
            r#type: "user".to_owned(),
            ids: ids.iter().map(|i| i.as_ref().to_owned()).collect(),
        })
    }

    pub async fn get_playback_state(&mut self, market: Option<&str>) -> Result<PlaybackState> {
        let market = market.map(|m| [("market", m)]);
        self.get::<[(&str, &str); 1], _>("/me/player".to_owned(), market)
            .await
    }

    pub fn transfer_playback(
        &mut self,
        device_id: impl Into<String>,
    ) -> Builder<'_, F, V, TransferPlaybackEndpoint> {
        self.builder(TransferPlaybackEndpoint {
            device_ids: vec![device_id.into()],
            play: None,
        })
    }

    pub async fn get_available_devices(&mut self) -> Result<Vec<Device>> {
        self.get::<(), _>("/me/player/devices".to_owned(), None)
            .await
            .map(|d: Devices| d.devices)
    }

    pub async fn get_currently_playing_track(
        &mut self,
        market: Option<&str>,
    ) -> Result<PlaybackState> {
        let market = market.map(|m| [("market", m)]);
        self.get::<Option<[(&str, &str); 1]>, _>("/me/player/currently-playing".to_owned(), market)
            .await
    }

    pub fn start_playback(&mut self) -> Builder<'_, F, V, StartPlaybackEndpoint> {
        self.builder(StartPlaybackEndpoint::default())
    }

    pub async fn pause_playback(&mut self, device_id: Option<&str>) -> Result<Nil> {
        let device_id = device_id.map(|d| [("device_id", d)]);
        self.request(Method::PUT, "/me/player/pause".to_owned(), device_id, None)
            .await
    }

    pub async fn skip_to_next(&mut self, device_id: Option<&str>) -> Result<Nil> {
        let device_id = device_id.map(|d| [("device_id", d)]);
        self.request(Method::POST, "/me/player/next".to_owned(), device_id, None)
            .await
    }

    pub async fn skip_to_previous(&mut self, device_id: Option<&str>) -> Result<Nil> {
        let device_id = device_id.map(|d| [("device_id", d)]);
        self.request(
            Method::POST,
            "/me/player/previous".to_owned(),
            device_id,
            None,
        )
        .await
    }

    pub fn seek_to_position(&mut self, position: u32) -> Builder<'_, F, V, SeekToPositionEndpoint> {
        self.builder(SeekToPositionEndpoint {
            position_ms: position,
            device_id: None,
        })
    }

    /// *Note: This endpoint seems to be broken, returning 403 Forbidden "Player command failed: Restriction violated"*
    pub fn set_repeat_mode(
        &mut self,
        repeat_mode: RepeatMode,
    ) -> Builder<'_, F, V, SetRepeatModeEndpoint> {
        self.builder(SetRepeatModeEndpoint {
            state: repeat_mode,
            device_id: None,
        })
    }

    pub fn set_playback_volume(
        &mut self,
        volume: u32,
    ) -> Builder<'_, F, V, SetPlaybackVolumeEndpoint> {
        self.builder(SetPlaybackVolumeEndpoint {
            volume_percent: volume,
            device_id: None,
        })
    }

    /// *Note: This endpoint seems to be broken, returning 403 Forbidden "Player command failed: Restriction violated"*
    pub fn toggle_playback_shuffle(
        &mut self,
        shuffle: bool,
    ) -> Builder<'_, F, V, ToggleShuffleEndpoint> {
        self.builder(ToggleShuffleEndpoint {
            state: shuffle,
            device_id: None,
        })
    }

    pub fn recently_played_tracks(&mut self) -> Builder<'_, F, V, RecentlyPlayedTracksEndpoint> {
        self.builder(RecentlyPlayedTracksEndpoint::default())
    }

    pub async fn get_user_queue(&mut self) -> Result<Queue> {
        self.get::<(), _>("/me/player/queue".to_owned(), None).await
    }

    pub fn add_item_to_queue(
        &mut self,
        uri: impl Into<String>,
    ) -> Builder<'_, F, V, AddItemToQueueEndpoint> {
        self.builder(AddItemToQueueEndpoint {
            uri: uri.into(),
            device_id: None,
        })
    }
}

impl Client<UnAuthenticated, AuthCodeFlow, CsrfVerifier> {
    /// This will exchange the `auth_code` for a token which will allow the client
    /// to make requests.
    ///
    /// `csrf_state` is used for CSRF protection.
    pub async fn authenticate(
        self,
        auth_code: impl Into<String>,
        csrf_state: impl Into<String>,
    ) -> Result<Client<Token, AuthCodeFlow, NoVerifier>> {
        if csrf_state.into() != *self.verifier.0.secret() {
            return Err(Error::InvalidStateParameter);
        }

        let token = self
            .oauth
            .exchange_code(AuthorizationCode::new(auth_code.into()))
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        Ok(Client {
            auto_refresh: self.auto_refresh,
            auth: token,
            oauth: self.oauth,
            http: self.http,
            verifier: NoVerifier,
            marker: PhantomData,
        })
    }
}

impl Client<UnAuthenticated, AuthCodePkceFlow, PkceVerifier> {
    /// This will exchange the `auth_code` for a token which will allow the client
    /// to make requests.
    ///
    /// `csrf_state` is used for CSRF protection.
    pub async fn authenticate(
        self,
        auth_code: impl Into<String>,
        csrf_state: impl Into<String>,
    ) -> Result<Client<Token, AuthCodePkceFlow, NoVerifier>> {
        if csrf_state.into() != *self.verifier.csrf_token.secret() {
            return Err(Error::InvalidStateParameter);
        }

        let token = self
            .oauth
            .exchange_code(AuthorizationCode::new(auth_code.into()))
            .set_pkce_verifier(self.verifier.pkce_verifier)
            .request_async(async_http_client)
            .await?
            .set_timestamps();

        Ok(Client {
            auto_refresh: self.auto_refresh,
            auth: token,
            oauth: self.oauth,
            http: self.http,
            verifier: NoVerifier,
            marker: PhantomData,
        })
    }
}

impl Client<UnAuthenticated, ClientCredsFlow, NoVerifier> {
    /// This will exchange the client credentials for an access token used
    /// to make requests.
    ///
    /// This authentication method doesn't allow for token refreshing or to access
    /// user resources.
    pub async fn authenticate(
        ClientCredsFlow {
            client_id,
            client_secret,
        }: ClientCredsFlow,
    ) -> Result<Client<Token, ClientCredsFlow, NoVerifier>> {
        let oauth = OAuthClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(AUTHORISATION_URL.to_owned()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()),
        );

        let token = oauth
            .exchange_client_credentials()
            .request_async(async_http_client)
            .await?;

        Ok(Client {
            auto_refresh: false,
            auth: token,
            oauth,
            http: reqwest::Client::new(),
            verifier: NoVerifier,
            marker: PhantomData,
        })
    }
}
