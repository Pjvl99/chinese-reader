use std::collections::HashMap;
use crate::words::WordDefinition;


pub fn generate_number_meanings(words: &mut HashMap<String, WordDefinition>) {
    let base_numbers: Vec<String> = words
        .iter()
        .filter(|(_, def)| def.category == "number") // Filter by category "number"
        .map(|(pinyin, _)| pinyin.clone()) // Extract pinyin
        .collect();
    if base_numbers.len() != 11 {
        panic!("Expected 11 base numers (0-10) in the YAML file, but found {}.", base_numbers.len());
    }
    for tens in 1..=9 {
        for ones in 0..=9 {
            let pinyin = if ones == 0 {
                format!("{} shí", base_numbers[tens])
            } else {
                format!("{} shí {}", base_numbers[tens], base_numbers[ones])
            };
            let meaning = if ones == 0 {
                format!("{}0", tens)
            } else {
                format!("{}{}", tens, ones)
            };
            words.insert(pinyin, WordDefinition { meaning, category: "number".to_string() });
            };
        }
    }