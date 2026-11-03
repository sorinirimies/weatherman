//! UI-agnostic **presentation view-model**.
//!
//! These types pre-compute every display string (temperatures, wind, times,
//! probabilities, labels) and the semantic [`ConditionTone`] for each element,
//! so that the GUI and TUI only have to map already-formatted data onto their
//! own widgets. All formatting rules live here — in one place — which keeps the
//! two front-ends pixel-for-pixel consistent and makes them trivially testable.

use crate::format::{
    ConditionTone, condition_tone, convert_to_local, day_label, format_local_time, pop_percent,
    temp_unit_label, uv_label, weekday_name, wind_direction_label, wind_unit_label,
};
use crate::types::{CurrentConditions, DailyForecast, HourlyForecast, Location, WeatherConfig};
use chrono::Timelike;

/// Maximum number of hourly rows a [`ForecastView`] exposes.
pub const HOURLY_LIMIT: usize = 24;
/// Maximum number of daily rows a [`ForecastView`] exposes.
pub const DAILY_LIMIT: usize = 7;

/// Pre-formatted "current conditions" block.
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentView {
    pub emoji: &'static str,
    pub condition: String,
    pub tone: ConditionTone,
    pub location_line: String,
    pub temperature: String,
    pub feels_like: String,
    pub humidity: String,
    pub wind: String,
    pub local_time: String,
}

impl CurrentView {
    /// Build from anything implementing [`CurrentConditions`].
    pub fn build<C: CurrentConditions>(
        current: &C,
        location: &Location,
        config: &WeatherConfig,
    ) -> Self {
        let temp_unit = temp_unit_label(&config.units);
        let wind_unit = wind_unit_label(&config.units);
        let local = convert_to_local(&current.observed_at(), &location.timezone);
        let condition = current.condition();

        Self {
            emoji: condition.get_emoji(),
            condition: condition.to_string(),
            tone: condition_tone(&condition),
            location_line: format!("{}, {}", location.name, location.country),
            temperature: format!("{:.1}{}", current.temperature(), temp_unit),
            feels_like: format!("{:.1}{}", current.feels_like(), temp_unit),
            humidity: format!("{}%", current.humidity()),
            wind: format!(
                "{:.1} {} {}",
                current.wind_speed(),
                wind_unit,
                wind_direction_label(current.wind_direction())
            ),
            local_time: format!("{:02}:{:02}", local.hour(), local.minute()),
        }
    }
}

/// Pre-formatted single-hour row.
#[derive(Debug, Clone, PartialEq)]
pub struct HourRow {
    pub time: String,
    pub emoji: &'static str,
    pub tone: ConditionTone,
    pub condition: String,
    pub temperature: String,
    pub pop_percent: u8,
}

impl HourRow {
    /// Build a row from an hourly forecast entry.
    pub fn build(hour: &HourlyForecast, location: &Location, config: &WeatherConfig) -> Self {
        let temp_unit = temp_unit_label(&config.units);
        let local = convert_to_local(&hour.timestamp, &location.timezone);
        Self {
            time: format!("{:02}:00", local.hour()),
            emoji: hour.main_condition.get_emoji(),
            tone: condition_tone(&hour.main_condition),
            condition: hour.main_condition.to_string(),
            temperature: format!("{:.1}{}", hour.temperature, temp_unit),
            pop_percent: pop_percent(hour.pop),
        }
    }
}

/// Extended, pre-formatted details for a single day, shown when a day row is
/// expanded. Only fields Open-Meteo actually provides for daily data are
/// included (humidity/pressure are omitted as they are not real for daily).
#[derive(Debug, Clone, PartialEq)]
pub struct DayDetail {
    pub temp_morning: String,
    pub temp_day: String,
    pub temp_evening: String,
    pub temp_night: String,
    pub feels_like_day: String,
    pub feels_like_night: String,
    pub sunrise: String,
    pub sunset: String,
    pub wind: String,
    pub uv_index: String,
    pub precipitation: String,
}

impl DayDetail {
    fn build(day: &DailyForecast, location: &Location, config: &WeatherConfig) -> Self {
        let t = temp_unit_label(&config.units);
        let wind_unit = wind_unit_label(&config.units);
        let tz = &location.timezone;

        let mut precipitation = format!("{}% chance", pop_percent(day.pop));
        if let Some(rain) = day.rain
            && rain > 0.0
        {
            precipitation.push_str(&format!(" · {:.1} mm rain", rain));
        }
        if let Some(snow) = day.snow
            && snow > 0.0
        {
            precipitation.push_str(&format!(" · {:.1} mm snow", snow));
        }

        Self {
            temp_morning: format!("{:.0}{}", day.temp_morning, t),
            temp_day: format!("{:.0}{}", day.temp_day, t),
            temp_evening: format!("{:.0}{}", day.temp_evening, t),
            temp_night: format!("{:.0}{}", day.temp_night, t),
            feels_like_day: format!("{:.0}{}", day.feels_like_day, t),
            feels_like_night: format!("{:.0}{}", day.feels_like_night, t),
            sunrise: format_local_time(&day.sunrise, tz),
            sunset: format_local_time(&day.sunset, tz),
            wind: format!(
                "{:.1} {} {}",
                day.wind_speed,
                wind_unit,
                wind_direction_label(day.wind_direction)
            ),
            uv_index: format!("{:.1} ({})", day.uv_index, uv_label(day.uv_index)),
            precipitation,
        }
    }
}

/// Pre-formatted single-day row.
#[derive(Debug, Clone, PartialEq)]
pub struct DayRow {
    pub label: String,
    pub date: String,
    pub emoji: &'static str,
    pub tone: ConditionTone,
    pub condition: String,
    pub temp_high_low: String,
    pub pop_percent: u8,
    /// Extended details for the expandable panel.
    pub detail: DayDetail,
}

impl DayRow {
    /// Build a row from a daily forecast entry at position `index`.
    pub fn build(
        index: usize,
        day: &DailyForecast,
        location: &Location,
        config: &WeatherConfig,
    ) -> Self {
        let temp_unit = temp_unit_label(&config.units);
        let local = convert_to_local(&day.date, &location.timezone);
        Self {
            label: day_label(index, weekday_name(&local)),
            date: local.format("%m/%d").to_string(),
            emoji: day.main_condition.get_emoji(),
            tone: condition_tone(&day.main_condition),
            condition: day.main_condition.to_string(),
            temp_high_low: format!(
                "{:.0}{} / {:.0}{}",
                day.temp_max, temp_unit, day.temp_min, temp_unit
            ),
            pop_percent: pop_percent(day.pop),
            detail: DayDetail::build(day, location, config),
        }
    }
}

/// The full, ready-to-render forecast for one location.
#[derive(Debug, Clone, PartialEq)]
pub struct ForecastView {
    pub current: Option<CurrentView>,
    pub hours: Vec<HourRow>,
    pub days: Vec<DayRow>,
}

impl ForecastView {
    /// Build the whole view-model from the raw pieces. `current` may be `None`,
    /// in which case the first hourly entry is promoted to the current block.
    pub fn build<C: CurrentConditions>(
        current: Option<&C>,
        hourly: &[HourlyForecast],
        daily: &[DailyForecast],
        location: &Location,
        config: &WeatherConfig,
    ) -> Self {
        let current_view = match current {
            Some(c) => Some(CurrentView::build(c, location, config)),
            None => hourly
                .first()
                .map(|h| CurrentView::build(h, location, config)),
        };

        let hours = hourly
            .iter()
            .take(HOURLY_LIMIT)
            .map(|h| HourRow::build(h, location, config))
            .collect();

        let days = daily
            .iter()
            .take(DAILY_LIMIT)
            .enumerate()
            .map(|(i, d)| DayRow::build(i, d, location, config))
            .collect();

        Self {
            current: current_view,
            hours,
            days,
        }
    }
}

/// Generate a `ConditionTone -> Color` function from a colour table, so each
/// front-end defines its palette once without repeating the (identical) match
/// structure. The generated function pairs naturally with the `tone` field on
/// [`CurrentView`], [`HourRow`] and [`DayRow`].
///
/// ```ignore
/// weatherman_core::tone_color_fn!(pub fn tone_color -> ratatui::style::Color {
///     sunny:   ratatui::style::Color::Yellow,
///     cloudy:  ratatui::style::Color::Gray,
///     wet:     ratatui::style::Color::Blue,
///     storm:   ratatui::style::Color::Magenta,
///     snow:    ratatui::style::Color::White,
///     fog:     ratatui::style::Color::DarkGray,
///     neutral: ratatui::style::Color::Gray,
/// });
/// ```
#[macro_export]
macro_rules! tone_color_fn {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident -> $ty:ty {
            sunny:   $sunny:expr,
            cloudy:  $cloudy:expr,
            wet:     $wet:expr,
            storm:   $storm:expr,
            snow:    $snow:expr,
            fog:     $fog:expr,
            neutral: $neutral:expr $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis fn $name(tone: $crate::ConditionTone) -> $ty {
            match tone {
                $crate::ConditionTone::Sunny => $sunny,
                $crate::ConditionTone::Cloudy => $cloudy,
                $crate::ConditionTone::Wet => $wet,
                $crate::ConditionTone::Storm => $storm,
                $crate::ConditionTone::Snow => $snow,
                $crate::ConditionTone::Fog => $fog,
                $crate::ConditionTone::Neutral => $neutral,
            }
        }
    };
}
