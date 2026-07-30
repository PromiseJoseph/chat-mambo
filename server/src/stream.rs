use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast;

pub async fn handle_stream(
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
    sender: broadcast::Sender<String>,
    mut receiver: broadcast::Receiver<String>,
    peer_addr: SocketAddr,
    username: String,
) {
    let mut buffer = [0; 1024];
    loop {
        println!("Waiting for messages: {}", peer_addr); //for testing purposes, to be removed 
        tokio::select! {
               message = receiver.recv() => {
                   match message {
                   Ok(message) => {
                       println!("{peer_addr} received broadcast: {message}");
                       if writer.write_all(message.as_bytes()).await.is_ok() {
                           println!("Sent message to client {peer_addr}: {message}");
                       } else {
                           eprintln!("Failed to send message to client {peer_addr}");
                       }
                   }
                   Err(_) => {
                       eprintln!("Channel closed, no more messages to receive");
                       break;
                     }

                  }
               }

               result = reader.read(&mut buffer) => {
                   match result {
                   Ok(0) =>{
                       println!("Client disconnected: {:?}", peer_addr);
                       break;
                   }

                   Ok(bytes) => {

                       let message_str = String::from_utf8_lossy(&buffer[..bytes]);
                       println!("Received {bytes} bytes from client {peer_addr}: \"{message_str}\"");

                       sender.send(format!("[{}] {}", username, message_str)).unwrap();

                   }

                   Err(e) => {
                       eprintln!("Failed to read from socket: {}", e);
                       break;
                   }
               }
           }
        }
    }
}
