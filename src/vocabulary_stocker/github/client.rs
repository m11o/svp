use cynic::{GraphQlResponse, Operation};
use std::env;
use std::marker::Send;
use async_trait::async_trait;

#[async_trait]
pub trait Client<QueryStruct: for<'de> cynic::serde::Deserialize<'de> + 'static, VariableStruct: cynic::serde::Serialize + Send> {
    async fn exec(&self) -> GraphQlResponse<QueryStruct> {
        use cynic::http::SurfExt;

        let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env var must be set");

        surf::post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "m11o")
            .run_graphql(self.build_query())
            .await
            .unwrap()
    }
    fn build_query(&self) -> Operation<QueryStruct, VariableStruct>;
}
