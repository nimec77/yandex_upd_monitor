use std::{io, net::UdpSocket};

pub struct MetricsSender {
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_address: &str) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(bind_address)?;
        Ok(Self { socket })
    }
}
