use cynic::{Id, Operation};
use std::env;
use crate::vocabulary_stocker::github::client::Client;

pub struct UpdateProjectItemFieldClient<'a> {
    project_item_id: &'a Id,
    field_id: Id,
    select_option_id: String
}

impl<'b> UpdateProjectItemFieldClient<'b> {
    pub fn new(project_item_id: &'b Id, field_id: Id, select_option_id: String) -> UpdateProjectItemFieldClient {
        Self {
            project_item_id,
            field_id,
            select_option_id
        }
    }
}

impl<'b> Client<UpdateProjectItemField, UpdateProjectItemFieldArgument> for UpdateProjectItemFieldClient<'b> {
    fn build_query(&self) -> Operation<UpdateProjectItemField, UpdateProjectItemFieldArgument> {
        use cynic::MutationBuilder;

        let project_id = Id::new(env::var("VOCABULARY_PROJECT_ID").unwrap());

        UpdateProjectItemField::build(
            UpdateProjectItemFieldArgument {
                field_id: self.field_id.clone(),
                item_id: self.project_item_id.clone(),
                project_id,
                select_option_id: self.select_option_id.clone()
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
    variables = "UpdateProjectItemFieldArgument"
)]
pub struct UpdateProjectItemField {
    #[arguments(
        input: {
            fieldId: $field_id,
            itemId: $item_id,
            projectId: $project_id,
            value: {
                singleSelectOptionId: $select_option_id
            }
        }
    )]
    pub update_project_v2_item_field_value: Option<UpdateProjectV2ItemFieldValuePayload>
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateProjectItemFieldArgument {
    pub field_id: Id,
    pub item_id: Id,
    pub project_id: Id,
    pub select_option_id: String
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
pub struct UpdateProjectV2ItemFieldValuePayload {
    pub project_v2_item: Option<ProjectV2Item>
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "github")]
pub struct ProjectV2Item {
    pub id: Id
}
