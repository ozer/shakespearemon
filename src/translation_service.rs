extern crate derive_more;

use actix_web::{get, HttpResponse, web};
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::ResponseError;
use actix_web::http::{header, StatusCode};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::{poke, shakespeare};
use crate::poke::poke_client_exception::PokeClientException;
use crate::settings::Settings;
use crate::shakespeare::shakespeare_client_exception::ShakespeareClientException;

#[derive(Debug, Error, Serialize, Deserialize, Display)]
pub enum ShakespearemonException {
    PokeClientException(PokeClientException),
    ShakespeareClientException(ShakespeareClientException),
}

impl ResponseError for ShakespearemonException {
    fn status_code(&self) -> StatusCode {
        match *self {
            ShakespearemonException::PokeClientException(PokeClientException::PokeClientWentWrong) => StatusCode::INTERNAL_SERVER_ERROR,
            ShakespearemonException::PokeClientException(PokeClientException::PokemonNotFound) => StatusCode::NOT_FOUND,
            ShakespearemonException::PokeClientException(PokeClientException::PokemonDescriptionNotFound) => StatusCode::NOT_FOUND,
            ShakespearemonException::ShakespeareClientException(ShakespeareClientException::TranslationNotFound) => StatusCode::NOT_FOUND,
            ShakespearemonException::ShakespeareClientException(ShakespeareClientException::ShakespeareClientWentWrong) => StatusCode::INTERNAL_SERVER_ERROR,
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
    pub name: String,
    pub description: String,
}

#[get("/pokemon/{name}")]
pub async fn translate_pokemon_description_by_shakespeare(data: web::Data<Settings>, web::Path(name): web::Path<String>) -> Result<HttpResponse, ShakespearemonException> {
    let pokemon_description = poke::poke_client::get_pokemon_description(&data.application.poke_api_base_url, &name).await
        .map_err(|error| {
            ShakespearemonException::PokeClientException(error)
        })?;

    let translation = shakespeare::shakespeare_client::get_shakespearean_translation(&data.application.shakespeare_translator_api_base_url, &pokemon_description).await
        .map_err(|error| {
            ShakespearemonException::ShakespeareClientException(error)
        })?;

    let shakespearemon_response = ShakespearemonResponse {
        description: translation,
        name,
    };

    Ok(HttpResponse::Ok().json(shakespearemon_response))
}