use std::net::SocketAddr;
use tokio::sync::broadcast;
mod listener;
mod stream;
mod username_handler;
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

    let (sender, _): (broadcast::Sender<String>, broadcast::Receiver<String>) =
        broadcast::channel::<String>(5);

    loop {
        let (mut stream, peer_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let receiver = sender.subscribe();
        let sender = sender.clone();
        let username = username_handler::request_client_username(&mut stream).await?;

        let (reader, writer) = stream.into_split();

        tokio::spawn(async move {
            stream::handle_stream(reader, writer, sender, receiver, peer_addr, username).await
        });
    }
    Ok(())
}
