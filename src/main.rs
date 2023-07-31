mod word_meaning_searcher;
mod vocabulary_stocker;

use std::env;
use dotenv::dotenv;

use word_meaning_searcher::WordMeaningSearcher;
use vocabulary_stocker::VocabularyStocker;

#[async_std::main]
async fn main() {
    dotenv().ok(); // Load .env file to use OPENAI_API_KEY

    let word: String = match env::args().nth(1) {
        Some(word) => word,
        None => panic!("Please provide a word to search")
    };
    println!("{:?}", word);

    let searcher = WordMeaningSearcher::new(&word);
    let response = searcher.fetch_meaning().await;

    let stocker = VocabularyStocker::new(
        &word,
        response.meaning,
        response.examples,
        response.collocations,
        response.frequency
    );
    stocker.stock();
}
