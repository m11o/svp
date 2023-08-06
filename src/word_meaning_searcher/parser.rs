pub mod frequency;

use regex::Regex;
use frequency::Frequency;

pub struct Parser {}

impl Parser {
    pub fn parse_meaning(responded_message: String) -> Option<String> {
        let meaning_regex = Regex::new(r"意味:\s(.+?)\n").unwrap();
        match meaning_regex.captures(&responded_message) {
            Some(meaning_captures) => match meaning_captures.get(1) {
                Some(meaning) => Some(meaning.as_str().to_owned()),
                None => None,
            },
            None => None,
        }
    }

    pub fn parse_examples(responded_message: String) -> Option<Vec<String>> {
        let examples_regex = Regex::new(r"例文:\s?\n((.+\n)+)").unwrap();
        match examples_regex.captures(&responded_message) {
            Some(examples_captures) => match examples_captures.get(1) {
                Some(examples) => Some(
                    examples
                        .as_str()
                        .trim()
                        .split("\n")
                        .map(|example| example.to_string())
                        .collect(),
                ),
                None => None,
            },
            None => None,
        }
    }

    pub fn parse_frequency(responded_message: String) -> Option<Frequency> {
        let frequency_regex = Regex::new(r"頻度:\s(.+?)\s?\(").unwrap();
        match frequency_regex.captures(&responded_message) {
            Some(frequency_captures) => match frequency_captures.get(1) {
                Some(frequency) => Some(Frequency::str2enum(frequency.as_str().to_string())),
                None => None,
            },
            None => None,
        }
    }

    pub fn parse_collations(responded_message: String) -> Option<Vec<String>> {
        let collations_regex = Regex::new(r"collocations:\s?\n((.+\n?)+)").unwrap();
        match collations_regex.captures(&responded_message) {
            Some(examples_captures) => match examples_captures.get(1) {
                Some(examples) => Some(
                    examples
                        .as_str()
                        .trim()
                        .split("\n")
                        .map(|example| example.to_string())
                        .collect(),
                ),
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::word_meaning_searcher::parser::frequency::Frequency;

    const RESPONDED_MESSAGE: &str = "意味: 中毒性のある、依存性のある\n頻度: high (7)\n例文: \n1. Video games can be highly addictive, causing people to spend hours playing them.\n2. The new smartphone app is so addictive that I can't stop using it.\n3. Some people find social media to be addictive and spend hours scrolling through their feeds.\n\ncollocations:\n- addictive behavior\n- addictive personality\n- addictive substances\n- addictive tendencies\n- addictive habits";

    #[test]
    fn it_parse_meaning() {
        let meaning = super::Parser::parse_meaning(RESPONDED_MESSAGE.to_string()).unwrap();
        assert_eq!(meaning, "中毒性のある、依存性のある");
    }

    #[test]
    fn it_parse_examples() {
        let examples = super::Parser::parse_examples(RESPONDED_MESSAGE.to_string()).unwrap();
        assert_eq!(examples.len(), 3);
        assert_eq!(
            examples[0],
            "1. Video games can be highly addictive, causing people to spend hours playing them."
        );
        assert_eq!(
            examples[1],
            "2. The new smartphone app is so addictive that I can't stop using it."
        );
        assert_eq!(examples[2], "3. Some people find social media to be addictive and spend hours scrolling through their feeds.");
    }

    #[test]
    fn it_return_option_none_when_examples_not_found() {
        let examples = super::Parser::parse_examples(
            "意味: 中毒性のある、依存性のある\n頻度: high (7)\n".to_string(),
        );
        assert_eq!(examples, None);
    }

    #[test]
    fn it_parse_frequency() {
        let frequency = super::Parser::parse_frequency(RESPONDED_MESSAGE.to_string()).unwrap();
        assert_eq!(frequency, Frequency::High);
    }

    #[test]
    fn it_return_option_none_when_frequency_not_found() {
        let frequency =
            super::Parser::parse_frequency("意味: 中毒性のある、依存性のある\n".to_string());
        assert_eq!(frequency, None);
    }

    #[test]
    fn it_parse_collocations() {
        let collocations = super::Parser::parse_collations(RESPONDED_MESSAGE.to_string()).unwrap();
        assert_eq!(collocations.len(), 5);
        assert_eq!(collocations[0], "- addictive behavior");
        assert_eq!(collocations[1], "- addictive personality");
        assert_eq!(collocations[2], "- addictive substances");
        assert_eq!(collocations[3], "- addictive tendencies");
        assert_eq!(collocations[4], "- addictive habits");
    }
}
