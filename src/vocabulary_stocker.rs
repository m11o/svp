mod github;

pub struct VocabularyStocker <'a> {
    word: &'a String,
    meaning: String,
    examples: Vec<String>,
    collocations: Vec<String>,
    frequency: String,
}

impl <'a> VocabularyStocker <'a> {
    pub fn new(word: &'a String, meaning: String, examples: Vec<String>, collocations: Vec<String>, frequency: String) -> Self {
        Self {
            word,
            meaning,
            examples,
            collocations,
            frequency
        }
    }

    pub fn stock(&self) {
        println!("saving...");
        println!("word: {}", self.word);
        println!("meaning: {}", self.meaning);
        println!("examples: {:?}", self.examples);
        println!("collocations: {:?}", self.collocations);
        println!("frequency: {}", self.frequency);
    }
}