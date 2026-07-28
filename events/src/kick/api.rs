use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    data: ApiKey,
}
#[derive(Serialize, Deserialize)]
struct ApiKey {
    public_key: String,
}

pub async fn retrieve_public_key(uri: String) -> String {
    let res = reqwest::get(uri)
        .await
        .expect("Error Retrieving Kick Public Key");
    let response: ApiResponse = res.json::<ApiResponse>().await.expect("Wrong Api Response");
    response.data.public_key
}

#[cfg(test)]
mod test {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_retrieve_public_key_success() {
        let mock_server = MockServer::start().await;
        let body = ApiResponse {
            data: ApiKey {
                public_key: "test key".to_string(),
            },
        };
        Mock::given(method("GET"))
            .and(path("/public_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&mock_server)
            .await;
        let url = format!("{}/public_key", mock_server.uri());
        println!("{}", url);
        let res = retrieve_public_key(url).await;
        assert_eq!(res, "test key");
    }
}
