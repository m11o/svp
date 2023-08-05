use cynic::{Id, MutationBuilder, Operation, http::SurfExt};
use std::env;
use crate::vocabulary_stocker::github::client;

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

impl client::Client<createIssue, createIssueField> for CreateIssueClient {
    fn build_query(&self) -> Operation<createIssue, createIssueField> {
        use MutationBuilder;

        let repositoryId: Id = Id::new(env::var("VOCABULARY_REPOSITORY_ID").unwrap());
        let projectId: Id = Id::new(env::var("VOCABULARY_PROJECT_ID").unwrap());

        createIssue::build(
            createIssueField {
                body: self.body.clone(),
                title: self.title.clone(),
                repositoryId,
                projectIds: vec![projectId]
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
    variables = "createIssueField"
)]
struct createIssue {
    #[arguments(
        input: {
            body: $body,
            projectIds: $projectIds,
            repositoryId: $repositoryId,
            title: $title,
        }
    )]
    pub CreateIssue: Option<CreateIssuePayload>,
}

#[derive(cynic::QueryVariables, Debug)]
struct createIssueField {
    pub body: String,
    pub projectIds: Vec<Id>,
    pub repositoryId: Id,
    pub title: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
struct CreateIssuePayload {
    pub issue: Option<Issue>
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
struct Issue {
    pub id: cynic::Id
}
