use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, PartialEq, Serialize, Deserialize)]
pub enum PokeClientException {
    #[display(fmt = "Pokemon Not Found")]
    PokemonNotFound,
    #[display(fmt = "Pokemon Description Not Found")]
    PokemonDescriptionNotFound,
    #[display(fmt = "Unable to process the request")]
    PokeClientWentWrong,
}