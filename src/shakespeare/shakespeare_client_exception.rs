use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, PartialEq, Serialize, Deserialize)]
pub enum ShakespeareClientException {
    #[display(fmt = "Translation not found")]
    TranslationNotFound,
    #[display(fmt = "Unable to process the request")]
    ShakespeareClientWentWrong,
}