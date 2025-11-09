use std::{io, net::UdpSocket, thread, time::Duration};

use log::{debug, error, info};

use crate::{init_logger, metrics::RoomMetrics};

pub struct MetricsSender {
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_address: &str) -> Result<Self, io::Error> {
        init_logger();
        
        let socket = UdpSocket::bind(bind_address)?;

        info!("MetricsSender initialized");

        Ok(Self { socket })
    }

    pub fn send_to(
        &self,
        metrics: &RoomMetrics,
        target_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending metrics to {target_address}");
        let encoded = bincode::serde::encode_to_vec(metrics, bincode::config::standard())?;

        self.socket.send_to(&encoded, target_address)?;
        debug!("Metrics sent to {target_address}");
        Ok(())
    }

    pub fn start_broadcasting(
        self,
        target_address: String,
        interval_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "The sensor simulator has been launched. Sending to {target_address} every {interval_ms}ms"
        );

        debug!(
            "The sensor simulator has been launched. Sending to {target_address} every {interval_ms}ms"
        );

        #[cfg(feature = "random")]
        info!("✅ The 'random' feature is active - rand is used to generate data");

        #[cfg(not(feature = "random"))]
        info!("✅ The 'random' feature is inactive - fixed data is used");

        loop {
            debug!("Sending metrics...");
            let metrics = RoomMetrics::random();

            match self.send_to(&metrics, &target_address) {
                Ok(()) => {
                    info!(
                        "[{}] Sent: {:.1}C, {:.1}%RH, {:.1}hPa, Door: {}, Vibration: {:.1}%",
                        metrics.formatted_time(),
                        metrics.temperature,
                        metrics.humidity,
                        metrics.pressure,
                        metrics.door_to_string(),
                        metrics.vibration_level,
                    );

                    #[cfg(feature = "sqlite")]
                    {
                        info!("   💾 SQL: {}", metrics.to_sql());
                    }
                }
                Err(e) => error!("Error sending metrics: {e}"),
            }

            thread::sleep(Duration::from_millis(interval_ms));
        }
    }
}
