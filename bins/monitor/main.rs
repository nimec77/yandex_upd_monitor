use yandex_upd_monitor::MetricsReceiver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_address = "127.0.0.1:8080";

    println!("Launch of a bank vault monitoring system");
    println!("Listening to address: {bind_address}");
    println!("──────────────────────────────────────────────────");

    let receiver = MetricsReceiver::new(bind_address)?;
    let (receiver_handle, metrics_rx) = receiver.start_with_channel();

    println!("The monitoring system has been launched. Waiting for data.");
    println!("Press Ctrl+C to stop");

    let mut total_received = 0;

    loop {
        match metrics_rx.recv() {
            Ok((metrics, src_addr)) => {
                total_received += 1;

                let alert_status = if metrics.door_open {
                    "🚨 ALARM: DOOR OPEN!"
                } else if metrics.temperature > 25.0 {
                    "⚠️ WARNING: High temperature"
                } else if metrics.humidity > 60.0 {
                    "⚠️ WARNING: High humidity"
                } else if metrics.pressure < 980.0 || metrics.pressure > 1020.0 {
                    "⚠️ WARNING: Pressure out of range"
                } else {
                    "✅ All is well"
                };

                println!(
                    "[{}] Received from: {}, {:.1}C, {:.1}%RH, {:.1}hPa, Door: {}, Alert: {}",
                    metrics.formatted_time(),
                    src_addr,
                    metrics.temperature,
                    metrics.humidity,
                    metrics.pressure,
                    metrics.door_to_string(),
                    alert_status,
                );
            }
            Err(e) => {
                eprintln!("Error receiving metrics: {e}");
                break;
            }
        }
    }

    let _ = receiver_handle.join();
    println!("Result: {total_received} data packets received");

    Ok(())
}
