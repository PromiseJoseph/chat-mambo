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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_bind_addresses() {
        let default_addresses: [SocketAddr; 2] = [
            SocketAddr::from(([127, 0, 0, 1], 4000)),
            SocketAddr::from(([127, 0, 0, 1], 4040)),
        ];

        // Test binding to a custom address that is likely available
        let custom_address = Some("127.0.0.1:4082");
        let listener = bind_addresses(custom_address, &default_addresses).await;
        assert!(listener.is_ok());
    }
}
