use std::{net::UdpSocket, sync::mpsc, thread};

use log::{error, info};

use crate::{init_logger, metrics::RoomMetrics};

pub struct MetricsReceiver {
    socket: UdpSocket,
}

impl MetricsReceiver {
    pub fn new(bind_address: &str) -> Result<Self, std::io::Error> {
        init_logger();
        let socket = UdpSocket::bind(bind_address)?;
        info!("Server run on {bind_address}");

        Ok(Self { socket })
    }

    pub fn start_in_thread(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            if let Err(e) = self.receive_loop() {
                error!("Error receiving metrics: {e}");
            }
        })
    }

    fn receive_loop(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0u8; 1024];

        info!("Waiting for metrics...");

        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, src_addr)) => {
                    match bincode::serde::decode_from_slice::<RoomMetrics, _>(
                        &buf[..size],
                        bincode::config::standard(),
                    ) {
                        Ok((metrics, _)) => {
                            info!(
                                "[{}] Received metrics from {}: {:.1}C, {:.1}%RH, {:.1}hPa, Door: {}, Vibration: {:.1}%",
                                metrics.formatted_time(),
                                src_addr,
                                metrics.temperature,
                                metrics.humidity,
                                metrics.pressure,
                                metrics.door_to_string(),
                                metrics.vibration_level,
                            );
                        }
                        Err(e) => error!("Error decoding metrics: {e}"),
                    }
                }
                Err(e) => error!("Error receiving metrics: {e}"),
            }
        }
    }

    pub fn start_with_channel(
        self,
    ) -> (
        thread::JoinHandle<()>,
        mpsc::Receiver<(RoomMetrics, std::net::SocketAddr)>,
    ) {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            if let Err(e) = self.receive_loop_with_channel(tx) {
                error!("Error receiving metrics: {e}");
            }
        });

        (handle, rx)
    }

    fn receive_loop_with_channel(
        self,
        tx: mpsc::Sender<(RoomMetrics, std::net::SocketAddr)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0u8; 1024];

        info!("Data channel is ready");

        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, src_addr)) => match bincode::serde::decode_from_slice::<RoomMetrics, _>(
                    &buf[..size],
                    bincode::config::standard(),
                ) {
                    Ok((metrics, _)) => {
                        if tx.send((metrics, src_addr)).is_err() {
                            error!("Channel is closed. Stopping the receiver thread.");
                            break;
                        }
                    }
                    Err(e) => error!("Error decoding metrics: {e}"),
                },
                Err(e) => error!("Error receiving metrics: {e}"),
            }
        }
        Ok(())
    }
}
