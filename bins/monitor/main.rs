mod logger;

use yandex_upd_monitor::{
    MetricsReceiver,
    receiver::{MockReceiver, Receiver},
};

use crate::logger::{ConsoleLogger, Logger, MemoryLogger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_address = "127.0.0.1:8080";

    let console = Box::new(ConsoleLogger);
    let memory = Box::new(MemoryLogger::new());

    let loggers: Vec<Box<dyn Logger>> = vec![console.clone(), memory];

    console.log("Launch of a bank vault monitoring system");
    console.log(&format!("Listening to address: {bind_address}"));
    console.log("──────────────────────────────────────────────────");

    let receiver: Box<dyn Receiver> = if std::env::var("USE_MOCK").is_ok() {
        println!("Using mock receiver");
        Box::new(MockReceiver)
    } else {
        println!("Using metrics receiver");
        Box::new(MetricsReceiver::new(bind_address)?)
    };
    let (receiver_handle, metrics_rx) = receiver.start_with_channel();

    console.log("The monitoring system has been launched. Waiting for data.");
    console.log("Press Ctrl+C to stop");

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
                } else if metrics.vibration_level > 80.0 {
                    "⚠️ WARNING: High vibration"
                } else {
                    "✅ All is well"
                };

                for logger in loggers.iter() {
                    logger.log(&format!(
                        "[{}] Received from: {}, {:.1}C, {:.1}%RH, {:.1}hPa, Door: {}, Vibration: {:.1}%, Alert: {}",
                        metrics.formatted_time(),
                        src_addr,
                        metrics.temperature,
                        metrics.humidity,
                        metrics.pressure,
                        metrics.door_to_string(),
                        metrics.vibration_level,
                        alert_status,
                    ));
                }
            }
            Err(e) => {
                console.log(&format!("Error receiving metrics: {e}"));
                break;
            }
        }
    }

    let _ = receiver_handle.join();

    for logger in loggers.iter() {
        if let Some(mem) = logger.as_any().downcast_ref::<MemoryLogger>() {
            println!("Content MemoryLogger:");
            for entry in mem.get_entries() {
                println!("  {entry}");
            }
        }
    }

    console.log(&format!("Result: {total_received} data packets received"));

    Ok(())
}
