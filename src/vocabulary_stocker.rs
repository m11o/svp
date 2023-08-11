mod github;

use github::client::Client;
use github::create_issue::CreateIssueClient;

use cynic::Id;
use crate::vocabulary_stocker::github::{
    add_issue_to_project_client::AddIssueToProjectClient,
    update_project_item_field::UpdateProjectItemFieldClient,
    constants
};

use std::env;

use crate::word_meaning_searcher::parser::frequency::Frequency;

pub struct VocabularyStocker<'a> {
    word: &'a String,
    meaning: String,
    examples: Vec<String>,
    collocations: Vec<String>,
    frequency: Frequency,
}

impl<'a> VocabularyStocker<'a> {
    pub fn new(
        word: &'a String,
        meaning: String,
        examples: Vec<String>,
        collocations: Vec<String>,
        frequency: Frequency,
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

        // TODO: 登録済みかどうかを確認
        let issue_id: Id = self.execute_issue_creation().await;
        let project_item_id: Id = self.execute_project_item_addition(issue_id).await;
        let select_option_id = self.frequency2select_option_id();

        self.execute_to_update_frequency(project_item_id, select_option_id).await;
    }

    async fn execute_issue_creation(&self) -> Id {
        let client = CreateIssueClient::new(self.word.clone(), self.build_issue_body());
        let response = client.exec().await;
        response
            .data
            .unwrap()
            .create_issue
            .expect("登録に失敗しました")
            .issue
            .expect("登録に失敗しました")
            .id
    }

    async fn execute_project_item_addition(&self, issue_id: Id) -> Id {
        let client = AddIssueToProjectClient::new(issue_id);
        let response = client.exec().await;
        response
            .data
            .unwrap()
            .add_project_v2_item_by_id
            .expect("登録に失敗しました")
            .item
            .expect("登録に失敗しました")
            .id
    }

    async fn execute_to_update_frequency(&self, item_id: Id, select_option_id: String) {
        let field_id = Id::new(env::var("VOCABULARY_STATUS_FIELD_ID").unwrap());
        let client = UpdateProjectItemFieldClient::new(
            item_id,
            field_id,
            select_option_id
        );
        client.exec().await;
    }

    fn frequency2select_option_id(&self) -> String {
        match self.frequency {
            Frequency::VeryHigh => String::from(constants::PROJECT_FREQUENCY_OPTION_VERY_HIGH),
            Frequency::High => String::from(constants::PROJECT_FREQUENCY_OPTION_HIGH),
            Frequency::Middle => String::from(constants::PROJECT_FREQUENCY_OPTION_MIDDLE),
            Frequency::Low => String::from(constants::PROJECT_FREQUENCY_OPTION_LOW)
        }
    }

    fn build_issue_body(&self) -> String {
        format!("\
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
            ",
            meaning = self.meaning,
            collocations = self.collocations.join("\n"),
            examples = self.examples.join("\n")
        )
    }
}
