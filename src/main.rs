mod word_meaning_searcher;

use std::env;
use dotenv::dotenv;

use word_meaning_searcher::WordMeaningSearcher;

#[async_std::main]
async fn main() {
    dotenv().ok(); // Load .env file to use OPENAI_API_KEY

    let word: String = match env::args().nth(1) {
        Some(word) => word,
        None => panic!("Please provide a word to search")
    };
    println!("{:?}", word);

    let searcher = WordMeaningSearcher::new(word);
    let response = searcher.fetch_meaning().await;
    println!("meaning: {:?}", response.meaning);
    println!("examples: {:?}", response.examples);
    println!("collocations: {:?}", response.collocations);
    println!("frequency: {:?}", response.frequency);
}
