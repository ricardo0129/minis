use crate::kick::constants;
use serde_json::Value;
pub async fn retrieve_public_key() -> String {
    let url = format!("{}/public-key", constants::KICK_API);
    let res = reqwest::get(url)
        .await
        .expect("Error Retrieving Kick Public Key");
    let json: Value = res.json().await.expect("Unable to get Json");
    json["data"]["public_key"].to_string()
}
