use dotenv::dotenv;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use rig::loaders::FileLoader;

use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok(); // Load .env file into the environment
    let openai = openai::Client::from_env();

    FileLoader::with_glob("docs/*.toml")?
        .read()
        .into_iter()
        .for_each(|result| match result {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Error reading file: {}", e),
        });

    // Write
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    write!(tmpfile, "Hello World!").unwrap();

    Ok(())
}
