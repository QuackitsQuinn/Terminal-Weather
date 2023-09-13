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

impl Default for Configuration{
    fn default() -> Configuration {
        Configuration {
            open_weather_key: String::new(),
            location: String::new(),
            is_location_ip_based: true,
            units: Units::Imperial
        }
    }
}
