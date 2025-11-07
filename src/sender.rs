use std::{io, net::UdpSocket, thread, time::Duration};

use crate::metrics::RoomMetrics;

pub struct MetricsSender {
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_address: &str) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(bind_address)?;
        Ok(Self { socket })
    }

    pub fn send_to(
        &self,
        metrics: &RoomMetrics,
        target_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = bincode::serde::encode_to_vec(metrics, bincode::config::standard())?;
        self.socket.send_to(&encoded, target_address)?;
        Ok(())
    }

    pub fn start_broadcasting(
        self,
        target_address: String,
        interval_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "The sensor simulator has been launched. Sending to {target_address} every {interval_ms}ms"
        );

        loop {
            let metrics = RoomMetrics::random();

            match self.send_to(&metrics, &target_address) {
                Ok(()) => print!(
                    "[{}] Sent: {:.1}C, {:.1}%RH, {:.1}hPa, Door: {}",
                    metrics.formatted_time(),
                    metrics.temperature,
                    metrics.humidity,
                    metrics.pressure,
                    metrics.door_to_string(),
                ),
                Err(e) => eprintln!("Error sending metrics: {e}"),
            }

            thread::sleep(Duration::from_millis(interval_ms));
        }
    }
}
