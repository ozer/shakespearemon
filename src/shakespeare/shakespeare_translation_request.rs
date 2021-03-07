use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ShakespeareTranslationRequest {
    pub text: String
}