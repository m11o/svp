use cynic::Operation;
use std::env;

pub trait Client<QueryStruct, VariableStruct> {
    fn exec(&self) {
        use cynic::http::SurfExt;

        let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env var must be set");

        surf::post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "m11o")
            .run_graphql(self.query)
            .unwrap()
    }
    fn build_query(&self) -> Operation<QueryStruct, VariableStruct>;
}
