use async_openai::{
    config::OpenAIConfig, 
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs}, 
    Client
};
use dotenv::dotenv;
use std::env;
use tokio;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>{
    // println!("Hello, world!");
    dotenv().ok();

    // get env vars
    let api_key = env::var("OPENAI_API_KEY")
                    .expect("OPENAI_API_KEY environment variable is not set.");
    let base_url = env::var("OPENAI_API_BASE")
    .expect("OPENAI_API_BASE environment variable is not set");
    print!("{}", base_url);


    let config = OpenAIConfig::new()
    .with_api_base(base_url)
    .with_api_key(api_key);

    let client = Client::with_config(config);

    // first query to GH copliot
    let prompt = "Can you tell me a joke";

    let user_msg = ChatCompletionRequestUserMessageArgs::default()
    .content(prompt)
    .build()?
    .into();

    println!("{:?}", user_msg);

    // Create a chat completion request
    let request = CreateChatCompletionRequestArgs::default()
    // .model("deepseek-ai/DeepSeek-V3")
    // .model("copilot-chat")
    .model("openai/gpt-4o")
    .messages([user_msg])
    .max_tokens(150_u32) // Limits the length of the explanation
    .temperature(0.6) // Controls creativity
    .presence_penalty(0.5) // Encourages new concepts
    .frequency_penalty(0.2) // // Discourages repetition
    .build()?;

    let response = client.chat()
    .create(request)
    .await?;

    println!("Query: {}", prompt);
    println!(
    "Answer: {:?}",
    response
    // response.choices[0].message.content.as_deref().unwrap_or_default()
    );
        
    Ok(())
}
