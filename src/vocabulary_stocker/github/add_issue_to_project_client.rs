use cynic::{Id, Operation};
use std::env;
use crate::vocabulary_stocker::github::client::Client;

pub struct AddIssueToProjectClient {
    issue_id: Id,
}

impl AddIssueToProjectClient {
    pub fn new(issue_id: Id) -> AddIssueToProjectClient {
        Self {
            issue_id,
        }
    }
}

impl Client<AddIssueToProject, AddIssueToProjectField> for AddIssueToProjectClient {
    fn build_query(&self) -> Operation<AddIssueToProject, AddIssueToProjectField> {
        use cynic::MutationBuilder;

        let project_id = Id::new(env::var("VOCABULARY_PROJECT_ID").unwrap());

        AddIssueToProject::build(
            AddIssueToProjectField {
                contentId: self.issue_id.clone(),
                projectId: project_id
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
    variables = "AddIssueToProjectField"
)]
pub struct AddIssueToProject {
    #[arguments(
        input: {
            contentId: $contentId,
            projectId: $projectId,
        }
    )]
    pub add_project_v2_item_by_id: Option<AddProjectV2ItemByIdPayload>
}

#[derive(cynic::QueryVariables, Debug)]
#[cynic(rename_all="camelCase")]
pub struct AddIssueToProjectField {
    pub contentId: Id,
    pub projectId: Id
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
pub struct AddProjectV2ItemByIdPayload {
    pub item: Option<ProjectV2Item>
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
pub struct ProjectV2Item {
    pub id: Id
}
