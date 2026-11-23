use crate::{
    udp_cipher::{MAX_UDP_DATAGRAM_BYTES, UdpCipher},
    ChannelId, StreamError,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};
use tokio::net::UdpSocket;

pub const UDP_HELLO_PAYLOAD: &[u8] = b"WW2U_HELLO";

#[derive(Debug, Clone, Copy)]
pub struct UdpEndpointHandle {
    pub local_port: u16,
    pub stream_id: u32,
}

/// Encrypted UDP channel bound to a single authenticated peer discovered via `UDP_HELLO`.
pub struct UdpDataChannel {
    socket: Arc<UdpSocket>,
    peer: Arc<RwLock<Option<SocketAddr>>>,
    tx_cipher: Mutex<UdpCipher>,
    rx_cipher: Mutex<UdpCipher>,
    stream_id: u32,
}

impl UdpDataChannel {
    /// Binds an ephemeral local UDP port (`0.0.0.0:0`) for this session's real-time stream.
    pub async fn bind(udp_key: [u8; 32], stream_id: u32) -> Result<(Arc<Self>, UdpEndpointHandle), StreamError> {
        let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0))).await?;
        let local_port = socket.local_addr()?.port();

        let channel = Arc::new(Self {
            socket: Arc::new(socket),
            peer: Arc::new(RwLock::new(None)),
            tx_cipher: Mutex::new(UdpCipher::new(udp_key, stream_id)),
            rx_cipher: Mutex::new(UdpCipher::new(udp_key, stream_id)),
            stream_id,
        });

        Ok((
            channel,
            UdpEndpointHandle {
                local_port,
                stream_id,
            },
        ))
    }

    pub fn peer_addr(&self) -> Option<SocketAddr> {
        self.peer.read().ok().and_then(|g| *g)
    }

    /// Waits for an authenticated `UDP_HELLO` datagram from the paired Android phone
    /// so desktop learns its exact UDP port and punches through Windows Firewall.
    pub async fn accept_peer_hello(&self) -> Result<SocketAddr, StreamError> {
        let mut buf = [0u8; MAX_UDP_DATAGRAM_BYTES];
        loop {
            let (len, from) = self.socket.recv_from(&mut buf).await?;
            let decrypted = {
                let mut rx = self.rx_cipher.lock().map_err(|_| StreamError::DecryptFailed)?;
                rx.decrypt_datagram(&buf[..len])
            };
            if let Ok((_ch, _seq, payload)) = decrypted {
                if payload == UDP_HELLO_PAYLOAD {
                    if let Ok(mut guard) = self.peer.write() {
                        *guard = Some(from);
                    }
                    return Ok(from);
                }
            }
        }
    }

    /// Encrypts and transmits a single MTU-safe UDP datagram to the locked peer.
    pub async fn send_datagram(&self, channel: ChannelId, payload: &[u8]) -> Result<(), StreamError> {
        let Some(target) = self.peer_addr() else {
            return Ok(());
        };
        let datagram = {
            let mut tx = self.tx_cipher.lock().map_err(|_| StreamError::EncryptFailed)?;
            tx.encrypt_datagram(channel, payload)?
        };
        self.socket.send_to(&datagram, target).await?;
        Ok(())
    }

    pub fn stream_id(&self) -> u32 {
        self.stream_id
    }
}
