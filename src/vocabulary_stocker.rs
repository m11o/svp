mod github;

use github::create_issue::CreateIssueClient;
use github::client::Client;

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

    pub async fn stock(&self) {
        println!("saving...");
        println!("word: {}", self.word);
        println!("meaning: {}", self.meaning);
        println!("examples: {:?}", self.examples);
        println!("collocations: {:?}", self.collocations);
        println!("frequency: {}", self.frequency);

        let client = CreateIssueClient::new(
            self.word.clone(),
            self.build_issue_body()
        );
        client.exec().await;
    }

    fn build_issue_body(&self) -> String {
        format!("\
            # 意味
            <details>\n\
            <summary>答えを見る</summary>\n\
            {}\n\
            </details>\n\
            \n
            # コロケーション\n\
            {}\n\
            \n
            # 例文\n\
            {}\n\
            \n
            # イメージ\n\
            \n\
            ",
            self.meaning,
            self.collocations.join("\n"),
            self.examples.join("\n")
        )
    }
}