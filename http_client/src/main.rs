// This task involves creating an HTTP client that fetches data from a web API asynchronously using the reqwest library and Tokio runtime.

use reqwest::Error;

// Asynchronous function to fetch data from a URL
async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?; // Make an HTTP GET request
    let body = response.text().await?;      // Extract the response body
    Ok(body)
}

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/posts/"; // Example API endpoint

    match fetch_url(url).await {
        Ok(data) => println!("Response: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}

