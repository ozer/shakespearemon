use surf::{post, StatusCode};

use crate::shakespeare::shakespeare_client_exception::ShakespeareClientException;
use crate::shakespeare::shakespeare_translation_request::ShakespeareTranslationRequest;
use crate::shakespeare::shakespeare_translation_response::ShakespeareTranslationResponse;

pub async fn get_shakespearean_translation(url: &str, text: &str) -> Result<String, ShakespeareClientException> {
    let body = surf::Body::from_json(&ShakespeareTranslationRequest {
        text: text.to_owned()
    }).map_err(|_| {
        ShakespeareClientException::ShakespeareClientWentWrong
    })?;

    let mut res = post(url)
        .body(body)
        .await.map_err(|_| {
        ShakespeareClientException::ShakespeareClientWentWrong
    })?;

    match res.status() {
        StatusCode::Ok => {
            let translation: ShakespeareTranslationResponse = res.body_json().await.map_err(|_| {
                ShakespeareClientException::ShakespeareClientWentWrong
            })?;
            Ok(translation.contents.translated)
        }
        StatusCode::NotFound => {
            Err(ShakespeareClientException::TranslationNotFound)
        }
        _ => Err(ShakespeareClientException::ShakespeareClientWentWrong)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, path_regex};

    use super::*;

    #[actix_rt::test]
    #[allow(unused_must_use)]
    async fn should_throw_translation_not_found_error_if_request_returns_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(StatusCode::NotFound))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        get_shakespearean_translation(&mock_server.uri(), pokemon).await.map_err(|error| {
            assert_eq!(error, ShakespeareClientException::TranslationNotFound)
        });
    }

    #[actix_rt::test]
    #[allow(unused_must_use)]
    async fn should_throw_shakespeare_client_failed_if_request_returns_too_many_requests() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(StatusCode::TooManyRequests))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        get_shakespearean_translation(&mock_server.uri(), pokemon).await.map_err(|error| {
            assert_eq!(error, ShakespeareClientException::ShakespeareClientWentWrong)
        });
    }

    #[actix_rt::test]
    async fn should_return_translated_string() {
        let mock_server = MockServer::start().await;

        let translation = ShakespeareTranslationResponse::new(String::from("translated"), String::from("text"), String::from("translation"));

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(translation))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        let translated = get_shakespearean_translation(&mock_server.uri(), pokemon).await.unwrap();
        assert_eq!(translated, "translated");
    }
}
