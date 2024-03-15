use serde::{Deserialize, Serialize};
use std::{fs, time::SystemTime};
use serde_json;


#[derive(Debug, Deserialize, Serialize)]
pub struct Monitor {
    pub name: String,
    #[serde(default)]
    pub monitor_id: Option<u32>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub result: Option<Result>,
    pub code: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub monitor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Result {
    pub value: i32,
    pub processed_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorData {
    pub monitors: Vec<Monitor>,
}

impl MonitorData {
    pub fn from_file(file_path: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let json_data = fs::read_to_string(file_path)?;
        serde_json::from_str(&json_data).map_err(|e| e.into())
    }

    pub fn with_random_results(mut self) -> Self {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        for monitor in &mut self.monitors {
            let random_value = rand::random::<i32>();
            monitor.result = Some(Result {
                value: random_value,
                processed_at: current_time as i64,
            });
        }
        self
    }
}
