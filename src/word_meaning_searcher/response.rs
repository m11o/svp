use crate::word_meaning_searcher::parser::Parser;

pub struct MeaningResponse {
    pub meaning: String,
    pub examples: Vec<String>,
    pub collocations: Vec<String>,
    pub frequency: String,
}

impl MeaningResponse {
    pub fn new(meaning: String, examples: Vec<String>, collocations: Vec<String>, frequency: String) -> Self {
        Self {
            meaning,
            examples,
            collocations,
            frequency
        }
    }

    pub fn parse(responded_message: String) -> MeaningResponse {
        let meaning = Parser::parse_meaning(responded_message.clone()).unwrap();
        let examples = Parser::parse_examples(responded_message.clone()).unwrap();
        let frequency = Parser::parse_frequency(responded_message.clone()).unwrap();
        let collocations = Parser::parse_collations(responded_message.clone()).unwrap();

        MeaningResponse::new(
            meaning,
            examples,
            collocations,
            frequency
        )
    }
}