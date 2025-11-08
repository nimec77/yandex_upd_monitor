use yandex_upd_monitor::MetricsSender;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let target_address = args.get(1).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");
    let interval_ms = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2000);

    println!("🚀 Launching a bank vault sensor simulator");
    println!("📍 Destination address: {target_address}");
    println!("⏱️ Sending interval: {interval_ms} ms");
    println!("──────────────────────────────────────────────────");

    let sender = MetricsSender::new("127.0.0.1:0")?;

    sender.start_broadcasting(target_address.to_string(), interval_ms)?;

    Ok(())
}
