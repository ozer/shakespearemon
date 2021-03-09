extern crate log;

use std::io::{Error, ErrorKind};

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use crate::settings::Settings;
use crate::translation_service::translate_pokemon_description_by_shakespeare;

mod poke;
mod shakespeare;
mod settings;
mod translation_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let settings = Settings::new().map_err(|error| {
        Error::new(ErrorKind::Other, format!("Config failed with an error: {:?}", error))
    })?;

    let addr = format!("{}:{}", settings.application.host, settings.application.port);

    HttpServer::new(|| App::new()
        .data(Settings::new().expect("Config failed!"))
        .wrap(Logger::default()).service(translate_pokemon_description_by_shakespeare))
        .bind(addr)?
        .run()
        .await
}