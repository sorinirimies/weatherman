use chrono::{TimeZone, Utc};
use weatherman_core::{
    ConditionTone, WeatherCondition, condition_tone, day_label, pop_percent, temp_unit_label,
    weekday_name, wind_direction_arrow, wind_direction_label, wind_unit_label,
};

#[test]
fn test_temp_unit_label() {
    assert_eq!(temp_unit_label("metric"), "°C");
    assert_eq!(temp_unit_label("imperial"), "°F");
    assert_eq!(temp_unit_label("standard"), "K");
    assert_eq!(temp_unit_label("anything-else"), "°C");
}

#[test]
fn test_wind_unit_label() {
    assert_eq!(wind_unit_label("imperial"), "mph");
    assert_eq!(wind_unit_label("metric"), "m/s");
}

#[test]
fn test_wind_direction_label() {
    assert_eq!(wind_direction_label(0), "N");
    assert_eq!(wind_direction_label(45), "NE");
    assert_eq!(wind_direction_label(90), "E");
    assert_eq!(wind_direction_label(180), "S");
    assert_eq!(wind_direction_label(270), "W");
    assert_eq!(wind_direction_label(350), "N");
    assert_eq!(wind_direction_label(720), "N"); // wraps
}

#[test]
fn test_wind_direction_arrow() {
    assert_eq!(wind_direction_arrow(0), "↑");
    assert_eq!(wind_direction_arrow(90), "→");
    assert_eq!(wind_direction_arrow(180), "↓");
    assert_eq!(wind_direction_arrow(270), "←");
}

#[test]
fn test_day_label() {
    assert_eq!(day_label(0, "Monday"), "Today");
    assert_eq!(day_label(1, "Monday"), "Tomorrow");
    assert_eq!(day_label(2, "Monday"), "Monday");
}

#[test]
fn test_condition_tone() {
    assert_eq!(
        condition_tone(&WeatherCondition::Clear),
        ConditionTone::Sunny
    );
    assert_eq!(
        condition_tone(&WeatherCondition::Clouds),
        ConditionTone::Cloudy
    );
    assert_eq!(condition_tone(&WeatherCondition::Rain), ConditionTone::Wet);
    assert_eq!(
        condition_tone(&WeatherCondition::Thunderstorm),
        ConditionTone::Storm
    );
    assert_eq!(condition_tone(&WeatherCondition::Snow), ConditionTone::Snow);
    assert_eq!(condition_tone(&WeatherCondition::Fog), ConditionTone::Fog);
}

#[test]
fn test_pop_percent() {
    assert_eq!(pop_percent(0.0), 0);
    assert_eq!(pop_percent(0.5), 50);
    assert_eq!(pop_percent(1.0), 100);
    assert_eq!(pop_percent(1.5), 100); // clamped
    assert_eq!(pop_percent(-0.2), 0); // clamped
}

#[test]
fn test_weekday_name() {
    let date = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap(); // Monday
    assert_eq!(weekday_name(&date), "Monday");
}

#[test]
fn local_time_at_the_end_of_the_date_range_does_not_panic() {
    let last = chrono::DateTime::<chrono::Utc>::MAX_UTC;
    assert_eq!(
        weatherman_core::format::format_local_time(&last, "Asia/Tokyo").len(),
        5
    );
    let first = chrono::DateTime::<chrono::Utc>::MIN_UTC;
    assert_eq!(
        weatherman_core::format::format_local_time(&first, "America/New_York").len(),
        5
    );
}
