use chrono::{Duration, Utc};
use tui_nuances::Theme;
use weatherman_core::{
    ConditionTone, DailyForecast, ForecastView, HourlyForecast, Location, WeatherCondition,
    WeatherConfig, WeatherDescription,
};
use weatherman_tui::view;

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
    let base = Utc::now();
    (0..n)
        .map(|i| HourlyForecast {
            timestamp: base + Duration::hours(i as i64),
            temperature: 20.0,
            feels_like: 19.0,
            humidity: 55,
            pressure: 1013,
            wind_speed: 3.0,
            wind_direction: 90,
            conditions: vec![WeatherDescription {
                id: 0,
                main: "Clear".into(),
                description: "clear sky".into(),
                icon: "01d".into(),
            }],
            main_condition: WeatherCondition::Clear,
            pop: 0.1,
            visibility: 10000,
            clouds: 0,
            rain: None,
            snow: None,
        })
        .collect()
}

fn daily(n: usize) -> Vec<DailyForecast> {
    let base = Utc::now();
    (0..n)
        .map(|i| DailyForecast {
            date: base + Duration::days(i as i64),
            sunrise: base,
            sunset: base,
            temp_morning: 12.0,
            temp_day: 22.0,
            temp_evening: 18.0,
            temp_night: 10.0,
            temp_min: 10.0,
            temp_max: 24.0,
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

fn build_view(nh: usize, nd: usize) -> ForecastView {
    ForecastView::build(
        None::<&HourlyForecast>,
        &hourly(nh),
        &daily(nd),
        &location(),
        &WeatherConfig::default(),
    )
}

#[test]
fn tone_color_is_stable_and_distinct() {
    assert_eq!(
        view::tone_color(&Theme::default(), ConditionTone::Sunny),
        view::tone_color(&Theme::default(), ConditionTone::Sunny)
    );
    assert_ne!(
        view::tone_color(&Theme::default(), ConditionTone::Sunny),
        view::tone_color(&Theme::default(), ConditionTone::Wet)
    );
}

#[test]
fn current_lines_has_header_and_data() {
    let v = build_view(3, 0);
    let lines = view::current_lines(&Theme::default(), v.current.as_ref());
    assert_eq!(lines.len(), 3); // header + two info lines
}

#[test]
fn current_lines_handles_missing() {
    let lines = view::current_lines(&Theme::default(), None);
    assert_eq!(lines.len(), 2); // header + "no data"
}

#[test]
fn hourly_lines_caps_at_24() {
    let v = build_view(48, 0);
    let lines = view::hourly_lines(&Theme::default(), &v.hours);
    assert_eq!(lines.len(), 25); // header + 24 rows
}

#[test]
fn daily_lines_caps_at_7() {
    let v = build_view(0, 10);
    let lines = view::daily_lines(&Theme::default(), &v.days);
    assert_eq!(lines.len(), 8); // header + 7 rows
}

#[test]
fn page_line_count_sums_sections() {
    let v = build_view(24, 7);
    // current(3) + blank(1) + hourly(25) + blank(1) + daily(8) = 38
    assert_eq!(view::page_line_count(&v), 38);
}
