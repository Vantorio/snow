use std::{fmt::Debug, sync::Arc};

use error::{Error, NetError, Result};
use tokio::net::{ToSocketAddrs, UdpSocket};
use tracing::{info, trace, warn};

const DEFAULT_BUFFER_SIZE: usize = 2048;

pub struct Listener {
    guid: u64,
    socket: Arc<UdpSocket>,
}

impl Listener {
    pub async fn bind<A: ToSocketAddrs + Debug>(addr: A) -> Result<Self> {
        let guid = rand::random();
        let socket = UdpSocket::bind(&addr)
            .await
            .map_err(|e| NetError::BindError(format!("Failed to bind to {:?}: {}", addr, e)))
            .map_err(Error::Net)?;
        let socket = Arc::new(socket);
        Ok(Self { guid, socket })
    }

    pub async fn listen(&self) -> Result<()> {
        info!("Server running on {}", self.socket.local_addr()?);
        info!("Press CTRL + C to exit.");

        let _temp = self.guid;

        let mut buf = [0u8; DEFAULT_BUFFER_SIZE];

        loop {
            let (len, src) = match self.socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(e) => {
                    warn!("Failed to receive packet: {}", e);
                    continue;
                }
            };

            // Validate size early
            if len == 0 {
                continue;
            }

            let _packet = &buf[..len];

            trace!("Received {} bytes from {}", len, src);
        }
    }
}
