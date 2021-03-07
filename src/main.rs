use actix_web::{get, web, App, HttpServer, error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::Logger;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};

use crate::poke::poke_client::PokeClientError;
use crate::shakespeare::shakespeare_client::ShakespeareClientError;
use crate::settings::Settings;

mod poke;
mod shakespeare;
mod settings;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";
static SHAKESPEARE_TRANSLATOR_BASE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

#[derive(Debug, Display, Error, Serialize, Deserialize)]
pub enum ShakespearemonException {
    PokeClientException(PokeClientError),
    ShakespeareClientException(ShakespeareClientError),
}

impl error::ResponseError for ShakespearemonException {
    fn status_code(&self) -> StatusCode {
        match *self {
            ShakespearemonException::PokeClientException(PokeClientError::PokeClientFailed) => StatusCode::INTERNAL_SERVER_ERROR,
            ShakespearemonException::PokeClientException(PokeClientError::PokemonNotFound) => StatusCode::NOT_FOUND,
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
async fn translation_pokemon_shakespearen(web::Path(name): web::Path<String>) -> Result<HttpResponse, ShakespearemonException> {
    poke::poke_client::get_pokemon(POKE_API_BASE_URL, &name).await
        .map_err(|error| {
            ShakespearemonException::PokeClientException(error)
        })?;

    let translation = shakespeare::shakespeare_client::get_shakespearean_translation(SHAKESPEARE_TRANSLATOR_BASE_URL, &name).await
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

    HttpServer::new(|| App::new().wrap(Logger::default()).service(translation_pokemon_shakespearen))
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

    #[actix_rt::test]
    async fn returns_pokemon_named_ozer_not_found() {
        let mut app = test::init_service(App::new().service(translation_pokemon_shakespearen)).await;
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