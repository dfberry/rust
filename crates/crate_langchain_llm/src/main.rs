use langchain_rust::{
    language_models::llm::LLM,
    llm::openai::{AzureConfig, OpenAI},
};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let azure_config = AzureConfig::default()
        .with_api_key(&env::var("AZURE_OPENAI_API_KEY").expect("AZURE_OPENAI_API_KEY must be set"))
        .with_api_base(&env::var("AZURE_OPENAI_API_BASE").expect("AZURE_OPENAI_API_BASE must be set"))
        .with_api_version(&env::var("AZURE_OPENAI_API_VERSION").unwrap_or_else(|_| "2024-02-15-preview".to_string()))
        .with_deployment_id(&env::var("AZURE_OPENAI_API_DEPLOYMENT_NAME").expect("AZURE_OPENAI_API_DEPLOYMENT_NAME must be set"));

    let open_ai = OpenAI::new(azure_config);
    let response = open_ai.invoke("Why is the sky blue?").await.unwrap();
    println!("{}", response);
}