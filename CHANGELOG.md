# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [0.3.1] - 2023-08-10

### Changed

- Changed the signature of methods that take several IDs to allow for more flexibility in the arguments.

# [0.3.0] - 2023-08-07

### Added

- Added two new authorisation flows: Authorisation Code Flow (no PKCE) and Client Credentials Flow.
- Added endpoints:
  - All [artist endpoints](https://developer.spotify.com/documentation/web-api/reference/get-an-artist)
  - All [audiobook endpoints](https://developer.spotify.com/documentation/web-api/reference/get-an-audiobook)
  - All [category endpoints](https://developer.spotify.com/documentation/web-api/reference/get-categories)
  - All [chapter endpoints](https://developer.spotify.com/documentation/web-api/reference/get-a-chapter)
      
    *Note: they return `500 Server error`, which is an issue with the API.*
  - All [episode endpoints](https://developer.spotify.com/documentation/web-api/reference/get-an-episode)
  - The [genre endpoint](https://developer.spotify.com/documentation/web-api/reference/get-recommendation-genres)

### Changed

- Removed duplicate token refresh methods from `Client<AuthCodeGrantPKCEFlow>` and the new `Client<AuthCodeGrantFlow>` since they were the same as the generic implementation.
- Client now takes a new generic type parameter: `A: AuthenticatedState`. The `Token` type now implements said trait and endpoint methods are *only* available on the `Client<Token, F>` type now. This ensures endpoint methods can only be called after authentication (not taking into account expired tokens).
- Removed the existing types for the Implicit Grant flow as it will not be implemented: it's very basic not recommended.
- Removed `oauth2::AuthorizationUrl` from the public API and re-exported `oauth2::RedirectUrl`.

# [0.2.0] - 2023-08-06

### Changed

- Moved from `Option<T>` for optional parameters to builders for each endpoint.

## [0.1.2] - 2023-08-04

### Added

- All [album endpoints](https://developer.spotify.com/documentation/web-api/reference/get-an-album) from the Spotify API.
- `Spotify` variant for the `Error` enum and deserializing into a struct that is converted into said variant.
- `RefreshUnavailable` variant of the `Error` enum, for when you're requesting a refresh but the auth flow doesn't support it.
- Auto refresh for the token (if enabled).
- Created CHANGELOG.md.

### Changed

- Slight internal changes to the model.
- The `flow` field of the client is gone, instead using `PhantomData` now.

## [0.1.1] - 2023-08-03

### Changed

- Added additional `Cargo.toml` metadata.

## [0.1.0] - 2023-08-03

- Initial release.