// Declare the `words` module, which contains the logic for loading word meanings from a YAML file.
mod words;
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
    let path = "assets/beginner_conversational/level1/unit1/english.yml";
    let mut words = words::load_words(path)?;
    generate_number_meanings(&mut words.words); // Generate number meanings (11-99)
    // Example sentences to be parsed and validated.
    let sentences = vec![
        "wo3 shi4 zhong1 guo2 ren2",
    ];

    for sentence in sentences {
        let trimmed_sentence = sentence.trim();
        println!("{}", trimmed_sentence);
        match ChineseParser::parse(Rule::sentence, trimmed_sentence) {
            Ok(pairs) => {
                println!("Valid sentence: {}", sentence);
                for pair in pairs {
                    for inner_pair in pair.into_inner() {
                        let word = inner_pair.as_str();
                        if let Some(definition) = words.words.get(word) {
                            println!("  {}: {} ({})", word, definition.meaning, definition.category);
                        }
                    }
                }
            }
            Err(e) => println!("Invalid sentence: {}, error: {}", trimmed_sentence, e),
        }
    }

    Ok(())
}