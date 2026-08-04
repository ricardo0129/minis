use axum::http::HeaderValue;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tracing::info;

pub fn header_to_string(header: &HeaderValue) -> String {
    header
        .to_str()
        .expect("Unable to parse header value")
        .to_string()
}

pub async fn init_listener() -> Result<TcpListener, tokio::io::Error> {
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let addr = SocketAddr::new(ip, 5000);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", listener.local_addr()?);
    Ok(listener)
}
