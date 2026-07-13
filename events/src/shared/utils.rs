use axum::http::HeaderValue;

pub fn header_to_string(header: &HeaderValue) -> String {
    header
        .to_str()
        .expect("Unable to parse header value")
        .to_string()
}
