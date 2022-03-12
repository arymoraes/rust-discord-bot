use reqwest::Error;

pub async fn bork() -> Result<String, Error> {
    let res = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .text()
        .await?;

    let json: serde_json::Value =
        serde_json::from_str(&res).expect("JSON was not well-formatted");

    Ok(json.get("message").unwrap().to_string())
}
