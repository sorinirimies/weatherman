use weatherman_core::{
    WeatherCondition, WeatherConfig, WeatherForecaster, weather_description_from_wmo,
    wmo_code_to_condition,
};

#[test]
fn test_wmo_code_to_condition_free_fn() {
    assert_eq!(wmo_code_to_condition(0), WeatherCondition::Clear);
    assert_eq!(wmo_code_to_condition(2), WeatherCondition::Clouds);
    assert_eq!(wmo_code_to_condition(45), WeatherCondition::Fog);
    assert_eq!(wmo_code_to_condition(51), WeatherCondition::Drizzle);
    assert_eq!(wmo_code_to_condition(61), WeatherCondition::Rain);
    assert_eq!(wmo_code_to_condition(71), WeatherCondition::Snow);
    assert_eq!(wmo_code_to_condition(80), WeatherCondition::Rain);
    assert_eq!(wmo_code_to_condition(95), WeatherCondition::Thunderstorm);
    assert_eq!(wmo_code_to_condition(96), WeatherCondition::Thunderstorm);
    assert_eq!(wmo_code_to_condition(1234), WeatherCondition::Unknown);
}

#[test]
fn test_weather_description_from_wmo_free_fn() {
    let clear = weather_description_from_wmo(0, true);
    assert_eq!(clear.main, "Clear");
    assert_eq!(clear.description, "Clear sky");
    assert_eq!(clear.icon, "01d");

    let clear_night = weather_description_from_wmo(0, false);
    assert_eq!(clear_night.icon, "01n");

    let overcast = weather_description_from_wmo(3, true);
    assert_eq!(overcast.main, "Clouds");
    assert_eq!(overcast.description, "Overcast");
}

#[test]
fn test_forecaster_methods_delegate() {
    let forecaster = WeatherForecaster::new(WeatherConfig::default());
    assert_eq!(
        forecaster.wmo_code_to_condition(95),
        WeatherCondition::Thunderstorm
    );
    let desc = forecaster.get_weather_description_from_wmo(61, true);
    assert_eq!(desc.main, "Rain");
    assert_eq!(desc.description, "Slight rain");
}
