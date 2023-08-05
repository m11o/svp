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
        let project_id: Id = Id::new(env::var("VOCABULARY_PROJECT_ID").unwrap());

        CreateIssue::build(
            CreateIssueField {
                body: self.body.clone(),
                title: self.title.clone(),
                repository_id,
                project_ids: Some(vec![project_id])
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
            projectIds: $project_ids,
            repositoryId: $repository_id,
            title: $title,
        }
    )]
    pub create_issue: Option<CreateIssuePayload>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateIssueField {
    pub body: String,
    pub project_ids: Option<Vec<Id>>,
    pub repository_id: Id,
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
