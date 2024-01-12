use base64::prelude::*;
use dotenv::dotenv;
use generative_ai::GoogleGenerativeAI;
use generative_ai::{GenerativeContentBlob, InlineDataPart, Part};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::{env, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!("{}", std::env::current_dir().unwrap().display());

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found in .env");

    let prompt = vec![
        Part::from("What do you see?"),
        file_to_blob_part("./test_files/cat.jpg", "image/jpg").unwrap(),
        file_to_blob_part("./test_files/dog.jpg", "image/jpg").unwrap(),
    ];

    let gen_ai = GoogleGenerativeAI::new(&api_key);

    let model = gen_ai.get_model("gemini-pro-vision");

    let result = model.generate_content(prompt).await?;

    println!("{:?}", result);

    Ok(())
}

fn file_to_blob_part(file_path: &str, mime: &str) -> io::Result<Part> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let base64_str = BASE64_STANDARD.encode(buffer);

    Ok(Part::InlineData(InlineDataPart {
        text: None,
        inline_data: GenerativeContentBlob {
            data: base64_str,
            mime_type: mime.to_string(),
        },
    }))
}
