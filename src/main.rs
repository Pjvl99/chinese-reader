// Declare the `words` module, which contains the logic for loading word meanings from a YAML file.
mod words;
// Import the `Parser` trait and related types for working with parsed pairs.
use pest::Parser;
use pest_derive::Parser;

mod chinese {
    pub mod beginner_conversational {
        pub mod level1 {
            pub mod unit1 {
                pub mod generate_numbers;
            }
        }

    }
}
use chinese::beginner_conversational::level1::unit1::generate_numbers::generate_number_meanings;

// Derive the `Parser` trait for the `ChineseParser` struct.
#[derive(Parser)]
#[grammar = "assets/beginner_conversational/level1/unit1/chinese.pest"]
struct ChineseParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load word meanings from the YAML file.
    let path = "assets/beginner_conversational/level1/unit1/english.yml";
    let mut words = words::load_words(path)?;
    generate_number_meanings(&mut words.words);
    // Example sentences to be parsed and validated.
    let sentences = vec![
        "wǒ shì zhōng guó rén",
        "nǐ shì yīng guó",
        "wǒ shì Alice",
        "nǐ shì Bob",

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