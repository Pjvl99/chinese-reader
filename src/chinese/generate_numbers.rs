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
                format!("{} shi2", base_numbers[tens])
            } else {
                format!("{} shi2 {}", base_numbers[tens], base_numbers[ones])
            };
            let meaning = if ones == 0 {
                format!("{}0", tens)
            } else {
                format!("{}{}", tens, ones)
            };
            words.insert(pinyin, WordDefinition { meaning, category: "number".to_string() });
            };
        }
        for hundreds in 1..=9 {
            for tens in 0..=9 {
                for ones in 0..=9 {
                    let pinyin = if tens == 0 && ones == 0 {
                        format!("{} bai3", base_numbers[hundreds]) // e.g., "yī bǎi" for 100
                    } else if tens == 0 {
                        format!("{} bai3 {}", base_numbers[hundreds], base_numbers[ones]) // e.g., "yī bǎi líng yī" for 101
                    } else if ones == 0 {
                        format!("{} bai3 {} shi2", base_numbers[hundreds], base_numbers[tens]) // e.g., "yī bǎi èr shí" for 120
                    } else {
                        format!(
                            "{} bai3 {} shi2 {}",
                            base_numbers[hundreds], base_numbers[tens], base_numbers[ones]
                        ) // e.g., "yī bǎi èr shí sān" for 123
                    };
                    let meaning = if tens == 0 && ones == 0 {
                        format!("{}00", hundreds) // e.g., "100"
                    } else if tens == 0 {
                        format!("{}0{}", hundreds, ones) // e.g., "101"
                    } else if ones == 0 {
                        format!("{}{}0", hundreds, tens) // e.g., "120"
                    } else {
                        format!("{}{}{}", hundreds, tens, ones) // e.g., "123"
                    };
                    words.insert(
                        pinyin,
                        WordDefinition {
                            meaning,
                            category: "number".to_string(),
                        },
                    );
                }
            }
        }
    }