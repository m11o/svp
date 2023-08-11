pub mod client;
pub mod create_issue;
pub mod add_issue_to_project_client;
pub mod update_project_item_field;

pub mod r#const {
    pub const PROJECT_STATUS_OPTION_LEARNING: &str = "f75ad846";
    pub const PROJECT_STATUS_OPTION_LEARNED: &str = "47fc9ee4";
    pub const PROJECT_STATUS_OPTION_IMMEDIATELY_USING: &str = "98236657";
    pub const PROJECT_STATUS_OPTION_AVAILABLE_TO_TALK: &str = "d30568bb";

    pub const PROJECT_FREQUENCY_OPTION_LOW: &str = "3d8bd700";
    pub const PROJECT_FREQUENCY_OPTION_MIDDLE: &str = "12a7adb5";
    pub const PROJECT_FREQUENCY_OPTION_HIGH: &str = "c6fe92ca";
    pub const PROJECT_FREQUENCY_OPTION_VERY_HIGH: &str = "2d405bbb";
}
