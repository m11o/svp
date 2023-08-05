mod github;

use github::client::Client;
use github::create_issue::CreateIssueClient;

use cynic::Id;

const ISSUE_BODY_TEMPLATE: string = "\
  # 意味\n\
  <details>\n\
  <summary>答えを見る</summary>\n\
  \n\
  ```\n\
  {meaning}\n\
  ```\n\
  \n\
  </details>\n\
  \n\
  # コロケーション\n\
  \n\
  {collocations}\n\
  \n\
  # 例文\n\
  \n\
  {examples}\n\
  \n\
  # イメージ\n\
  \n\
";

pub struct VocabularyStocker<'a> {
    word: &'a String,
    meaning: String,
    examples: Vec<String>,
    collocations: Vec<String>,
    frequency: String,
}

impl<'a> VocabularyStocker<'a> {
    pub fn new(
        word: &'a String,
        meaning: String,
        examples: Vec<String>,
        collocations: Vec<String>,
        frequency: String,
    ) -> Self {
        Self {
            word,
            meaning,
            examples,
            collocations,
            frequency,
        }
    }

    pub async fn stock(&self) {
        println!("saving...");
        println!("word: {}", self.word);
        println!("meaning: {}", self.meaning);
        println!("examples: {:?}", self.examples);
        println!("collocations: {:?}", self.collocations);
        println!("frequency: {}", self.frequency);

        let client = CreateIssueClient::new(self.word.clone(), self.build_issue_body());
        // TODO: 登録済みかどうかを確認
        let response = client.exec().await;
        let issue_id: Id = response
            .data
            .unwrap()
            .create_issue
            .expect("登録に失敗しました")
            .issue
            .expect("登録に失敗しました")
            .id;
        println!("{:?}", issue_id)
    }

    fn build_issue_body(&self) -> String {
        format!(
            ISSUE_BODY_TEMPLATE,
            meaning = self.meaning,
            collocations = self.collocations.join("\n"),
            examples = self.examples.join("\n")
        )
    }
}
