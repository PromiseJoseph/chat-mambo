use tokio::net::TcpStream;
mod stream_handler;
use stream_handler::handle_stream;

const SERVER_ADDR: &str = "127.0.0.1:8080"; // Address of the EchoMambo-client server

#[tokio::main]
async fn main() {
    let arg = std::env::args().nth(1);
    let server_addr = arg.as_deref().unwrap_or(SERVER_ADDR);

    // Connect to the server
    let stream = connect_to_server(&server_addr).await;

    println!(
        "Connected to echo server at {}",
        stream.peer_addr().unwrap()
    );

    let _ = handle_stream(stream).await;
}

//=== Function to connect to the server ===//
async fn connect_to_server(server_addr: &str) -> TcpStream {
    match TcpStream::connect(server_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            std::process::exit(1);
        }
    }
}
