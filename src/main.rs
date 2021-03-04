use actix_web::{get, web, App, HttpServer, Responder, error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

use crate::poke::poke_client::PokeClientError;
use crate::shakespeare::shakespeare_client::{ShakespeareClientError, ShakespeareTranslation};
use std::convert::TryInto;

mod poke;
mod shakespeare;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";
static SHAKESPEARE_TRANSLATOR_BASE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

#[derive(Debug, Display, Error)]
pub enum ShakespearemonException {
    PokeClientException(PokeClientError),
    ShakespeareClientException(ShakespeareClientError),
}

impl error::ResponseError for ShakespearemonException {
    fn status_code(&self) -> StatusCode {
        match *self {
            ShakespearemonException::PokeClientException(PokeClientError::PokeClientFailed) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShakespearemonResponse {
    name: String,
    description: String,
}

#[get("/pokemon/{name}")]
async fn translation_pokemon_shakespearen(web::Path((name)): web::Path<(String)>) -> Result<HttpResponse, ShakespearemonException> {
    poke::poke_client::get_pokemon(POKE_API_BASE_URL, &name).await
        .map_err(|error| {
            ShakespearemonException::PokeClientException(error)
        })?;

    let translation = shakespeare::shakespeare_client::get_shakespearean_translation(SHAKESPEARE_TRANSLATOR_BASE_URL, &name).await
        .map_err(|error| {
            ShakespearemonException::ShakespeareClientException(error)
        })?;

    println!("what is this translation? {:?}", translation);

    let shakespearemon_response = ShakespearemonResponse {
        description: translation,
        name,
    };

    Ok(HttpResponse::Ok().json(shakespearemon_response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(translation_pokemon_shakespearen))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}