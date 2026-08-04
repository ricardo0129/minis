use rsa::RsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    data: ApiKey,
}
#[derive(Serialize, Deserialize)]
struct ApiKey {
    public_key: String,
}

pub trait FromPublicKeyPem {
    fn retrieve_public_key(key: &str) -> Self;
}

impl FromPublicKeyPem for RsaPublicKey {
    fn retrieve_public_key(key: &str) -> Self {
        RsaPublicKey::from_public_key_pem(key).expect("Bad Public Key")
    }
}

pub async fn retrieve_public_key<T: FromPublicKeyPem>(uri: String) -> T {
    let res = reqwest::get(uri)
        .await
        .expect("Error Retrieving Kick Public Key");
    let response: ApiResponse = res.json::<ApiResponse>().await.expect("Wrong Api Response");
    T::retrieve_public_key(&response.data.public_key)
}

#[cfg(test)]
mod test {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    impl FromPublicKeyPem for String {
        fn retrieve_public_key(key: &str) -> Self {
            key.to_string()
        }
    }

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
        let mock_url = format!("{}/public_key", mock_server.uri());
        let response_key: String = retrieve_public_key(mock_url).await;
        assert_eq!(response_key, "test key");
    }
}
