use std::fmt::Debug;

use error::{NetError, Result, ServerError};
use tokio::net::{ToSocketAddrs, UdpSocket};
use tracing::{info, warn};

const BUFFER_SIZE: usize = 1472;

pub struct Listener {
    guid: u64,
    socket: UdpSocket,
}

impl Listener {
    pub async fn bind<A: ToSocketAddrs + Debug>(addr: A) -> Result<Self> {
        let guid = rand::random();

        let socket = UdpSocket::bind(&addr)
            .await
            .map_err(|e| NetError::BindError(format!("Failed to bind to {:?}: {}", addr, e)))
            .map_err(ServerError::Net)?;

        Ok(Self { guid, socket })
    }

    pub async fn listen(&self) -> Result<()> {
        info!("Server listening (GUID: {})", self.guid);

        let mut buf = [0u8; BUFFER_SIZE];

        loop {
            let (size, addr) = match self.socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(e) => {
                    warn!("Failed to receive packet: {}", e);
                    continue;
                }
            };

            // Validate size early
            if size == 0 {
                continue;
            }

            let _packet = &buf[..size];

            tracing::trace!("Received {} bytes from {}", size, addr);
        }
    }
}
