use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumString;
use strum_macros::Display;

/// Represents configuration options for the weather forecasting tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub units: String,
    pub location: Option<String>,
    pub json_output: bool,
    pub animation_enabled: bool,
    pub detail_level: DetailLevel,
    pub no_charts: bool,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            units: "metric".to_string(),
            location: None,
            json_output: false,
            animation_enabled: true,
            detail_level: DetailLevel::Standard,
            no_charts: false,
        }
    }
}

/// Level of detail for weather output
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Display, EnumString,
)]
pub enum DetailLevel {
    #[strum(to_string = "Basic")]
    Basic,
    #[strum(to_string = "Standard")]
    Standard,
    #[strum(to_string = "Detailed")]
    Detailed,
    #[strum(to_string = "Debug")]
    Debug,
}

/// Represents weather condition categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeatherCondition {
    Clear,
    Clouds,
    Rain,
    Drizzle,
    Thunderstorm,
    Snow,
    Mist,
    Fog,
    Smoke,
    Haze,
    Dust,
    Sand,
    Ash,
    Squall,
    Tornado,
    Unknown,
}

impl WeatherCondition {
    /// Converts a string representation to a WeatherCondition
    ///
    /// This is different from FromStr trait implementation - it's a helper method
    /// that doesn't require Result handling
    #[allow(clippy::should_implement_trait)] // Infallible, unlike FromStr.
    pub fn from_str(condition: &str) -> Self {
        match condition.to_lowercase().as_str() {
            "clear" => WeatherCondition::Clear,
            "clouds" => WeatherCondition::Clouds,
            "rain" => WeatherCondition::Rain,
            "drizzle" => WeatherCondition::Drizzle,
            "thunderstorm" => WeatherCondition::Thunderstorm,
            "snow" => WeatherCondition::Snow,
            "mist" => WeatherCondition::Mist,
            "fog" => WeatherCondition::Fog,
            "smoke" => WeatherCondition::Smoke,
            "haze" => WeatherCondition::Haze,
            "dust" => WeatherCondition::Dust,
            "sand" => WeatherCondition::Sand,
            "ash" => WeatherCondition::Ash,
            "squall" => WeatherCondition::Squall,
            "tornado" => WeatherCondition::Tornado,
            _ => WeatherCondition::Unknown,
        }
    }

    pub fn get_emoji(&self) -> &'static str {
        match self {
            WeatherCondition::Clear => "☀️",
            WeatherCondition::Clouds => "☁️",
            WeatherCondition::Rain => "🌧️",
            WeatherCondition::Drizzle => "🌦️",
            WeatherCondition::Thunderstorm => "⛈️",
            WeatherCondition::Snow => "❄️",
            WeatherCondition::Mist => "🌫️",
            WeatherCondition::Fog => "🌫️",
            WeatherCondition::Smoke => "🌫️",
            WeatherCondition::Haze => "🌫️",
            WeatherCondition::Dust => "🌫️",
            WeatherCondition::Sand => "🌫️",
            WeatherCondition::Ash => "🌫️",
            WeatherCondition::Squall => "💨",
            WeatherCondition::Tornado => "🌪️",
            WeatherCondition::Unknown => "❓",
        }
    }
}

impl fmt::Display for WeatherCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            WeatherCondition::Clear => "Clear",
            WeatherCondition::Clouds => "Cloudy",
            WeatherCondition::Rain => "Rainy",
            WeatherCondition::Drizzle => "Drizzle",
            WeatherCondition::Thunderstorm => "Thunderstorm",
            WeatherCondition::Snow => "Snowy",
            WeatherCondition::Mist => "Misty",
            WeatherCondition::Fog => "Foggy",
            WeatherCondition::Smoke => "Smoky",
            WeatherCondition::Haze => "Hazy",
            WeatherCondition::Dust => "Dusty",
            WeatherCondition::Sand => "Sandy",
            WeatherCondition::Ash => "Ashy",
            WeatherCondition::Squall => "Squall",
            WeatherCondition::Tornado => "Tornado",
            WeatherCondition::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

/// Represents location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub region: Option<String>,
    pub state: Option<String>,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            country: "Unknown".to_string(),
            country_code: "UN".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            timezone: "UTC".to_string(),
            region: None,
            state: None,
        }
    }
}

/// Represents current weather data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub conditions: Vec<WeatherDescription>,
    pub main_condition: WeatherCondition,
    pub visibility: u32,
    pub clouds: u8,
    pub uv_index: f64,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub rain_last_hour: Option<f64>,
    pub snow_last_hour: Option<f64>,
    pub air_quality_index: Option<u8>,
}

/// Represents detailed weather description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherDescription {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

/// Represents hourly forecast data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub conditions: Vec<WeatherDescription>,
    pub main_condition: WeatherCondition,
    pub pop: f64, // Probability of precipitation
    pub visibility: u32,
    pub clouds: u8,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
}

/// Represents daily forecast data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyForecast {
    pub date: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub temp_morning: f64,
    pub temp_day: f64,
    pub temp_evening: f64,
    pub temp_night: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub feels_like_day: f64,
    pub feels_like_night: f64,
    pub pressure: u32,
    pub humidity: u8,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub conditions: Vec<WeatherDescription>,
    pub main_condition: WeatherCondition,
    pub clouds: u8,
    pub pop: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
    pub uv_index: f64,
}

/// Represents a complete weather forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forecast {
    pub current: Option<CurrentWeather>,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
    pub timezone_offset: i32,
    pub units: String,
}

/// A snapshot of "right now" weather, implemented by both [`CurrentWeather`]
/// and [`HourlyForecast`] so presentation code can treat them uniformly.
pub trait CurrentConditions {
    /// Observation timestamp (UTC).
    fn observed_at(&self) -> DateTime<Utc>;
    /// Air temperature in the configured units.
    fn temperature(&self) -> f64;
    /// "Feels like" temperature in the configured units.
    fn feels_like(&self) -> f64;
    /// Relative humidity as a percentage.
    fn humidity(&self) -> u8;
    /// Wind speed in the configured units.
    fn wind_speed(&self) -> f64;
    /// Wind bearing in degrees.
    fn wind_direction(&self) -> u16;
    /// The dominant weather condition.
    fn condition(&self) -> WeatherCondition;
}

impl CurrentConditions for CurrentWeather {
    fn observed_at(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn temperature(&self) -> f64 {
        self.temperature
    }
    fn feels_like(&self) -> f64 {
        self.feels_like
    }
    fn humidity(&self) -> u8 {
        self.humidity
    }
    fn wind_speed(&self) -> f64 {
        self.wind_speed
    }
    fn wind_direction(&self) -> u16 {
        self.wind_direction
    }
    fn condition(&self) -> WeatherCondition {
        self.main_condition
    }
}

impl CurrentConditions for HourlyForecast {
    fn observed_at(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn temperature(&self) -> f64 {
        self.temperature
    }
    fn feels_like(&self) -> f64 {
        self.feels_like
    }
    fn humidity(&self) -> u8 {
        self.humidity
    }
    fn wind_speed(&self) -> f64 {
        self.wind_speed
    }
    fn wind_direction(&self) -> u16 {
        self.wind_direction
    }
    fn condition(&self) -> WeatherCondition {
        self.main_condition
    }
}
