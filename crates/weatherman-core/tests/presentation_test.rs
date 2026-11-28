use chrono::{Duration, TimeZone, Utc};
use weatherman_core::{
    ConditionTone, CurrentView, DailyForecast, DayRow, ForecastView, HourRow, HourlyForecast,
    Location, WeatherCondition, WeatherConfig, WeatherDescription,
};

fn location() -> Location {
    Location {
        name: "Testville".into(),
        country: "Testland".into(),
        country_code: "TL".into(),
        latitude: 0.0,
        longitude: 0.0,
        timezone: "UTC".into(),
        region: None,
        state: None,
    }
}

fn hourly(n: usize) -> Vec<HourlyForecast> {
    let base = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    (0..n)
        .map(|i| HourlyForecast {
            timestamp: base + Duration::hours(i as i64),
            temperature: 20.5,
            feels_like: 19.4,
            humidity: 55,
            pressure: 1013,
            wind_speed: 3.2,
            wind_direction: 90,
            conditions: vec![WeatherDescription {
                id: 0,
                main: "Clear".into(),
                description: "clear sky".into(),
                icon: "01d".into(),
            }],
            main_condition: WeatherCondition::Clear,
            pop: 0.12,
            visibility: 10000,
            clouds: 0,
            rain: None,
            snow: None,
        })
        .collect()
}

fn daily(n: usize) -> Vec<DailyForecast> {
    let base = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    (0..n)
        .map(|i| DailyForecast {
            date: base + Duration::days(i as i64),
            sunrise: base,
            sunset: base,
            temp_morning: 12.0,
            temp_day: 22.0,
            temp_evening: 18.0,
            temp_night: 10.0,
            temp_min: 10.4,
            temp_max: 24.6,
            feels_like_day: 22.0,
            feels_like_night: 9.0,
            pressure: 1013,
            humidity: 50,
            wind_speed: 4.0,
            wind_direction: 180,
            conditions: vec![],
            main_condition: WeatherCondition::Rain,
            clouds: 40,
            pop: 0.6,
            rain: Some(1.2),
            snow: None,
            uv_index: 3.0,
        })
        .collect()
}

#[test]
fn current_view_formats_metric() {
    let cfg = WeatherConfig::default();
    let cv = CurrentView::build(&hourly(1)[0], &location(), &cfg);
    assert_eq!(cv.emoji, "☀️");
    assert_eq!(cv.condition, "Clear");
    assert_eq!(cv.tone, ConditionTone::Sunny);
    assert_eq!(cv.location_line, "Testville, Testland");
    assert_eq!(cv.temperature, "20.5°C");
    assert_eq!(cv.feels_like, "19.4°C");
    assert_eq!(cv.humidity, "55%");
    assert_eq!(cv.wind, "3.2 m/s E");
    assert_eq!(cv.local_time, "00:00");
}

#[test]
fn current_view_formats_imperial() {
    let cfg = WeatherConfig {
        units: "imperial".into(),
        ..Default::default()
    };
    let cv = CurrentView::build(&hourly(1)[0], &location(), &cfg);
    assert_eq!(cv.temperature, "20.5°F");
    assert_eq!(cv.wind, "3.2 mph E");
}

#[test]
fn hour_row_formats() {
    let cfg = WeatherConfig::default();
    let hr = HourRow::build(&hourly(1)[0], &location(), &cfg);
    assert_eq!(hr.time, "00:00");
    assert_eq!(hr.temperature, "20.5°C");
    assert_eq!(hr.pop_percent, 12);
    assert_eq!(hr.tone, ConditionTone::Sunny);
}

#[test]
fn day_row_labels_and_hi_lo() {
    let cfg = WeatherConfig::default();
    let days = daily(3);
    let d0 = DayRow::build(0, &days[0], &location(), &cfg);
    let d1 = DayRow::build(1, &days[1], &location(), &cfg);
    let d2 = DayRow::build(2, &days[2], &location(), &cfg);
    assert_eq!(d0.label, "Today");
    assert_eq!(d1.label, "Tomorrow");
    assert_eq!(d2.label, "Wednesday"); // 2024-01-03 is a Wednesday
    assert_eq!(d0.date, "01/01");
    assert_eq!(d0.temp_high_low, "25°C / 10°C"); // rounded
    assert_eq!(d0.pop_percent, 60);
    assert_eq!(d0.tone, ConditionTone::Wet);
}

#[test]
fn day_row_detail_is_formatted() {
    let cfg = WeatherConfig::default();
    let days = daily(1);
    let d = DayRow::build(0, &days[0], &location(), &cfg);
    // Derived day-part temperatures (see sample_daily below).
    assert_eq!(d.detail.temp_day, "22°C");
    assert_eq!(d.detail.temp_night, "10°C");
    assert_eq!(d.detail.feels_like_day, "22°C");
    assert_eq!(d.detail.uv_index, "3.0 (Moderate)");
    // pop 0.6 + rain 1.2mm.
    assert!(d.detail.precipitation.contains("60% chance"));
    assert!(d.detail.precipitation.contains("1.2 mm rain"));
    assert!(d.detail.wind.starts_with("4.0 m/s"));
}

#[test]
fn forecast_view_caps_and_promotes_current() {
    let cfg = WeatherConfig::default();
    // No explicit current -> first hourly promoted.
    let view = ForecastView::build(
        None::<&HourlyForecast>,
        &hourly(48),
        &daily(10),
        &location(),
        &cfg,
    );
    assert!(view.current.is_some());
    assert_eq!(view.hours.len(), 24); // capped
    assert_eq!(view.days.len(), 7); // capped
}

#[test]
fn forecast_view_empty_has_no_current() {
    let cfg = WeatherConfig::default();
    let view = ForecastView::build(None::<&HourlyForecast>, &[], &[], &location(), &cfg);
    assert!(view.current.is_none());
    assert!(view.hours.is_empty());
    assert!(view.days.is_empty());
}

// Exercises the tone_color_fn! macro end-to-end with a trivial target type.
weatherman_core::tone_color_fn!(fn tone_char -> char {
    sunny: 'S',
    cloudy: 'C',
    wet: 'W',
    storm: 'T',
    snow: 'N',
    fog: 'F',
    neutral: '-',
});

#[test]
fn tone_color_fn_macro_maps_all_tones() {
    assert_eq!(tone_char(ConditionTone::Sunny), 'S');
    assert_eq!(tone_char(ConditionTone::Cloudy), 'C');
    assert_eq!(tone_char(ConditionTone::Wet), 'W');
    assert_eq!(tone_char(ConditionTone::Storm), 'T');
    assert_eq!(tone_char(ConditionTone::Snow), 'N');
    assert_eq!(tone_char(ConditionTone::Fog), 'F');
    assert_eq!(tone_char(ConditionTone::Neutral), '-');
}

#[test]
fn hours_start_at_next_full_hour_after_current() {
    let cfg = WeatherConfig::default();
    let hours = hourly(72);
    // 2024-01-01 07:30 -> strip should start at 08:00.
    let mut now = hours[7].clone();
    now.timestamp = Utc.with_ymd_and_hms(2024, 1, 1, 7, 30, 0).unwrap();
    let view = ForecastView::build(Some(&now), &hours, &daily(7), &location(), &cfg);
    assert_eq!(view.hours.len(), 24);
    assert_eq!(view.hours[0].time, "08:00");

    // Exactly on the hour -> that hour is kept.
    let mut now = hours[7].clone();
    now.timestamp = Utc.with_ymd_and_hms(2024, 1, 1, 7, 0, 0).unwrap();
    let view = ForecastView::build(Some(&now), &hours, &daily(7), &location(), &cfg);
    assert_eq!(view.hours[0].time, "07:00");
}
