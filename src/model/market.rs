use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Markets {
    pub(crate) markets: Vec<String>,
}
