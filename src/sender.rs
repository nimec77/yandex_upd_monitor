use std::{io, net::UdpSocket};

use crate::metrics::RoomMetrics;

pub struct MetricsSender {
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_address: &str) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(bind_address)?;
        Ok(Self { socket })
    }

    pub fn send_to(&self, metrics: &RoomMetrics, target_address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = bincode::serde::encode_to_vec(metrics, bincode::config::standard())?;
        self.socket.send_to(&encoded, target_address)?;
        Ok(())
    }
}
