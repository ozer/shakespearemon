use actix_web::{get, web, App, HttpServer, error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::Logger;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};

use crate::poke::poke_client::PokeClientException;
use crate::shakespeare::shakespeare_client::ShakespeareClientError;
use crate::settings::Settings;

mod poke;
mod shakespeare;
mod settings;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";
static SHAKESPEARE_TRANSLATOR_BASE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

#[derive(Debug, Display, Error, Serialize, Deserialize)]
pub enum ShakespearemonException {
    PokeClientException(PokeClientException),
    ShakespeareClientException(ShakespeareClientError),
}

impl error::ResponseError for ShakespearemonException {
    fn status_code(&self) -> StatusCode {
        match *self {
            ShakespearemonException::PokeClientException(PokeClientException::PokeClientFailed) => StatusCode::INTERNAL_SERVER_ERROR,
            ShakespearemonException::PokeClientException(PokeClientException::PokemonNotFound) => StatusCode::NOT_FOUND,
            ShakespearemonException::ShakespeareClientException(ShakespeareClientError::TranslationNotFound) => StatusCode::NOT_FOUND,
            ShakespearemonException::ShakespeareClientException(ShakespeareClientError::ShakespeareClientFailed) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(self.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShakespearemonResponse {
    name: String,
    description: String,
}

#[get("/pokemon/{name}")]
async fn translation_pokemon_shakespearen(data: web::Data<Settings>, web::Path(name): web::Path<String>) -> Result<HttpResponse, ShakespearemonException> {
    poke::poke_client::get_pokemon(&data.application.poke_api_base_url, &name).await
        .map_err(|error| {
            ShakespearemonException::PokeClientException(error)
        })?;

    let translation = shakespeare::shakespeare_client::get_shakespearean_translation(&data.application.shakespeare_translator_api_base_url, &name).await
        .map_err(|error| {
            ShakespearemonException::ShakespeareClientException(error)
        })?;

    let shakespearemon_response = ShakespearemonResponse {
        description: translation,
        name,
    };

    Ok(HttpResponse::Ok().json(shakespearemon_response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().map_err(|error| {
        Error::new(ErrorKind::Other, format!("Config failed with an error: {:?}", error))
    })?;

    let addr = format!("{}:{}", settings.application.host, settings.application.port);

    HttpServer::new(|| App::new()
        .data(Settings::new().expect("Config failed!"))
        .wrap(Logger::default()).service(translation_pokemon_shakespearen))
        .bind(addr)?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test};
    use actix_web::http::StatusCode;
    use actix_web::test::{read_body_json};
    use crate::settings::Application;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use surf::{StatusCode as SurfStatusCode};

    #[actix_rt::test]
    async fn returns_pokemon_named_ozer_not_found() {
        let mock_server = MockServer::start().await;

        let application = Application {
            host: "127.0.0.1".to_owned(),
            port: 8080,
            poke_api_base_url: mock_server.uri(),
            shakespeare_translator_api_base_url: mock_server.uri()
        };

        Mock::given(method("GET"))
            .and(path("/ozer"))
            .respond_with(ResponseTemplate::new(SurfStatusCode::NotFound))
            .mount(&mock_server)
            .await;

        let mut app = test::init_service(App::new()
            .data(Settings {
                application
            })
            .service(translation_pokemon_shakespearen)).await;

        let req = test::TestRequest::get()
            .uri("/pokemon/ozer").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn translates_pokemon_to_shakespaearen() {

        let mut app = test::init_service(App::new().service(translation_pokemon_shakespearen)).await;
        let req = test::TestRequest::get()
            .uri("/pokemon/pikachu").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let shakespearemon_response: ShakespearemonResponse = read_body_json(resp).await;
        assert_eq!(shakespearemon_response.name, "pikachu");
    }
}