use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct UnconnectedPing {
    pub ping_timestamp: u64,
    pub client_guid: i64,
}

#[derive(Clone, Debug)]
pub struct UnconnectedPong {
    pub ping_timestamp: u64,
    pub server_guid: u64,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    pub client_guid: i64,
    pub request_timestamp: u64,
    pub use_security: bool,
}

#[derive(Debug, Clone)]
pub struct ConnectionRequestAccepted {
    pub client_address: SocketAddr,
    pub system_index: u16,
    pub system_addresses: Vec<SocketAddr>,
    pub request_timestamp: u64,
    pub server_timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct OpenConnectionRequest1 {
    pub protocol_version: u8,
    pub estimated_mtu: u16,
}

#[derive(Debug, Clone)]
pub struct OpenConnectionReply1 {
    pub server_guid: u64,
    pub use_security: bool,
    pub cookies: u32,
    pub mtu_size: u16,
}

#[derive(Debug, Clone)]
pub struct OpenConnectionRequest2 {
    pub server_address: SocketAddr,
    pub mtu_size: u16,
    pub client_guid: i64,
    pub use_encryption: bool,
    pub cookies: u32,
}

#[derive(Debug, Clone)]
pub struct OpenConnectionReply2 {
    pub server_guid: u64,
    pub client_address: SocketAddr,
    pub mtu_size: u16,
    pub use_encryption: bool,
}
