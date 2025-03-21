use std::net::{SocketAddr, TcpListener};

pub fn find_open_port_in_range(start: u16, end: u16) -> Option<u16> {
    for port in start..=end {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if TcpListener::bind(addr).is_ok() {
            return Some(port);
        }
    }
    None
}
