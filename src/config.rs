use std::collections::HashMap;
use std::fs;

// Define constants
pub const DEFAULT_WIDTH: i32 = 800;
pub const DEFAULT_HEIGHT: i32 = 600;

// Define Config struct
pub struct Config {
    pub width: i32,
    pub height: i32,
}

// Implement Config methods
impl Config {
    pub fn from_file(file_path: &str) -> Self {
        let mut config_map = HashMap::new();
        if let Ok(content) = fs::read_to_string(file_path) {
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    config_map.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        } else {
            eprintln!("Error reading file: {}", file_path);
        }

        let width = config_map
            .get("width")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(DEFAULT_WIDTH);

        let height = config_map
            .get("height")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(DEFAULT_HEIGHT);

        Self {
            width,
            height,
        }
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}
