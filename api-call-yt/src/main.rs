use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    #[allow(dead_code)]
    #[serde(rename = "courierCode")]
    courier_code: String,
    #[allow(dead_code)]
    #[serde(rename = "courierNameCN")]
    courier_name_cn: String,
    #[allow(dead_code)]
    #[serde(rename = "courierNameEN")]
    courier_name_en: String,
    #[allow(dead_code)]
    #[serde(rename = "courierHomePage")]
    courier_home_page: String,
}

#[derive(Debug, Deserialize)]
struct CourierList {
    #[allow(dead_code)]
    code: String,
    #[allow(dead_code)]
    msg: String,
    #[allow(dead_code)]
    #[serde(rename = "traceId")]
    trace_id: String,
    #[allow(dead_code)]
    data: Vec<Data>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // https://docs.github.com/zh/rest/activity/starring?apiVersion=2022-11-28
    // let request_url = format!(
    //     "https://api.github.com/repos/{owner}/{repo}/stargazers/",
    //     owner = "rust-lang-nursery",
    //     repo = "rust-cookbook"
    // );
    let request_url = "https://api.track123.com/gateway/open-api/tk/v2/courier/list";
    println!("request_url: {}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header("Track123-Api-Secret", "de154caa82d14a1f9e4cbeefe8440aea")
        .header("Content-Type", "application/json")
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let res: CourierList = response.json().await?;
    println!("{:#?}", res);

    Ok(())
}
