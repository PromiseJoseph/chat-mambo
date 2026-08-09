use std::net::SocketAddr;
use tokio::sync::broadcast;
mod listener;
mod stream;
mod types;
use types::Message;
mod connection;
mod username_handler;
use connection::handle_connection;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_mambo_addr: [SocketAddr; 2] = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];

    println!("Starting ChatMambo server ");
    let arg = std::env::args().nth(1);
    let custom_addr = arg.as_deref();
    let listener = listener::bind_addresses(custom_addr, &chat_mambo_addr).await?;
    println!("Server listening on: {:?}", listener.local_addr().unwrap());

    let (sender, _): (broadcast::Sender<Message>, broadcast::Receiver<Message>) =
        broadcast::channel::<Message>(20);

    loop {
        let (stream, peer_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let sender = sender.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, sender, peer_addr).await {
                eprintln!("Connection {peer_addr} failed: {e}");
            }
        });
    }
}
