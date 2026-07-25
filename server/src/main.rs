use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_mambo_addr: [SocketAddr; 2] = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];

    println!("Starting ChatMambo server ");

    let listener = bind_addresses(&chat_mambo_addr).await?;

    println!("Server listening on: {:?}", listener.local_addr().unwrap());

    let (sender, _): (broadcast::Sender<String>, broadcast::Receiver<String>) =
        broadcast::channel::<String>(5);

    loop {
        let (stream, peer_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let receiver = sender.subscribe();
        let sender = sender.clone();

        let (reader, writer) = stream.into_split();

        tokio::spawn(async move {
            handle_connection(reader, writer, sender, receiver, &peer_addr).await
        });
    }
    Ok(())
}

async fn handle_connection(
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
    sender: broadcast::Sender<String>,
    mut receiver: broadcast::Receiver<String>,
    peer_addr: &SocketAddr,
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

                       sender.send(message_str.to_string()).unwrap();

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

async fn bind_addresses(addresses: &[SocketAddr]) -> Result<TcpListener, std::io::Error> {
    for address in addresses {
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
