use std::net::SocketAddr;
use tokio::io::{Error, ErrorKind};
use tokio::net::TcpListener;

pub async fn bind_addresses(
    custom_address: Option<&str>,
    default_addresses: &[SocketAddr],
) -> Result<TcpListener, std::io::Error> {
    if let Some(addr) = custom_address {
        if let Ok(listener) = TcpListener::bind(addr).await {
            return Ok(listener);
        }
    }
    for address in default_addresses {
        match TcpListener::bind(address).await {
            Ok(listener) => return Ok(listener),
            Err(e) => eprintln!("Failed to bind to {}: {}", address, e),
        }
    }
    Err(Error::new(
        ErrorKind::AddrNotAvailable,
        "Failed to bind to any of the provided addresses",
    ))
}
