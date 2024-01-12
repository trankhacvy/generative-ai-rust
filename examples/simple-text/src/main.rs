use dotenv::dotenv;
use generative_ai::GoogleGenerativeAI;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found in .env");

    let gen_ai = GoogleGenerativeAI::new(&api_key);

    let model = gen_ai.get_model("gemini-pro");

    let prompt = "Who are you?";

    let count_token_res = model.count_tokens(prompt).await?;

    println!("{:?}", count_token_res);

    let result = model.generate_content(prompt).await?;

    println!("{:#?}", result);

    Ok(())
}
