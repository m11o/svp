use crate::word_meaning_searcher::parser::{
    Parser,
    frequency::Frequency
};

pub struct MeaningResponse {
    pub meaning: String,
    pub examples: Vec<String>,
    pub collocations: Vec<String>,
    pub frequency: Frequency,
}

impl MeaningResponse {
    pub fn new(
        meaning: String,
        examples: Vec<String>,
        collocations: Vec<String>,
        frequency: Frequency,
    ) -> Self {
        Self {
            meaning,
            examples,
            collocations,
            frequency,
        }
    }

    pub fn parse(responded_message: String) -> MeaningResponse {
        let meaning = Parser::parse_meaning(responded_message.clone()).unwrap_or(String::from(""));
        let examples = Parser::parse_examples(responded_message.clone()).unwrap_or(vec![]);
        let frequency =
            Parser::parse_frequency(responded_message.clone()).unwrap_or(Frequency::Low);
        let collocations = Parser::parse_collations(responded_message.clone()).unwrap_or(vec![]);

        MeaningResponse::new(meaning, examples, collocations, frequency)
    }
}
