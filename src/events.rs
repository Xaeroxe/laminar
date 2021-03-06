use std::sync::{Arc, RwLock};

use crate::net::{NetworkQuality, VirtualConnection};

/// Events that are generated in response to a change in state of the connected client
pub enum Event {
    /// A new client connects. Clients are uniquely identified by the ip:port combination at this layer.
    Connected(Arc<RwLock<VirtualConnection>>),
    /// A client disconnects. This can be generated from the server-side intentionally disconnecting a client,
    /// or it could be from the client disconnecting.
    Disconnected(Arc<RwLock<VirtualConnection>>),
    /// This is generated if the server has not seen traffic from a client for a configurable amount of time.
    TimedOut(Arc<RwLock<VirtualConnection>>),
    /// This is generated when there is a change in the connection quality of a client.
    QualityChange {
        /// Connection whose quality changed
        conn: Arc<RwLock<VirtualConnection>>,
        /// Previous quality
        from: NetworkQuality,
        /// New quality
        to: NetworkQuality,
    },
}

#[cfg(test)]
mod test {
    use super::Event;
    use crate::config::NetworkConfig;
    use crate::net::VirtualConnection;
    use std::net::ToSocketAddrs;
    use std::sync::{Arc, RwLock};

    static TEST_HOST_IP: &'static str = "127.0.0.1";
    static TEST_PORT: &'static str = "20000";

    #[test]
    fn test_create_event() {
        let addr = format!("{}:{}", TEST_HOST_IP, TEST_PORT).to_socket_addrs();
        let mut addr = addr.unwrap();

        let test_conn = Arc::new(RwLock::new(VirtualConnection::new(
            addr.next().unwrap(),
            Arc::new(NetworkConfig::default()),
        )));
        let _ = Event::Connected(test_conn);
    }
}
