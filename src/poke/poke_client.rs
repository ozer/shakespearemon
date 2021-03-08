use surf::{get, StatusCode};

use crate::poke::poke_client_exception::PokeClientException;
use crate::poke::poke_species_response::PokeSpeciesResponse;

pub async fn get_pokemon_description(base_url: &str, name: &str) -> Result<String, PokeClientException> {
    let mut url = base_url.to_owned();
    url.push_str("/");
    url.push_str(name);

    let mut response = get(url).await.map_err(|_| {
        PokeClientException::PokeClientWentWrong
    })?;

    match response.status() {
        StatusCode::Ok => {
            let poke_species_response: PokeSpeciesResponse = response.body_json().await.map_err(|_| {
                PokeClientException::PokeClientWentWrong
            })?;
            let flavor_text = extract_english_flavor_text_from_poke_species_response(poke_species_response);

            match flavor_text {
                Some(text) => Ok(text.to_owned()),
                None => Err(PokeClientException::PokemonDescriptionNotFound)
            }
        }
        StatusCode::NotFound => {
            Err(PokeClientException::PokemonNotFound)
        }
        _ => {
            Err(PokeClientException::PokeClientWentWrong)
        }
    }
}

fn extract_english_flavor_text_from_poke_species_response(response: PokeSpeciesResponse) -> Option<String> {
    let first_text_flavor_entry_in_english = response.flavor_text_entries.iter()
        .find(|entry| {
            entry.language.name == "en"
        });

    match first_text_flavor_entry_in_english {
        Some(entry) => {
            Some(entry.flavor_text.to_owned())
        }
        None => None
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, path_regex};
    use crate::poke::poke_species_response::{TextFlavorEntryLanguage, TextFlavorEntry};

    fn generate_poke_species_response (language_name: String) -> PokeSpeciesResponse {
        let flavor_text = "Flavor text".to_owned();
        let flavor_text = vec![TextFlavorEntry::new(flavor_text, language_name)];
        let id = 16;
        let name = "pikachu".to_owned();
        PokeSpeciesResponse::new(id, name, flavor_text)
    }

    #[test]
    fn should_return_none_if_there_is_not_any_english_flavor_text() {
        let language_name = "qwerty".to_owned();
        let response = generate_poke_species_response(language_name);
        let result = extract_english_flavor_text_from_poke_species_response(response);

        assert_eq!(result, None);
    }

    #[test]
    fn should_return_some_description_if_there_is_an_english_flavor_text() {
        let language_name = "en".to_owned();
        let response = generate_poke_species_response(language_name);
        let result = extract_english_flavor_text_from_poke_species_response(response);

        assert_eq!(result, Some("Flavor text".to_owned()));
    }

    #[actix_rt::test]
    #[allow(unused_must_use)]
    async fn throw_pokemon_not_found_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/ozer"))
            .respond_with(ResponseTemplate::new(StatusCode::NotFound))
            .mount(&mock_server)
            .await;

        let pokemon_name = "ozer";

        get_pokemon_description(&mock_server.uri(), pokemon_name).await.map_err(|error| {
            assert_eq!(error, PokeClientException::PokemonNotFound)
        });
    }

    #[actix_rt::test]
    #[allow(unused_must_use)]
    async fn should_throw_poke_client_failed_if_request_returns_too_many_requests() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("ozer"))
            .respond_with(ResponseTemplate::new(StatusCode::TooManyRequests))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        get_pokemon_description(&mock_server.uri(), pokemon).await.map_err(|error| {
            assert_eq!(error, PokeClientException::PokeClientWentWrong)
        });
    }

    #[actix_rt::test]
    async fn returns_pokemon_name() {
        let mock_server = MockServer::start().await;

        let language_name = "en".to_owned();
        let response = generate_poke_species_response(language_name);

        Mock::given(method("GET"))
            .and(path("ozer"))
            .respond_with(ResponseTemplate::new(StatusCode::Ok).set_body_json(response))
            .mount(&mock_server)
            .await;

        let pokemon_name = "ozer";

        let result = get_pokemon_description(&mock_server.uri(), pokemon_name).await.unwrap();
        assert_eq!(result, "Flavor text");
    }
}

