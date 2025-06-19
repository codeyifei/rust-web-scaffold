use std::net::IpAddr;

pub fn ip_to_u32(ip: IpAddr) -> Option<u32> {
    match ip {
        IpAddr::V4(ipv4) => Some(u32::from(ipv4)),
        IpAddr::V6(_) => None,
    }
}

pub fn ip_str_to_u32(ip_str: &str) -> Option<u32> {
    ip_str.parse::<IpAddr>().ok().and_then(ip_to_u32)
}