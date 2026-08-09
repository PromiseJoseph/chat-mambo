use crate::stream;
use crate::types::{Client, Message};
use crate::username_handler;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::broadcast::Sender;

pub async fn handle_connection(
    mut stream: TcpStream,
    sender: Sender<Message>,
    peer_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("New client connected: {}", peer_addr);
    let username = username_handler::request_client_username(&mut stream, sender.clone()).await?;

    let client_id = format!("{}_{}", peer_addr, username);

    let receiver = sender.subscribe();

    let (reader, writer) = stream.into_split();

    let client = Client {
        username,
        client_id,
        peer_addr,
    };

    stream::handle_stream(reader, writer, sender, receiver, client).await;

    Ok(())
}
