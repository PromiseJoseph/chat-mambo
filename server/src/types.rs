use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Message(pub Option<String>, pub String);

#[derive(Debug, Clone)]
pub struct Client {
    pub username: String,
    pub client_id: String,
    pub peer_addr: SocketAddr,
}
