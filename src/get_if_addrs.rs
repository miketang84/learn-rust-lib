

use get_if_addrs::get_if_addrs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let interfaces = match get_if_addrs() {
        Ok(interfaces) => interfaces,
            Err(err) => {
                println!("could not get list of interfaces: {}", err);
                return;
            }
    };

    for iface in interfaces {
        if iface.is_loopback() {
            continue;
        }

        println!("found interface {:?}", iface);
        match iface.ip() {
            IpAddr::V4(ip) => {
                println!("{:?}", ip);
            }
            IpAddr::V6(ip) => {
                println!("{:?}", ip);
            }
            _ => (),
        }
    }

}
