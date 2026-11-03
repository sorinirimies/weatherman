//! Print a full forecast using only `weatherman-core` — no GUI or TUI.
//!
//! Run:
//!   cargo run -p weatherman-core --example report -- Berlin
//!   cargo run -p weatherman-core --example report            # auto-detect (IP)
//!
//! This demonstrates the library end-to-end: geocoding, fetching, and building
//! the UI-agnostic `ForecastView` whose fields are already display-ready.

use weatherman_core::{ForecastView, WeatherConfig, load_report};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let query = std::env::args().nth(1);
    let config = WeatherConfig::default();

    println!("⛅ weatherman-core — fetching forecast…\n");

    let report = load_report(&config, query.as_deref()).await?;
    let view = ForecastView::build(
        report.current.as_ref(),
        &report.hourly,
        &report.daily,
        &report.location,
        &config,
    );

    if let Some(c) = &view.current {
        println!("📍 {}", c.location_line);
        println!(
            "{}  {}  {}   (feels {})",
            c.emoji, c.condition, c.temperature, c.feels_like
        );
        println!(
            "   humidity {}   wind {}   local time {}\n",
            c.humidity, c.wind, c.local_time
        );
    }

    println!("🕓 Next 24 hours");
    for h in &view.hours {
        println!(
            "   {}  {}  {:>7}   💧 {:>3}%   {}",
            h.time, h.emoji, h.temperature, h.pop_percent, h.condition
        );
    }

    println!("\n📅 7-day forecast");
    for d in &view.days {
        println!(
            "   {:<10} {:<6} {}  {:<13} {:>13}   💧 {:>3}%",
            d.label, d.date, d.emoji, d.condition, d.temp_high_low, d.pop_percent
        );
    }

    Ok(())
}
