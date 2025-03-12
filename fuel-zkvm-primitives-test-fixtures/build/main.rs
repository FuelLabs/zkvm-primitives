mod block_execution_fixtures;

#[tokio::main]
pub async fn main() {
    let refresh_build = std::env::var("REFRESH_BUILD").unwrap_or_else(|_| "false".to_string());
    if refresh_build == "false" {
        return;
    }

    block_execution_fixtures::generate_fixture()
        .await
        .expect("Failed to generate fixture for block_execution_fixtures");
}
