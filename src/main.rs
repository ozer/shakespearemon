use actix_web::{get, web, App, HttpServer, Responder, error, HttpResponse};
use crate::poke::poke_client::PokeClientError;
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};

mod poke;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";
static SHAKESPEARE_TRANSLATOR_BASE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

impl error::ResponseError for PokeClientError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            PokeClientError::PokeClientFailed => StatusCode::INTERNAL_SERVER_ERROR,
            PokeClientError::PokemonNotFound => StatusCode::BAD_REQUEST,
        }
    }
}


#[get("/pokemon/{name}")]
async fn index(web::Path((name)): web::Path<(String)>) -> Result<String, PokeClientError> {
    poke::poke_client::get_pokemon(POKE_API_BASE_URL, &name).await?;

    Ok(String::from("asdsa"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}