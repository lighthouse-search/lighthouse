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
    // Iterate as u32 so the 65535 upper bound is actually reachable; a u16
    // counter would overflow-panic before the bound could ever be checked.
    for port in 4000u32..=65535 {
        let port = port as u16;
        if is_port_available(port) {
            return Ok(port);
        }
    }

    Err("Failed to find port (between 4000 and 65535). Either no port is available or Guard has insufficient permissions.".to_string())
}
