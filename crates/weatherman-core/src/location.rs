use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

use crate::types::Location;

/// Handles location detection and queries
#[derive(Debug, Clone)]
pub struct LocationService {
    client: Client,
}

impl LocationService {
    /// Create a new location service
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Get location from user's IP address
    pub async fn get_location_from_ip(&self) -> Result<Location> {
        // Try multiple IP geolocation services for redundancy
        let services = vec![
            "https://ipapi.co/json/",
            "https://ipinfo.io/json",
            "https://freegeoip.app/json/",
            "https://extreme-ip-lookup.com/json/",
        ];

        for service_url in services {
            match self.client.get(service_url).send().await {
                Ok(response) => {
                    if let Ok(json) = response.json::<Value>().await
                        && let Some(location) = self.parse_location_from_json(json)
                    {
                        return Ok(location);
                    }
                }
                Err(_) => continue,
            }
        }

        // Fallback to a default location if all services fail
        Err(anyhow::anyhow!("Could not detect location from IP address"))
    }

    /// Get location by name (city, address, etc)
    pub async fn get_location_by_name(&self, location_name: &str) -> Result<Location> {
        // Use OpenStreetMap/Nominatim for geocoding
        let url = format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
            urlencoding::encode(location_name)
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "weatherman/0.0.6")
            .send()
            .await?;

        let json: Value = response.json().await?;

        if let Some(place) = json.as_array().and_then(|arr| arr.first()) {
            let lat = place["lat"]
                .as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            let lon = place["lon"]
                .as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            let name = place["display_name"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();

            // Get more details using reverse geocoding
            return self.get_detailed_location(lat, lon, Some(name)).await;
        }

        Err(anyhow::anyhow!(
            "Could not find location: {}",
            location_name
        ))
    }

    /// Get detailed location info from coordinates
    async fn get_detailed_location(
        &self,
        lat: f64,
        lon: f64,
        name_override: Option<String>,
    ) -> Result<Location> {
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json",
            lat, lon
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "weatherman/0.0.6")
            .send()
            .await?;

        let json: Value = response.json().await?;

        let address = &json["address"];

        let city = address["city"]
            .as_str()
            .or_else(|| address["town"].as_str())
            .or_else(|| address["village"].as_str())
            .or_else(|| address["hamlet"].as_str())
            .unwrap_or("Unknown");

        let country = address["country"].as_str().unwrap_or("Unknown");
        let country_code = address["country_code"]
            .as_str()
            .map(|s| s.to_uppercase())
            .unwrap_or_else(|| "UN".to_string());

        let state = address["state"].as_str().map(|s| s.to_string());
        let region = address["region"].as_str().map(|s| s.to_string());

        // Get timezone from coordinates
        let timezone = self.get_timezone(lat, lon).await?;

        Ok(Location {
            name: name_override.unwrap_or_else(|| city.to_string()),
            country: country.to_string(),
            country_code,
            latitude: lat,
            longitude: lon,
            timezone,
            region,
            state,
        })
    }

    /// Get timezone from coordinates
    async fn get_timezone(&self, lat: f64, lon: f64) -> Result<String> {
        let url = format!(
            "http://api.geonames.org/timezoneJSON?lat={}&lng={}&username=weatherman",
            lat, lon
        );

        if let Ok(response) = self.client.get(&url).send().await
            && let Ok(json) = response.json::<Value>().await
            && let Some(tz) = json["timezoneId"].as_str()
        {
            return Ok(tz.to_string());
        }

        // Fallback to a simple timezone estimation
        Ok("UTC".to_string())
    }

    /// Parse location from various IP geolocation service responses
    fn parse_location_from_json(&self, json: Value) -> Option<Location> {
        let latitude = json["lat"]
            .as_f64()
            .or_else(|| json["latitude"].as_f64())
            .or_else(|| {
                json["location"]
                    .as_object()
                    .and_then(|loc| loc["lat"].as_f64())
            })?;

        let longitude = json["lon"]
            .as_f64()
            .or_else(|| json["longitude"].as_f64())
            .or_else(|| {
                json["location"]
                    .as_object()
                    .and_then(|loc| loc["lng"].as_f64())
            })?;

        let city = json["city"]
            .as_str()
            .or_else(|| json["region_name"].as_str())
            .unwrap_or("Unknown");

        let country = json["country_name"]
            .as_str()
            .or_else(|| json["country"].as_str())
            .unwrap_or("Unknown");

        let country_code = json["country_code"]
            .as_str()
            .or_else(|| json["countryCode"].as_str())
            .unwrap_or("UN")
            .to_uppercase();

        let region = json["region"]
            .as_str()
            .or_else(|| json["regionName"].as_str())
            .map(|s| s.to_string());

        let timezone = json["timezone"].as_str().unwrap_or("UTC").to_string();

        Some(Location {
            name: city.to_string(),
            country: country.to_string(),
            country_code,
            latitude,
            longitude,
            timezone,
            region,
            state: None,
        })
    }
}

impl Default for LocationService {
    fn default() -> Self {
        Self::new()
    }
}
