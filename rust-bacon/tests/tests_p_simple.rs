use reqwest::Client;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::test]
async fn test_simple() -> Result<()> {
    let client = Client::new(); // Create a new client

    let response = client.get("https://httpbin.org/get").send().await?; // Send a GET request to the URL

    assert_eq!(response.status(), 200); // Check that the response status is 200
    assert_eq!(
        response.text().await?,
        "{\"headers\":{\"Accept\":\"*/*\",\"User-Agent\":\"reqwest\"}}"
    );

    Ok(())
}

#[tokio::test]
async fn test_simple_with_headers() -> Result<()> {
    let client = Client::new(); // Create a new client

    let response = client
        .get("https://v1.hitokoto.cn/")
        .header("Accept", "application/json")
        .send()
        .await?; // Send a GET request to the URL with a custom header

    assert_eq!(response.status(), 200); // Check that the response status is 200
    println!("{}", response.text().await?); // Print the response body
    Ok(())
}

