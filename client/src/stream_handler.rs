use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn handle_stream(stream: TcpStream) {
    // Split the stream into independent read/write halves
    let (reader, writer) = stream.into_split();

    // Run sending and receiving concurrently
    let send_task = tokio::spawn(send_message(writer));
    let receive_task = tokio::spawn(receive_message(reader));

    // Wait for both tasks to complete
    let _ = tokio::try_join!(send_task, receive_task);
}

async fn send_message(mut writer: OwnedWriteHalf) {
    loop {
        println!("Enter message to send to server (or type 'exit' to quit):");

        let mut message = String::new();

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read input from user");

        let message = message.trim().to_string();

        if message.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        if let Err(e) = writer.write_all(message.as_bytes()).await {
            eprintln!("Failed to send message to server: {}", e);
            break;
        }

        println!("Sent message to server!");
    }
}

async fn receive_message(mut reader: OwnedReadHalf) {
    let mut buffer = [0; 1024];

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => {
                eprintln!("Server closed the connection.");
                break;
            }

            Ok(bytes_read) => {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received: {response}");
            }

            Err(e) => {
                eprintln!("Failed to read response from server: {}", e);
                break;
            }
        }
    }
}
