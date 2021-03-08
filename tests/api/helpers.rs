use serde::Serialize

use shakespearemon::poke::poke_species_response::{PokeSpeciesResponse, TextFlavorEntry};
use shakespearemon::settings::Application;

#[derive(Serialize)]
pub struct UndefinedResponse {
    pub message: String
}

pub fn generate_poke_species_response(language_name: String) -> PokeSpeciesResponse {
    let flavor_text = "Flavor text".to_owned();
    let flavor_text = vec![TextFlavorEntry::new(flavor_text, language_name)];
    let id = 16;
    let name = "pikachu".to_owned();
    PokeSpeciesResponse::new(id, name, flavor_text)
}

pub fn get_application(uri: String) -> Application {
    Application {
        host: "127.0.0.1".to_owned(),
        port: 8080,
        poke_api_base_url: uri.clone(),
        shakespeare_translator_api_base_url: uri,
    }
}