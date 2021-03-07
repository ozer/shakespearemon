use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    pub total: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub translated: String,
    pub text: String,
    pub translation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShakespeareTranslationResponse {
    pub success: Success,
    pub contents: Content,
}

impl ShakespeareTranslationResponse {
    #[allow(dead_code)]
    pub fn new(translated: String, text: String, translation: String) -> Self {
        let success = Success {
            total: 1
        };
        let contents = Content {
            translation,
            text,
            translated,
        };
        ShakespeareTranslationResponse {
            success,
            contents,
        }
    }
}