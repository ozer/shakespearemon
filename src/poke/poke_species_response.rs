use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TextFlavorEntryLanguage {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct TextFlavorEntry {
    pub flavor_text: String,
    pub language: TextFlavorEntryLanguage,
}

#[derive(Serialize, Deserialize)]
pub struct PokeSpeciesResponse {
    pub id: u16,
    pub name: String,
    pub flavor_text_entries: Vec<TextFlavorEntry>,
}

impl TextFlavorEntry {
    #[allow(dead_code)]
    pub fn new (flavor_text: String, language_name: String) -> Self {
        let language = TextFlavorEntryLanguage {
            name: language_name
        };
        TextFlavorEntry {
            flavor_text,
            language
        }
    }
}

impl PokeSpeciesResponse {
    #[allow(dead_code)]
    pub fn new(id: u16, name: String, flavor_text_entries: Vec<TextFlavorEntry>) -> Self {
        PokeSpeciesResponse {
            id,
            name,
            flavor_text_entries,
        }
    }
}