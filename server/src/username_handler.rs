use crate::types::Message;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
pub async fn request_client_username(
    stream: &mut TcpStream,
    sender: broadcast::Sender<Message>,
) -> Result<String, std::io::Error> {
    let mut buffer = [0; 1024];
    let (mut reader, mut writer) = stream.split();
    // Request username from client
    let request_message = "Please enter your username:";
    writer.write_all(request_message.as_bytes()).await?;

    // Read the username from the client

    let bytes = reader.read(&mut buffer).await?;
    if bytes == 0 {
        return Err(Error::new(
            ErrorKind::ConnectionReset,
            "Client disconnected",
        ));
    }
    let username = String::from_utf8_lossy(&buffer[..bytes]).trim().to_string();

    let _ = sender.send(Message(None, format!("{} has joined the chat.", username)));

    Ok(username)
}
