use crate::error::Result;
use tokio::net::UdpSocket;

/// A UDP listener for incoming connections.
pub struct Listener {
    /// The unique identifier for the server.
    server_guid: u64,
    /// The UDP socket used for listening.
    socket: UdpSocket,
}

impl Listener {
    /// Binds a new UDP listener to the specified port.
    pub async fn bind(port: u16) -> Result<Self> {
        // Generate a random server GUID
        let server_guid = rand::random();

        // Bind to all available network interfaces
        let addr = format!("0.0.0.0:{}", port);
        // Bind the socket to the specified address
        let socket = UdpSocket::bind(&addr).await?;

        Ok(Self {
            server_guid,
            socket,
        })
    }

    /// Accepts a new connection from the listener.
    pub async fn accept(&self) -> Result<()> {
        // Temporary placeholder to avoid clippy throw an error
        println!("Server guid: {}", self.server_guid);

        // Buffer to store incoming data
        let mut buf = [0; 2048];

        loop {
            // Receive data from the socket
            let (size, addr) = self.socket.recv_from(&mut buf).await?;

            // Temporary placeholder to avoid clippy throw an error
            println!("Received data from: {:?}", addr);

            // Process the received data
            let packet = &buf[..size];

            // Check if the packet is empty
            if packet.is_empty() {
                continue;
            }
        }
    }
}
