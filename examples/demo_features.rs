// examples/demo_features.rs

use yandex_upd_monitor::RoomMetrics;

fn main() {
    println!("Demonstration of features");
    println!("===============================");

    let metrics = RoomMetrics::random();

    println!("Generated metrics:");
    println!("  Temperature: {:.1}°C", metrics.temperature);
    println!("  Humidity: {:.1}%", metrics.humidity);
    println!("  Pressure: {:.1}hPa", metrics.pressure);
    println!("  Door: {}", metrics.door_to_string());
    println!("  Vibration: {:.1}%", metrics.vibration_level);

    #[cfg(feature = "random")]
    println!("\nFeature 'random' is active");

    #[cfg(feature = "sqlite")]
    println!("Feature 'sqlite' is active");

    #[cfg(feature = "sqlite")]
    {
        println!("\nSQL request:");
        println!("{}", metrics.to_sql());
    }
}
