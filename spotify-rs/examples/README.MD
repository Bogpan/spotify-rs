# Examples
This folder contains several examples, showing you how to use `spotify-rs`.

You might see that the main functions return
`Result<(), Box<dyn std::error::Error>>`.
This is only for making the examples more concise and should not be copied as is.

If you want to run any of the examples, you'll need to set the environment variables in [.env](.env).

Please note that this is *not* the only way one can use `spotify-rs`, as there are
a variety of approaches to both going through the OAuth process, as well as
integrating the library within others (e.g. a web server like axum).
