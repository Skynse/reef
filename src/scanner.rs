extern crate pnet;

use std::process::{Command, Stdio};
use crate::ports;

use std::sync::mpsc::{Sender, channel};

use crossbeam_utils::thread;

use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

use pnet::datalink;

pub struct IPScanner {

}

pub struct PortScanner {
    ip: IpAddr,
}

pub fn get_default_gateway() -> String {
    let mut gateway = String::new();
    for iface in datalink::interfaces() {
        if iface.ips[0].to_string().starts_with("192") {
            gateway = iface.ips[0].ip().to_string();
        }
    }
    gateway
}

impl IPScanner {

    pub fn run(&self) {
        let (sender, receiver) = channel::<u16>();
        let mut active_masks: Vec<u16> = vec![];
        let masks: Vec<u16> = (0..=256).collect();

        let chunk_count = 256 / 10;
        for chunk in  masks.chunks(chunk_count) {
            let chunk = chunk.to_owned();
            let sender = sender.clone();

            thread::scope(|s| {
                s.spawn(move |_| {
                    self.scan(sender, chunk);
                });
            }).unwrap();
    }
    drop(sender);

    for mask in receiver {
        active_masks.push(mask);
    }
    println!("{:?} Hosts online", active_masks);
    }
    
    pub fn scan(&self, sender: Sender<u16>, mask: Vec<u16>) {
        let start = get_default_gateway()[0..9].to_string();

        for i in mask {
            let ip = format!("{}.{}", start, i.to_string());
            let proc = Command::new("ping")
            .arg("-c").arg("1").arg("-W 0.1").arg("-q")
                .arg(ip)
                .stdout(Stdio::null())
                .spawn()
                .expect("Failed");

            let stat = proc
                .wait_with_output()
                .expect("Failed");

            if stat.status.code().unwrap() == 2 {
                println!("FOUND {}.{}", start, i);
            }
            sender.send(i).unwrap(); 
        } 
}

}
impl PortScanner {
    pub fn new(ip: &str) -> PortScanner {
        PortScanner {
            ip: ip.parse::<IpAddr>().expect("Invalid IP address"),
        }
    }

    pub fn run(&self) {
        self.scan("TCP");
    }

    pub fn scan(&self, protocol : &str) {
        thread::scope(|s| {
            s.spawn(move |_| {
                for &port in ports::TCP {
                    let socket = SocketAddr::new(self.ip, port);
                    let timeout = Duration::new(2, 0);
                    if let Ok(stream) = TcpStream::connect_timeout(&socket, timeout) {
                        println!("OPEN -> {} [{}]", port, protocol);
                        stream.shutdown(Shutdown::Both).expect("Exiting");
                    }
                }
            });
        })
        .unwrap();
    }
}
