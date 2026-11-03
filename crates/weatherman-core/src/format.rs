//! UI-agnostic formatting and presentation helpers shared by the GUI and TUI.

use crate::types::WeatherCondition;
use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};

/// A semantic colour tone for a weather condition. Each front-end maps these
/// to its own concrete colours (ratatui `Color`, iced `Color`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionTone {
    Sunny,
    Cloudy,
    Wet,
    Storm,
    Snow,
    Fog,
    Neutral,
}

/// Map a [`WeatherCondition`] to a semantic [`ConditionTone`].
pub fn condition_tone(condition: &WeatherCondition) -> ConditionTone {
    match condition {
        WeatherCondition::Clear => ConditionTone::Sunny,
        WeatherCondition::Clouds => ConditionTone::Cloudy,
        WeatherCondition::Rain | WeatherCondition::Drizzle => ConditionTone::Wet,
        WeatherCondition::Thunderstorm | WeatherCondition::Squall | WeatherCondition::Tornado => {
            ConditionTone::Storm
        }
        WeatherCondition::Snow => ConditionTone::Snow,
        WeatherCondition::Fog | WeatherCondition::Mist | WeatherCondition::Haze => {
            ConditionTone::Fog
        }
        _ => ConditionTone::Neutral,
    }
}

/// Temperature unit label for the given units string ("metric"/"imperial"/"standard").
pub fn temp_unit_label(units: &str) -> &'static str {
    match units {
        "imperial" => "°F",
        "standard" => "K",
        _ => "°C",
    }
}

/// Wind-speed unit label for the given units string.
pub fn wind_unit_label(units: &str) -> &'static str {
    if units == "imperial" { "mph" } else { "m/s" }
}

/// Convert a compass bearing in degrees to a short cardinal label (N, NE, …).
pub fn wind_direction_label(degrees: u16) -> &'static str {
    match degrees % 360 {
        0..=22 | 338..=359 => "N",
        23..=67 => "NE",
        68..=112 => "E",
        113..=157 => "SE",
        158..=202 => "S",
        203..=247 => "SW",
        248..=292 => "W",
        _ => "NW",
    }
}

/// Convert a compass bearing in degrees to a unicode arrow.
pub fn wind_direction_arrow(degrees: u16) -> &'static str {
    match degrees % 360 {
        0..=22 | 338..=359 => "↑",
        23..=67 => "↗",
        68..=112 => "→",
        113..=157 => "↘",
        158..=202 => "↓",
        203..=247 => "↙",
        248..=292 => "←",
        _ => "↖",
    }
}

/// Human-friendly label for a day given its index in a forecast list.
pub fn day_label(index: usize, weekday: &str) -> String {
    match index {
        0 => "Today".to_string(),
        1 => "Tomorrow".to_string(),
        _ => weekday.to_string(),
    }
}

/// Full weekday name for a UTC date.
pub fn weekday_name(date: &DateTime<Utc>) -> &'static str {
    match date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

/// Whether an hour value (0-23) falls within daytime (06:00–17:59).
pub fn is_daytime(hour: u32) -> bool {
    (6..18).contains(&hour)
}

/// Convert a UTC timestamp to local time using a fixed hour offset derived from
/// a small built-in timezone table. This mirrors the original app behaviour and
/// avoids pulling in a full tz database. Unknown names are UTC.
pub fn convert_to_local(time: &DateTime<Utc>, timezone: &str) -> DateTime<Utc> {
    let hours_offset = match timezone {
        "America/New_York" | "EST" | "EDT" => -5,
        "America/Chicago" | "CST" | "CDT" => -6,
        "America/Denver" | "MST" | "MDT" => -7,
        "America/Los_Angeles" | "PST" | "PDT" => -8,
        "America/Anchorage" | "AKST" | "AKDT" => -9,
        "Pacific/Honolulu" | "HST" => -10,
        "Europe/London" | "GMT" | "BST" => 0,
        "Europe/Paris" | "Europe/Berlin" | "Europe/Rome" | "CET" | "CEST" => 1,
        "Europe/Athens" | "Europe/Istanbul" | "EET" | "EEST" => 2,
        "Asia/Dubai" => 4,
        "Asia/Kolkata" | "IST" => 5,
        "Asia/Shanghai" | "Asia/Singapore" => 8,
        "Asia/Tokyo" | "JST" => 9,
        "Australia/Sydney" | "AEST" | "AEDT" => 10,
        _ => 0,
    };
    // At the very ends of chrono's range the shift would overflow: keep UTC.
    time.checked_add_signed(chrono::Duration::hours(hours_offset))
        .unwrap_or(*time)
}

/// Format a timestamp as `HH:MM` in the given timezone.
pub fn format_local_time(time: &DateTime<Utc>, timezone: &str) -> String {
    let local = convert_to_local(time, timezone);
    format!("{:02}:{:02}", local.hour(), local.minute())
}

/// Format a date as `M/D` in the given timezone.
pub fn format_date_short(date: &DateTime<Utc>, timezone: &str) -> String {
    let local = convert_to_local(date, timezone);
    format!("{}/{}", local.month(), local.day())
}

/// Convert a probability in the 0..=1 range to an integer percentage.
pub fn pop_percent(pop: f64) -> u8 {
    (pop.clamp(0.0, 1.0) * 100.0).round() as u8
}

/// Human-readable risk band for a UV index value.
pub fn uv_label(uv: f64) -> &'static str {
    match uv.max(0.0) as u32 {
        0..=2 => "Low",
        3..=5 => "Moderate",
        6..=7 => "High",
        8..=10 => "Very High",
        _ => "Extreme",
    }
}
