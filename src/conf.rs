use std::path::Path;
use crate::units::Units;
pub const CONFIG_PATH: &str = "~/.term-weather.toml";
#[derive(Debug)]
pub struct Configuration {
    /// The API key for OpenWeatherMap
    pub open_weather_key: String,
    /// The location to get the weather for
    pub location: String,
    /// If true, the location is based on an IP address, will be updated each run
    pub is_location_ip_based: bool,
    /// The units to use for the weather
    pub units: Units
}

impl Configuration {
    /// Load the configuration from the default path
    pub fn load() -> Configuration {
        let config_path = Path::new(CONFIG_PATH);
        Configuration::load_from_path(config_path)
    }
    /// Load the configuration from a given path
    pub fn load_from_path(path: &Path) -> Configuration {
        let config = match std::fs::read_to_string(path) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Could not read configuration file, using defaults");
                String::new()
            }
        };
        let config: Configuration = toml::from_str(&config).unwrap_or_default();
        config
    }
}