use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMetrics {
    pub timestamp: u64,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub door_open: bool,
}

impl RoomMetrics {
    pub fn new(temperature: f32, humidity: f32, pressure: f32, door_open: bool) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            temperature,
            humidity,
            pressure,
            door_open,
        }
    }

    pub fn random() -> Self {
        use rand::Rng;

        let mut rng = rand::rng();

        Self::new(
            rng.random_range(18.0..25.0),
            rng.random_range(40.0..60.0),
            rng.random_range(980.0..1020.0),
            rng.random_bool(0.1),
        )
    }

    pub fn formatted_time(&self) -> String {
        format!("{}s", self.timestamp)
    }

    pub fn door_to_string(&self) -> &str {
        if self.door_open { "Open" } else { "Closed" }
    }
}
