use crate::discord;

const DISCORD_API: &str = "https://discord.com/api/v10";

pub async fn post_message(content: &str) {
    println!("post message");
    let channel_id = std::env::var("CHANNEL_ID").unwrap();

    let url = format!("{DISCORD_API}/channels/{channel_id}/messages");
    let discord_token = std::env::var("DISCORD_TOKEN").expect("missing");

    let client = reqwest::Client::new();
    let request = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {}", discord_token))
        .json(&discord::models::Message {
            content: content.to_string(),
        })
        .build()
        .expect("failed to send");
    let res = reqwest::Client::execute(&client, request)
        .await
        .expect("bad request");
    println!("{:?}", res);
    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await.expect("bad"));
}
