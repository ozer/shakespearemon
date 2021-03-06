use surf::{post, StatusCode};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Display, Error, PartialEq, Serialize, Deserialize)]
pub enum ShakespeareClientError {
    #[display(fmt = "Translation not found")]
    TranslationNotFound,
    #[display(fmt = "ShakespeareClient went terribly wrong...")]
    ShakespeareClientFailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    total: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    translated: String,
    text: String,
    translation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShakespeareTranslation {
    success: Success,
    contents: Content,
}

impl ShakespeareTranslation {
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
        ShakespeareTranslation {
            success,
            contents,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShakespeareTranslationRequestBody {
    text: String
}

pub async fn get_shakespearean_translation(url: &str, text: &str) -> Result<String, ShakespeareClientError> {
    let body = surf::Body::from_json(&ShakespeareTranslationRequestBody {
        text: text.to_owned()
    }).map_err(|_| {
        ShakespeareClientError::ShakespeareClientFailed
    })?;

    let mut res = post(url)
        .body(body)
        .await.map_err(|_| {
        ShakespeareClientError::ShakespeareClientFailed
    })?;

    match res.status() {
        StatusCode::Ok => {
            let translation: ShakespeareTranslation = res.body_json().await.map_err(|_| {
                ShakespeareClientError::TranslationNotFound
            })?;
            Ok(translation.contents.translated)
        }
        StatusCode::BadRequest |
        StatusCode::NotFound => {
            Err(ShakespeareClientError::TranslationNotFound)
        }
        _ => Err(ShakespeareClientError::ShakespeareClientFailed)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, path_regex};

    #[async_std::test]
    #[allow(unused_must_use)]
    async fn should_throw_translation_not_found_error_if_request_returns_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(StatusCode::NotFound))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        get_shakespearean_translation(&mock_server.uri(), pokemon).await.map_err(|error| {
            assert_eq!(error, ShakespeareClientError::TranslationNotFound)
        });
    }

    #[async_std::test]
    #[allow(unused_must_use)]
    async fn should_throw_shakespeare_client_failed_if_request_returns_too_many_requests() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(StatusCode::TooManyRequests))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        get_shakespearean_translation(&mock_server.uri(), pokemon).await.map_err(|error| {
            assert_eq!(error, ShakespeareClientError::ShakespeareClientFailed)
        });
    }

    #[async_std::test]
    async fn should_return_translated_string() {
        let mock_server = MockServer::start().await;

        let translation = ShakespeareTranslation::new(String::from("translated"), String::from("text"), String::from("translation"));

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(translation))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        let translated = get_shakespearean_translation(&mock_server.uri(), pokemon).await.unwrap();
        assert_eq!(translated, "translated");
    }
}
