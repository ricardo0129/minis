use crate::kick::constants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    data: ApiKey,
}
#[derive(Serialize, Deserialize)]
struct ApiKey {
    public_key: String,
}

pub async fn retrieve_public_key() -> String {
    let url = format!("{}/public-key", constants::KICK_API);
    let res = reqwest::get(url)
        .await
        .expect("Error Retrieving Kick Public Key");
    let response: ApiResponse = res.json::<ApiResponse>().await.expect("Wrong Api Response");
    response.data.public_key
}
