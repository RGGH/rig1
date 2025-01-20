#![allow(unused)]
use dotenv::dotenv;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tempfile::tempdir;
use tokio::test;

use rig::loaders::FileLoader;
use rig::loaders::file::FileLoaderError;

use rig::{completion::Prompt, providers::openai};
use rig::{
    embeddings::EmbeddingsBuilder,
    providers::openai::{Client, TEXT_EMBEDDING_ADA_002},
    vector_store::VectorStoreIndex,
    Embed,
};


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok(); // Load .env file into the environment
    let openai = openai::Client::from_env();
    let model = openai.embedding_model(TEXT_EMBEDDING_ADA_002);

        // Load documents from files
    let glob_pattern = "docs/*.toml";
    let files = FileLoader::with_glob(glob_pattern)?;

        // Read the files and create Word struct for each document
    let documents: Vec<_> = files.read().into_iter().filter_map(|result| {
        match result {
            Ok(content) => {
                // Here you could map the content into your Word struct (e.g. parse TOML)
                     Some("some_unique_id".to_string())
            },
            Err(_) =>None // Skip files that couldn't be loaded
        }
    }).collect();

        // Create embeddings for the loaded documents
    let documents_with_embeddings = EmbeddingsBuilder::new(model.clone())
        .documents(documents)?
        .build()
        .await?;

    Ok(())
}

///
#[tokio::test]
async fn test_file_loader_with_glob() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the temporary directory using tempfile crate
    let dir = tempdir()?;
    let dir_path = dir.path().to_str().unwrap();

    // Create .toml files inside the temporary directory
    let file1_path = dir.path().join("test1.toml");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "[section]\nkey = \"value\"")?;

    let file2_path = dir.path().join("test2.toml");
    let mut file2 = File::create(&file2_path)?;
    writeln!(file2, "[section]\nkey = \"another_value\"")?;

    // Set up the dotenv (though it won't affect this test)
    dotenv().ok();

    // Create the glob pattern to match the .toml files
    let glob_pattern = format!("{}/{}.toml", dir_path, "*");

    // Use FileLoader to read files matching the glob pattern
    let files = FileLoader::with_glob(&glob_pattern)?;

    // let results = files.read();
    // Read the files and collect results
    let results: Vec<Result<String, FileLoaderError>> = files.read().into_iter().collect();

    // Check that we have the two files
    assert_eq!(results.len(), 2);

    // Validate content of each file
    results.into_iter().for_each(|result| match result {
        Ok(content) => {
            // Verify content, just checking the key-value pair
            assert!(content.contains("[section]"));
            assert!(content.contains("key ="));
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    });

    // Explicitly close the TempDir to ensure cleanup
    drop(file1);
    drop(file2);
    dir.close()?;

    Ok(())
}
