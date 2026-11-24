use tokio::net::TcpStream;
use wirewave_core_transport::TransportKind;

/// Consumer receiving accepted transport streams from listeners.
pub trait StreamAcceptor: Send + Sync + 'static {
    fn accept_stream(&self, stream: TcpStream, transport: TransportKind);
}

/// Common abstraction for network transport listeners (Wi-Fi TCP, Bluetooth RFCOMM).
pub trait TransportListener: Send + Sync + 'static {
    fn transport_kind(&self) -> TransportKind;
}
