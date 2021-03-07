use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Display, Error, PartialEq, Serialize, Deserialize)]
pub enum PokeClientException {
    #[display(fmt = "Pokemon Not Found")]
    PokemonNotFound,
    #[display(fmt = "Unable to process the request")]
    PokeClientWentWrong,
}