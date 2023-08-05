use cynic::{Id, Operation};
use std::env;
use crate::vocabulary_stocker::github::client::Client;

pub struct CreateIssueClient {
    body: String,
    title: String
}

impl CreateIssueClient {
    pub fn new(title: String, body: String) -> CreateIssueClient {
        Self {
            title,
            body
        }
    }
}

impl Client<CreateIssue, CreateIssueField> for CreateIssueClient {
    fn build_query(&self) -> Operation<CreateIssue, CreateIssueField> {
        use cynic::MutationBuilder;

        let repository_id: Id = Id::new(env::var("VOCABULARY_REPOSITORY_ID").unwrap());

        CreateIssue::build(
            CreateIssueField {
                body: self.body.clone(),
                title: self.title.clone(),
                repositoryId: repository_id,
            }
        )
    }
}

#[cynic::schema("github")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "github",
    graphql_type = "Mutation",
    variables = "CreateIssueField"
)]
pub struct CreateIssue {
    #[arguments(
        input: {
            body: $body,
            repositoryId: $repositoryId,
            title: $title,
        }
    )]
    pub create_issue: Option<CreateIssuePayload>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateIssueField {
    pub body: String,
    pub repositoryId: Id,
    pub title: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
pub struct CreateIssuePayload {
    pub issue: Option<Issue>
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
pub struct Issue {
    pub id: Id
}
