use crate::ports;
use crossbeam_utils::thread;
use std::net::{Ipv4Addr, Shutdown, TcpStream};

pub struct PortScanner {
    ip: Ipv4Addr,
}

impl PortScanner {
    pub fn new(ip: &str) -> PortScanner {
        PortScanner {
            ip: ip.parse::<Ipv4Addr>().expect("Invalid IP address"),
        }
    }

    pub fn scan_ports(&self) {
        thread::scope (|s| {
            s.spawn (move |_| {
                for port in ports::TCP {
                    let ip = format!("{}:{}", &self.ip, &port);
                    if let Ok(stream) = TcpStream::connect(ip) {
                        println!("OPEN -> {}", port);
                        stream.shutdown(Shutdown::Both).expect("Exiting");
                    }
                }

        }); //closer
    }).unwrap();



    }
}
