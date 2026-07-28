use crate::discord;
use std::fmt;
use tracing::debug;

#[derive(Debug)]
pub enum APIError {
    RequestFailed,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            APIError::RequestFailed => write!(f, "Request Failed"),
        }
    }
}

pub async fn post_message(
    url: &str,
    client: &reqwest::Client,
    discord_token: &str,
    content: &str,
) -> Result<(), APIError> {
    let res = client
        .post(url)
        .header("Authorization", format!("Bot {}", discord_token))
        .json(&discord::models::Message {
            content: content.to_string(),
        })
        .send()
        .await
        .map_err(|_e| APIError::RequestFailed)?;

    debug!("Status: {}", res.status());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_post_message() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/channels/123/messages"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let url = format!("{}/channels/123/messages", mock_server.uri());
        println!("{}", url);
        let client = reqwest::Client::new();
        let res = post_message(&url, &client, "123", "content").await;
        assert!(res.is_ok());
    }
}
