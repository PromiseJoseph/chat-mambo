# ChatMambo

ChatMambo is a small real-time chat application built in Rust with Tokio. It consists of a TCP server and a terminal-based client that let users connect, choose a username, and exchange messages over the network.

## What the project does

- The server accepts incoming TCP connections.
- Each client is prompted for a username when they connect.
- Messages sent by one client are broadcast to the other connected clients.
- The project is a lightweight prototype for learning async networking in Rust.

## Project structure

- server/: contains the chat server implementation
- client/: contains the terminal client implementation

## Requirements

- Rust and Cargo installed
- A terminal to run the server and client

If you do not already have Rust installed, install it with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Setup

1. Open a terminal in the repository root.
2. Build the server:

```bash
cd server
cargo build
```

3. Build the client:

```bash
cd ../client
cargo build
```

## Running the app

### Start the server

From the server directory, run:

```bash
cargo run
```

The server will try to bind to 127.0.0.1:8080 first and then 127.0.0.1:8081 if needed. If you want to choose a specific port, pass it as an argument:

```bash
cargo run -- 127.0.0.1:8080
```

### Start the client

Open a second terminal and run:

```bash
cd client
cargo run -- 127.0.0.1:8080
```

Use the same address that the server is listening on.

## Using the chat

- The server will ask you for a username when you connect.
- Type a message and press Enter to send it.
- Type exit to leave the chat.

## Licence

This project is licensed under the MIT License. See the `LICENSE` file
for details.
