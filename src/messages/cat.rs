use reqwest::Error;

pub async fn meow() -> Result<String, Error> {
    let res = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await?
        .text()
        .await?;

    let json: serde_json::Value =
        serde_json::from_str(&res).expect("JSON was not well-formatted");

    Ok(json[0].get("url").unwrap().to_string())
}
