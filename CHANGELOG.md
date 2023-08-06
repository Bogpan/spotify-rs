# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [0.2.0] - 2023-08

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