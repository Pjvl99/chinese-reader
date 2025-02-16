use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct WordDefinition {
    pub meaning: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordsYaml {
    pub words: HashMap<String, WordDefinition>,
}

pub fn load_words(file_path: &str) -> Result<WordsYaml, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let words: WordsYaml = serde_yaml::from_str(&file_content)?;
    Ok(words)
}