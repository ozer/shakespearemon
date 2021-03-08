use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShakespeareTranslationRequest {
    pub text: String
}