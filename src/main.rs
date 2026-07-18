use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let chat_mambo_addr: [SocketAddr; 2] = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];

    println!("Starting ChatMambo server ");

    let listener = bind_addresses(&chat_mambo_addr).await;
    println!("Server listening on: {:?}", listener.local_addr().unwrap());

    loop {
        let (sender, mut receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) =
            mpsc::channel::<String>(5);

        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let (mut reader, mut writer) = stream.into_split();
        tokio::spawn(async move { handle_connection(reader, writer, sender, receiver).await });
    }
}

async fn handle_connection(
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
    sender: mpsc::Sender<String>,
    mut receiver: mpsc::Receiver<String>,
) {
    let mut buffer = [0; 1024];

    loop {
        tokio::select! {
               message = receiver.recv() => {
                   match message {
                   Some(message) => {
                       println!("Received message from channel: {}", message);
                       if writer.write_all(message.as_bytes()).await.is_ok() {
                           println!("Sent message to client {}", writer.peer_addr().unwrap());
                       } else {
                           eprintln!("Failed to send message to client {}", writer.peer_addr().unwrap());
                       }
                   }
                   None => {
                       eprintln!("Channel closed, no more messages to receive");
                       break;
                   }

               }
           }

               result = reader.read(&mut buffer) => {
                   match result {
                   Ok(0) =>{
                       println!("Client disconnected: {:?}", reader.peer_addr().unwrap());
                       break;
                   }

                   Ok(bytes) => {

                       print!("Received {} bytes from client \n", bytes);

                       let message_str = String::from_utf8_lossy(&buffer[..bytes]);
                       println!("Received message: \"{}\"", message_str);

                       sender.send(message_str.to_string()).await.unwrap();



                       //  let  processor_code  = move || {
                       //     println!("Starting processor code");
                       //     loop{
                       //         println!("Attempting to receive message from channel");
                       //         if received_message.is_ok() {
                       //             println!("Received message from channel: {}", received_message.unwrap());
                       //             break;
                       //         } else {
                       //             println!("No message received from channel, waiting...");
                       //         }
                       //     }
                       //   };
                       //   tokio::spawn(async move {
                       //     processor_code();
                       //   });
                       // writer.write_all(message_str.as_bytes()).await.unwrap();
                       // println!("Sent response to client {}", writer.peer_addr().unwrap());

                       // if writer.shutdown().await.is_ok() {
                       //     println!("Connection closed with client {}", writer.peer_addr().unwrap());
                       // } else {
                       //     println!("Failed to close connection with client {}", writer.peer_addr().unwrap());
                       // }
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

async fn bind_addresses(addresses: &[SocketAddr]) -> TcpListener {
    for address in addresses {
        match TcpListener::bind(address).await {
            Ok(listener) => return listener,
            Err(e) => eprintln!("Failed to bind to {}: {}", address, e),
        }
    }
    panic!("Failed to bind to any ofthe provided addresses");
}
