use crate::{
    ChannelId, StreamError,
    udp_cipher::{MAX_UDP_DATAGRAM_BYTES, UdpCipher},
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
    health: Arc<crate::health::StreamHealth>,
}

impl UdpDataChannel {
    /// Binds an ephemeral local UDP port (`0.0.0.0:0`) for this session's real-time stream.
    pub async fn bind(
        udp_key: [u8; 32],
        stream_id: u32,
    ) -> Result<(Arc<Self>, UdpEndpointHandle), StreamError> {
        let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0))).await?;
        let local_port = socket.local_addr()?.port();

        let channel = Arc::new(Self {
            socket: Arc::new(socket),
            peer: Arc::new(RwLock::new(None)),
            tx_cipher: Mutex::new(UdpCipher::new(udp_key, stream_id)),
            rx_cipher: Mutex::new(UdpCipher::new(udp_key, stream_id)),
            stream_id,
            health: Arc::new(crate::health::StreamHealth::new()),
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

    /// Starts one authenticated round-trip probe per second on this service's socket.
    pub fn monitor(self: &Arc<Self>, kind: ChannelId) -> Arc<crate::health::StreamHealth> {
        let weak = Arc::downgrade(self);
        let health = self.health.clone();
        let stream_id = self.stream_id;
        tokio::spawn(async move {
            let mut sequence = 0u64;
            let mut previous = crate::health::LiveLatency::Pending;
            loop {
                let Some(channel) = weak.upgrade() else {
                    break;
                };
                if !health.enabled() {
                    break;
                }
                let (status, rtt) = health.snapshot();
                let event = match (previous, status) {
                    (_, crate::health::LiveLatency::Down) if previous != status => {
                        "connection_lost"
                    }
                    (crate::health::LiveLatency::Down, _) if previous != status => {
                        "connection_restored"
                    }
                    _ => "sample",
                };
                previous = status;
                let _ = tokio::task::spawn_blocking(move || {
                    crate::health_log::record(kind, stream_id, status, rtt, event)
                })
                .await;
                if channel.peer_addr().is_some() {
                    sequence = sequence.wrapping_add(1);
                    let probe = health.probe(sequence, std::time::Instant::now());
                    let _ = channel.send_datagram(kind, &probe).await;
                }
                drop(channel);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            let (status, rtt) = health.snapshot();
            if status == crate::health::LiveLatency::Down && previous != status {
                let _ = tokio::task::spawn_blocking(move || {
                    crate::health_log::record(kind, stream_id, status, rtt, "connection_lost")
                })
                .await;
            }
            let _ = tokio::task::spawn_blocking(move || {
                crate::health_log::record(kind, stream_id, status, rtt, "stopped")
            })
            .await;
        });
        self.health.clone()
    }

    /// Waits for an authenticated `UDP_HELLO` datagram from the paired Android phone
    /// so desktop learns its exact UDP port and punches through Windows Firewall.
    pub async fn accept_peer_hello(&self) -> Result<SocketAddr, StreamError> {
        let mut buf = [0u8; MAX_UDP_DATAGRAM_BYTES];
        loop {
            let (len, from) = self.socket.recv_from(&mut buf).await?;
            let decrypted = {
                let mut rx = self
                    .rx_cipher
                    .lock()
                    .map_err(|_| StreamError::DecryptFailed)?;
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
    pub async fn send_datagram(
        &self,
        channel: ChannelId,
        payload: &[u8],
    ) -> Result<(), StreamError> {
        let Some(target) = self.peer_addr() else {
            return Ok(());
        };
        let datagram = {
            let mut tx = self
                .tx_cipher
                .lock()
                .map_err(|_| StreamError::EncryptFailed)?;
            tx.encrypt_datagram(channel, payload)?
        };
        self.socket.send_to(&datagram, target).await?;
        Ok(())
    }

    /// Receives and authenticates a datagram only from the peer established by
    /// `accept_peer_hello`. Packets from other addresses are ignored.
    pub async fn recv_datagram(&self) -> Result<(ChannelId, u64, Vec<u8>), StreamError> {
        let peer = self.peer_addr().ok_or(StreamError::PeerNotEstablished)?;
        let mut buf = [0u8; MAX_UDP_DATAGRAM_BYTES];
        loop {
            let (len, from) = self.socket.recv_from(&mut buf).await?;
            if from != peer {
                continue;
            }
            let packet = {
                let mut rx = self
                    .rx_cipher
                    .lock()
                    .map_err(|_| StreamError::DecryptFailed)?;
                rx.decrypt_datagram(&buf[..len])?
            };
            if self.health.reply(&packet.2, std::time::Instant::now()) {
                continue;
            }
            return Ok(packet);
        }
    }

    pub fn stream_id(&self) -> u32 {
        self.stream_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::health::{LiveLatency, REPLY_PREFIX};
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn each_service_consumes_authenticated_pong_without_exposing_it_as_media() {
        for kind in [ChannelId::Speaker, ChannelId::Mic, ChannelId::Mouse] {
            let key = [7; 32];
            let (channel, endpoint) = UdpDataChannel::bind(key, 42).await.unwrap();
            let phone = UdpSocket::bind("127.0.0.1:0").await.unwrap();
            let target = SocketAddr::from(([127, 0, 0, 1], endpoint.local_port));
            let mut phone_tx = UdpCipher::new(key, 42);
            let hello = phone_tx.encrypt_datagram(kind, UDP_HELLO_PAYLOAD).unwrap();
            phone.send_to(&hello, target).await.unwrap();
            tokio::time::timeout(Duration::from_secs(1), channel.accept_peer_hello())
                .await
                .unwrap()
                .unwrap();

            let probe = channel.health.probe(17, Instant::now());
            channel.send_datagram(kind, &probe).await.unwrap();
            let mut buffer = [0; MAX_UDP_DATAGRAM_BYTES];
            let (len, _) =
                tokio::time::timeout(Duration::from_secs(1), phone.recv_from(&mut buffer))
                    .await
                    .unwrap()
                    .unwrap();
            let (_, _, received) = UdpCipher::new(key, 42)
                .decrypt_datagram(&buffer[..len])
                .unwrap();
            assert_eq!(received, probe);

            let pong = phone_tx
                .encrypt_datagram(kind, &[REPLY_PREFIX, &17u64.to_be_bytes()].concat())
                .unwrap();
            phone.send_to(&pong, target).await.unwrap();
            let media = phone_tx.encrypt_datagram(kind, b"media").unwrap();
            phone.send_to(&media, target).await.unwrap();
            let (received_kind, _, payload) =
                tokio::time::timeout(Duration::from_secs(1), channel.recv_datagram())
                    .await
                    .unwrap()
                    .unwrap();
            assert_eq!(received_kind, kind);
            assert_eq!(payload, b"media");
            assert!(matches!(
                channel.health.snapshot().0,
                LiveLatency::Millis(_) | LiveLatency::Slow
            ));
        }
    }
}
