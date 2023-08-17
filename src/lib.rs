pub mod auth;
pub mod client;
pub mod endpoint;
mod error;
pub mod model;

pub(crate) fn query_list<T: AsRef<str>>(list: &[T]) -> String {
    list.iter()
        .map(|i| i.as_ref())
        .collect::<Vec<&str>>()
        .join(",")
}

pub type Result<T> = std::result::Result<T, error::Error>;

pub use oauth2::RedirectUrl;
