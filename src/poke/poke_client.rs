use surf::{get, StatusCode};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Display, Error, PartialEq, Serialize, Deserialize)]
pub enum PokeClientException {
    #[display(fmt = "Pokemon Not Found")]
    PokemonNotFound,
    #[display(fmt = "Unable to process the request")]
    PokeClientFailed,
}

pub async fn get_pokemon(base_url: &str, name: &str) -> Result<String, PokeClientException> {
    let mut url = base_url.to_owned();
    url.push_str("/");
    url.push_str(name);

    let res = get(url).await.map_err(|_| {
        PokeClientException::PokeClientFailed
    })?;

    match res.status() {
        StatusCode::Ok => {
            Ok(name.to_owned())
        }
        StatusCode::NotFound => {
            Err(PokeClientException::PokemonNotFound)
        }
        _ => {
            Err(PokeClientException::PokeClientFailed)
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, path_regex};

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

        get_pokemon(&mock_server.uri(), pokemon_name).await.map_err(|error| {
            assert_eq!(error, PokeClientException::PokemonNotFound)
        });
    }

    #[actix_rt::test]
    #[allow(unused_must_use)]
    async fn should_throw_poke_client_failed_if_request_returns_too_many_requests() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("ozer"))
            .respond_with(ResponseTemplate::new(StatusCode::Ok))
            .mount(&mock_server)
            .await;

        let pokemon = "ozer";

        get_pokemon(&mock_server.uri(), pokemon).await.map_err(|error| {
            assert_eq!(error, PokeClientException::PokeClientFailed)
        });
    }

    #[actix_rt::test]
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

