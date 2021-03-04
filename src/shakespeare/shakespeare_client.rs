use surf::{post, StatusCode, Body};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};
use surf::http::Mime;

#[derive(Debug, Display, Error, PartialEq)]
pub enum ShakespeareClientError {
    #[display(fmt = "Translation not found")]
    TranslationNotFound,
    #[display(fmt = "ShakespeareClient went terribly wrong...")]
    ShakespeareClientFailed,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct Success {
    total: i32
}

#[derive(Serialize,Deserialize, Debug)]
pub struct Content {
    translated: String,
    text: String,
    translation: String
}

#[derive(Serialize,Deserialize, Debug)]
pub struct ShakespeareTranslation {
    success: Success,
    contents: Content
}

#[derive(Serialize,Deserialize)]
pub struct ShakespeareTranslationRequestBody {
    text: String
}

pub async fn get_shakespearean_translation(base_url: &str, text: &str) -> Result<String, ShakespeareClientError> {
    let body = surf::Body::from_json(&ShakespeareTranslationRequestBody {
        text: text.to_owned()
    }).map_err(|_| {
        ShakespeareClientError::ShakespeareClientFailed
    })?;

    let mut res = post(base_url)
        .body(body)
        .await.map_err(|_| {
        ShakespeareClientError::ShakespeareClientFailed
    })?;

    println!("what is the status code? {:?}", res.status());

    if res.status() == StatusCode::Ok {
        let translation: ShakespeareTranslation = res.body_json().await.map_err(|_| {
            ShakespeareClientError::TranslationNotFound
        })?;
        println!("what is the translation? {:?}", translation);
        Ok(translation.contents.translation)
    } else {
        Err(ShakespeareClientError::TranslationNotFound)
    }
}
