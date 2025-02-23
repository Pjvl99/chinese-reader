// Declare the `words` module, which contains the logic for loading word meanings from a YAML file.
mod words;
use std::fs;
// Import the `Parser` trait and related types for working with parsed pairs.
use pest::Parser;
use pest_derive::Parser;

mod chinese {
    pub mod generate_numbers;
}
use chinese::generate_numbers::generate_number_meanings;

// Derive the `Parser` trait for the `ChineseParser` struct.
#[derive(Parser)]
#[grammar = "assets/chinese.pest"]
struct ChineseParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load word meanings from the YAML file.
    let root_dir = "assets/beginner_conversational";

    // Initialize an empty `WordsYaml` structure to hold all the words.
    let mut words = words::WordsYaml { words: std::collections::HashMap::new() };

    load_yml_files_recursive(&root_dir, &mut words)?;
    generate_number_meanings(&mut words.words); // Generate number meanings (11-99)

    // Example sentences to be parsed and validated.
    let sentences = vec!["wo3 shi4 zhong1 guo2 ren2", "wo3 bu4 xi3 huan1 chi1 zhong1 guo2 cai4"];

    // Process each sentence.
    for sentence in sentences {
        let trimmed_sentence = sentence.trim();
        println!("Parsing sentence: {}", trimmed_sentence);

        match ChineseParser::parse(Rule::sentence, trimmed_sentence) {
            Ok(pairs) => {
                let mut final_words = String::new();
                for pair in pairs {
                    for inner_pair in pair.into_inner() {
                        let word = inner_pair.as_str();
                        if final_words.is_empty() && word.contains(" ") {
                            continue; // Skip empty words.
                        }
                        // Append the word to the final_words buffer.
                        final_words.push_str(word);
                        // If the word is not in the dictionary, skip printing its definition.
                        if let Some(definition) = words.words.get(&final_words) {
                            println!("  {}: {} ({})", final_words, definition.meaning, definition.category);
                            final_words.clear(); // Reset the buffer after printing the definition.
                        }
                    }
                }
                // Handle any remaining words in the buffer.
                if !final_words.is_empty() {
                    panic!("x");
                }
            }
            Err(e) => println!("Invalid sentence: {}, error: {}", trimmed_sentence, e),
        }
    }

    Ok(())
}

fn load_yml_files_recursive(dir_path: &str, words: &mut words::WordsYaml) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate over all entries in the directory.
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // If the entry is a directory, recursively traverse it.
        if path.is_dir() {
            load_yml_files_recursive(path.to_str().unwrap(), words)?;
        }
        // If the entry is a YAML file, load its words.
        else if path.extension().and_then(|ext| ext.to_str()) == Some("yml") {
            println!("Loading file: {:?}", path);

            // Load words from the current file and merge them into the `words` variable.
            let file_words = words::load_words(path.to_str().unwrap())?;
            words.words.extend(file_words.words);
        }
    }

    Ok(())
}