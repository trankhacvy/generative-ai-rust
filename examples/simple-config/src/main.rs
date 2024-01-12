use dotenv::dotenv;
use generative_ai::requests::{GenerationConfig, ModelConfig};
use generative_ai::GoogleGenerativeAI;
use std::error::Error;
use std::{env, vec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found in .env");

    let gen_ai = GoogleGenerativeAI::new(&api_key);

    let config = ModelConfig {
        generation_config: Some(GenerationConfig {
            stop_sequences: Some(vec![String::from("seven")]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut model = gen_ai.get_model("gemini-pro");
    let model = model.with_config(config);

    let prompt = "One, two, three, ";

    let result = model.generate_content(prompt).await?;

    println!("{:#?}", result);

    Ok(())
}
