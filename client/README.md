# ChatMambo Client

A terminal-based client for ChatMambo. It connects to the server, lets you send messages, and displays incoming chat messages from other users.

## What the client does

- Connects to the ChatMambo server
- Sends text messages to the server
- Receives broadcasted messages from other clients
- Runs from the terminal

## Requirements

- Rust and Cargo installed

## Build

From this folder, run:

```bash
cargo build
```

## Run

Start the client with:

```bash
cargo run -- 127.0.0.1:8080
```

If you do not provide an address, the client will try to use 127.0.0.1:7778, so it is better to pass the server address explicitly.

## Usage

- Type a message and press Enter to send it
- Type exit to quit the client
- Incoming messages will appear in the terminal

## Troubleshooting

- If the connection fails, make sure the server is already running
- Confirm that the address and port match the server configuration
- If nothing appears, check that the server and client were started in separate terminals
