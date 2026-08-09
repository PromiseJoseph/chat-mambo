# ChatMambo Server

A TCP chat server for ChatMambo. It accepts client connections, asks each client for a username, and broadcasts messages to the other connected clients.

## What the server does

- Accepts incoming TCP connections
- Prompts each client for a username
- Receives messages from one client and sends them to the others
- Runs as a simple async server using Tokio

## Requirements

- Rust and Cargo installed

## Build

From this folder, run:

```bash
cargo build
```

## Run

Start the server with:

```bash
cargo run
```

By default, the server tries to bind to:

- 127.0.0.1:8080
- 127.0.0.1:8081

You can also choose a specific address manually:

```bash
cargo run -- 127.0.0.1:8080
```
