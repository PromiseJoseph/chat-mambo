use crate::types::{Client, Message};
use std::println;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast;

pub async fn handle_stream(
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
    sender: broadcast::Sender<Message>,
    mut receiver: broadcast::Receiver<Message>,
    client: Client,
) {
    let mut buffer = [0; 1024];
    loop {
        let peer_addr = &client.peer_addr;
        let username = &client.username;
        let client_id = &client.client_id;

        println!("Waiting for messages: {}", peer_addr); //for testing purposes, to be removed 
        tokio::select! {
               message = receiver.recv() => {
                   match message {
                   Ok(msg) => {
                    let Message(sender_id, msg_content)= msg;

                    if sender_id.as_deref() != Some(client_id) {
                       if writer.write_all(msg_content.as_bytes()).await.is_ok() {
                           println!("Sent message to client {peer_addr}: {msg_content}");
                       } else {
                           eprintln!("Failed to send message to client {peer_addr}");
                       }
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
                       println!("Client {peer_addr} disconnected");
                       let disconnect_message = format!("[Alert]{} left the chat!", username);
                       if let Err(e) = sender.send(Message(Some(client_id.clone()), disconnect_message)) {
                            eprintln!("Failed to broadcast disconnection message: {}", e);
                        }
                       break;
                   }

                   Ok(bytes) => {

                       let message_str = String::from_utf8_lossy(&buffer[..bytes]);
                       println!("Received {bytes} bytes from client {peer_addr}: \"{message_str}\"");

                       if let Err(e) = sender.send(Message(Some(client_id.clone()), format!("[{}] {}", username, message_str))) {
                           eprintln!("Failed to broadcast message from {username}.. error : {e} ")
                       }
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
