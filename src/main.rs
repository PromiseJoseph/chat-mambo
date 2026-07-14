use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {

let chat_mambo_addr: [SocketAddr; 2] = [
    SocketAddr::from(([127, 0, 0, 1], 8080)),
    SocketAddr::from(([127, 0, 0, 1], 8081))
];

    println!("Starting ChatMambo server ");
    
let listener = bind_addresses(&chat_mambo_addr).await;

loop{

  let (stream, _) = listener.accept().await.expect("Failed to accept connection");
        println!("Accepted connection from: {:?}", stream.peer_addr().unwrap());
            tokio::spawn(async move {
                handle_connection(stream).await
            });
        
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) =>{
                println!("Client disconnected: {:?}", stream.peer_addr().unwrap());
                break;
            }

            Ok(bytes) => {

                print!("Received {} bytes from client \n", bytes);
               
                let message_str = String::from_utf8_lossy(&buffer[..bytes]);
                println!("Received message: {}", message_str);

                stream.write_all(message_str.as_bytes()).await.unwrap();
                println!("Sent response to client {}", stream.peer_addr().unwrap());
                
                if stream.shutdown().await.is_ok() {
                    println!("Connection closed with client {}", stream.peer_addr().unwrap());
                } else {
                    println!("Failed to close connection with client {}", stream.peer_addr().unwrap());
                }
            }

            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
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