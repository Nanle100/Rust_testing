use reqwest::Error;
use std::fs::File;
use std::io::{self, Write};
use tokio::join;
use futures_util::StreamExt;


// Function to download a single file from a URL and save it locally
async fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await?;

    // Open a file to save the downloaded content
    let mut file = File::create(filename)?;

    // Read the response body as a stream of bytes
    let mut stream = response.bytes_stream();

    println!("Downloading: {}", filename);

    // Process the response stream in chunks
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?; // Handle errors during streaming
        file.write_all(&chunk)?; // Write the chunk to the file
    }

    println!("Downloaded: {}", filename);

    Ok(())
}

#[tokio::main]
async fn main() {
    // Define the URLs to download
    let urls = vec![
        ("https://jsonplaceholder.typicode.com/posts/1", "file1.txt"),
        ("https://jsonplaceholder.typicode.com/posts/2", "file2.txt"),
    ];

    // Create tasks for downloading files concurrently
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|(url, filename)| download_file(url, filename))
        .collect();

    // Run all tasks concurrently
    join!(tasks);
}
