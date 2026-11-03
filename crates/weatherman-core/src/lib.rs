//! # weatherman-core
//!
//! Framework-free weather logic shared by the Iced GUI and Ratatui TUI, and
//! usable as a **drop-in weather API provider library** on its own.
//!
//! | Module | What lives here |
//! |--------|-----------------|
//! | [`types`] | Domain models — conditions, locations, current/hourly/daily forecasts, config |
//! | [`forecaster`] | [`WeatherProvider`] trait + Open-Meteo implementation ([`WeatherForecaster`]) |
//! | [`location`] | IP + name geocoding ([`LocationService`]) |
//! | [`api`] | One-call orchestration ([`load_report`]) returning a [`WeatherReport`] |
//! | [`presentation`] | UI-agnostic view-model ([`ForecastView`]) + the [`tone_color_fn!`] macro |
//! | [`mod@format`] | UI-agnostic formatting helpers (units, wind, tones, local time) |
//!
//! This crate has NO GUI or TUI dependencies.
//!
//! ## Quick start
//!
//! ```no_run
//! use weatherman_core::{load_report, ForecastView, WeatherConfig};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let report = load_report(&WeatherConfig::default(), Some("Berlin")).await?;
//! let view = ForecastView::build(
//!     report.current.as_ref(),
//!     &report.hourly,
//!     &report.daily,
//!     &report.location,
//!     &WeatherConfig::default(),
//! );
//! println!("{} days ready to render", view.days.len());
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod forecaster;
pub mod format;
pub mod location;
pub mod persistence;
pub mod presentation;
pub mod types;

// Convenience re-exports — the public surface most consumers need.
pub use api::{WeatherReport, load_report, resolve_location};
pub use forecaster::{
    WeatherForecaster, WeatherProvider, weather_description_from_wmo, wmo_code_to_condition,
};
pub use format::{
    ConditionTone, condition_tone, convert_to_local, day_label, format_date_short,
    format_local_time, is_daytime, pop_percent, temp_unit_label, uv_label, weekday_name,
    wind_direction_arrow, wind_direction_label, wind_unit_label,
};
pub use location::LocationService;
pub use persistence::{AppSettings, load_settings, save_settings, settings_path};
pub use presentation::{
    CurrentView, DAILY_LIMIT, DayDetail, DayRow, ForecastView, HOURLY_LIMIT, HourRow,
};
pub use types::{
    CurrentConditions, CurrentWeather, DailyForecast, DetailLevel, Forecast, HourlyForecast,
    Location, WeatherCondition, WeatherConfig, WeatherDescription,
};
