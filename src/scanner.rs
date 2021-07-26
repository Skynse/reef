use crate::ports;
use crossbeam_utils::thread;
use std::net::{IpAddr, Shutdown, TcpStream, SocketAddr};
use std::time::Duration;

pub struct PortScanner {
    ip: IpAddr,
}

impl PortScanner {
    pub fn new(ip: &str) -> PortScanner {
        PortScanner {
            ip: ip.parse::<IpAddr>().expect("Invalid IP address"),
        }
    }

    pub fn scan_ports(&self) {
        thread::scope (|s| {
            s.spawn (move |_| {
                for &port in ports::TCP {
                    let socket = SocketAddr::new(self.ip, port);
                    let timeout = Duration::new(2, 0);
                    if let Ok(stream) = TcpStream::connect_timeout(&socket, timeout) {
                        println!("OPEN -> {}", port);
                        stream.shutdown(Shutdown::Both).expect("Exiting");
                    }
                }

        }); //closer
    }).unwrap();



    }
}
