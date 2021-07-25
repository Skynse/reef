extern crate clap;
use clap::{App, Arg};
use regex::Regex;

mod ports;
mod scanner;

fn main() {
    let ip_regex = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    let app = App::new("Reef")
        .version("0.1.0")
        .arg(
            Arg::with_name("pscan")
                .help("Port Scanner")
                .short("S")
                .takes_value(true)
                .value_name("addr")
                .validator(move |x| {
                    if ip_regex.is_match(&x) {
                        Ok(())
                    } else {
                        Err(format!("Invalid IPv4 Address"))
                    }
                }),
        )
        .arg(
            Arg::with_name("start")
                .help("Starting value for port")
                .takes_value(true)
                .value_name("start"),
        );

    let matches = app.get_matches();
    if matches.is_present("pscan") {
        let ip = matches.value_of("pscan").unwrap();
        let ip_obj = scanner::PortScanner::new(ip);
        ip_obj.scan_ports();
    }
}
