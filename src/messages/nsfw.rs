use reqwest::Error;

pub async fn nsfw() -> Result<String, Error> {
    let res = reqwest::get("http://api.nekos.fun:8080/api/gasm")
        .await?
        .text()
        .await?;

    let json: serde_json::Value =
        serde_json::from_str(&res).expect("JSON was not well-formatted");

    Ok(json.get("image").unwrap().to_string())
}
