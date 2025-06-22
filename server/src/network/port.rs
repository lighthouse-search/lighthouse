use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

pub fn is_port_available(port: u16) -> bool {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);

    match TcpListener::bind(addr) {
        Ok(listener) => {
            // The bind succeeded; the port is free.
            drop(listener);        // Close immediately so others can take it.
            true
        }
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => false, // Someone else is listening.
        Err(e) => {
            // Any other error probably means you don’t have permission
            // (e.g., trying to bind <1024 without root on Unix) or the
            // address family isn’t available. Bubble it up if you need to
            // distinguish those cases.
            log::error!("Unexpected error probing port {port}: {e}");
            false
        }
    }
}

pub fn find_available_port() -> Result<u16, String> {
    let mut last_attempted = 4000;

    loop {
        if (last_attempted > 65535) {
            return Err("Failed to find port (between 4000 and 65535). Either not port is available of Guard has insufficient permissions.".to_string());
        }
        
        if is_port_available(last_attempted) {
            return Ok(last_attempted);
        } else {
            // Try again.
            last_attempted += 1;
        }
    }
}
