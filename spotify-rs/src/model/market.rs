use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Markets {
    pub(crate) markets: Vec<String>,
}
