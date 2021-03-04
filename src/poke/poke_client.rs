use surf::{get, StatusCode};
use derive_more::{Display, Error};

use crate::poke::pokemon::Pokemon;

#[derive(Debug, Display, Error, PartialEq)]
pub enum PokeClientError {
    #[display(fmt = "Pokemon is not found")]
    PokemonNotFound,
    #[display(fmt = "PokeClient went terribly wrong...")]
    PokeClientFailed,
}

pub async fn get_pokemon(base_url: &str, name: &str) -> Result<String, PokeClientError> {
    let mut url = base_url.to_owned();
    url.push_str("/");
    url.push_str(name);

    let mut res = get(url).await.map_err(|error| {
        println!("[Fetch Pokemon]: what is this error here? {:?}", error);
        PokeClientError::PokeClientFailed
    })?;

    println!("what is the status code? {:?}", res.status());

    if res.status() == StatusCode::Ok {
        Ok(name.to_owned())
    } else {
        Err(PokeClientError::PokemonNotFound)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, path_regex};
    use actix_web::error::ParseError::Status;

    #[async_std::test]
    async fn throw_pokemon_not_found_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/ozer"))
            .respond_with(ResponseTemplate::new(StatusCode::NotFound))
            .mount(&mock_server)
            .await;

        let pokemon_name = "ozer";

        get_pokemon(&mock_server.uri(), pokemon_name).await.map_err(|error| {
            assert_eq!(error, PokeClientError::PokemonNotFound)
        });
    }

    #[async_std::test]
    async fn returns_pokemon_name() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("ozer"))
            .respond_with(ResponseTemplate::new(StatusCode::Ok))
            .mount(&mock_server)
            .await;

        let pokemon_name = "ozer";

        let result = get_pokemon(&mock_server.uri(), pokemon_name).await.unwrap();
        assert_eq!(result, pokemon_name);
    }
}

