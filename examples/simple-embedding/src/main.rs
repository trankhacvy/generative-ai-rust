use dotenv::dotenv;
use generative_ai::GoogleGenerativeAI;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found in .env");

    let gen_ai = GoogleGenerativeAI::new(&api_key);

    let model = gen_ai.get_model("embedding-001");

    let prompt = "The quick brown fox jumps over the lazy dog.";

    let result = model.embed_content(prompt).await?;

    println!("{:?}", result);

    Ok(())
}
