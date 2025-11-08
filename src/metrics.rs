use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMetrics {
    pub timestamp: u64,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub door_open: bool,
    pub vibration_level: f32,
}

impl RoomMetrics {
    pub fn new(
        temperature: f32,
        humidity: f32,
        pressure: f32,
        door_open: bool,
        vibration_level: f32,
    ) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            temperature,
            humidity,
            pressure,
            door_open,
            vibration_level,
        }
    }

    pub fn random() -> Self {
        use rand::Rng;

        let mut rng = rand::rng();

        Self::new(
            rng.random_range(18.0..26.0),
            rng.random_range(40.0..60.0),
            rng.random_range(980.0..1030.0),
            rng.random_bool(0.1),
            rng.random_range(30.0..100.0),
        )
    }

    pub fn formatted_time(&self) -> String {
        format!("{}s", self.timestamp)
    }

    pub fn door_to_string(&self) -> &str {
        if self.door_open { "Open" } else { "Closed" }
    }
}
