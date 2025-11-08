use std::{sync::mpsc::RecvTimeoutError, thread, time::Duration};

use yandex_upd_monitor::{MetricsReceiver, MetricsSender, RoomMetrics};

const TARGET_ADDRESS: &str = "127.0.0.1";
const TARGET_PORT: u16 = 8080;
const BROADCAST_PORT: u16 = 0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstration work of the Diptychs library");
    println!("=============================================");

    let receiver = MetricsReceiver::new(&format!("{TARGET_ADDRESS}:{TARGET_PORT}")).unwrap();
    let (_, metrics_tx) = receiver.start_with_channel();

    thread::sleep(Duration::from_millis(100));

    let sender_handle = thread::spawn(move || {
        let sender = MetricsSender::new(&format!("{TARGET_ADDRESS}:{BROADCAST_PORT}")).unwrap();
        println!("The sensor simulator is running. Sending data every 1 second...");

        for i in 0..5 {
            let metrics = RoomMetrics::random();
            if let Err(e) = sender.send_to(&metrics, &format!("{TARGET_ADDRESS}:{TARGET_PORT}")) {
                eprintln!("Send error: {e}");
            } else {
                println!("[SENSOR] Packet sent: {i}");
            }

            thread::sleep(Duration::from_secs(1));
        }
        println!("The sensor simulator has completed its work");
    });
    println!("The main thread is waiting for data...");

    let mut receive_count = 0;
    while receive_count < 5 {
        match metrics_tx.recv_timeout(Duration::from_secs(2)) {
            Ok((metrics, src_adr)) => {
                receive_count += 1;
                println!(
                    "[MAIN THREAD] Received from: {}, {:.1}°C, {:.1}%RH, {:.1}hPa, Door: {}, Vibration: {:.1}%",
                    src_adr,
                    metrics.temperature,
                    metrics.humidity,
                    metrics.pressure,
                    metrics.door_to_string(),
                    metrics.vibration_level,
                );
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("⏰ Timeout waiting for data...");
                continue;
            }
            Err(RecvTimeoutError::Disconnected) => {
                println!("🔌 The channel is closed");
                continue;
            }
        }
    }

    sender_handle.join().unwrap();

    println!("=============================================");
    println!("Demonstration completed successfully!");
    println!("Packets received: {receive_count}");

    Ok(())
}
