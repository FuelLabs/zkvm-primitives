mod counter_contract;
mod opcodes;
mod utils;

#[tokio::main]
pub async fn main() {
    let refresh_build = std::env::var("REFRESH_BUILD").unwrap_or_else(|_| "false".to_string());
    if refresh_build == "false" {
        return;
    }

    counter_contract::generate_fixture()
        .await
        .expect("Failed to generate fixture");
    opcodes::generate_fixture()
        .await
        .expect("Failed to generate fixture");
}
