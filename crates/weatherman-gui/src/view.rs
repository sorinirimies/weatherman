//! View rendering for the Iced GUI.

use crate::app::{App, Loaded, Message, Status};
use crate::theme::{ACCENT, MUTED, tone_color};
use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};
use weatherman_core::{AppSettings, DayDetail, DayRow, ForecastView, HourRow};

/// Top-level view: search bar on top, sidebar + forecast below.
pub fn view(app: &App) -> Element<'_, Message> {
    let content: Element<Message> = match (&app.status, &app.loaded) {
        (Status::Error(err), _) => error_view(err),
        (Status::Loading, None) => centered("Loading weather…"),
        (_, Some(loaded)) => loaded_view(loaded, app.selected_day),
        (_, None) => centered("No data"),
    };

    let main = row![
        sidebar(&app.settings, app.loaded.as_ref()),
        container(content).width(Length::Fill),
    ]
    .spacing(16);

    column![search_bar(app), main]
        .spacing(12)
        .padding(16)
        .into()
}

fn search_bar(app: &App) -> Element<'_, Message> {
    let units_label = if app.config.units == "imperial" {
        "°F"
    } else {
        "°C"
    };

    row![
        text("⛅ Weather Man").size(24).color(ACCENT),
        Space::new().width(Length::Fill),
        text_input("Search city…", &app.query)
            .on_input(Message::QueryChanged)
            .on_submit(Message::Search)
            .width(Length::Fixed(240.0))
            .padding(8),
        button(text("Search")).on_press(Message::Search).padding(8),
        button(text(units_label))
            .on_press(Message::ToggleUnits)
            .padding(8),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

/// Left sidebar listing saved locations with add/remove/switch controls.
fn sidebar<'a>(settings: &'a AppSettings, loaded: Option<&'a Loaded>) -> Element<'a, Message> {
    let mut col = column![text("📍 Saved Locations").size(16).color(ACCENT)].spacing(8);

    if settings.locations.is_empty() {
        col = col.push(text("No saved locations yet.").size(13).color(MUTED));
    } else {
        let active = loaded.map(|l| l.location_name.clone());
        for (i, loc) in settings.locations.iter().enumerate() {
            let is_active = active.as_deref() == Some(loc.as_str());
            let name_color = if is_active { ACCENT } else { MUTED };
            let entry = row![
                button(text(loc.clone()).size(14).color(name_color))
                    .on_press(Message::SelectSavedLocation(loc.clone()))
                    .style(button::text)
                    .width(Length::Fill)
                    .padding(4),
                button(text("✕").size(13))
                    .on_press(Message::RemoveSavedLocation(i))
                    .style(button::text)
                    .padding(4),
            ]
            .align_y(Alignment::Center);
            col = col.push(entry);
        }
    }

    // "Save current" is only actionable when a location is loaded.
    let mut save_btn = button(text("＋ Save current").size(13)).padding(6);
    if loaded.is_some() {
        save_btn = save_btn.on_press(Message::SaveCurrentLocation);
    }
    col = col
        .push(Space::new().height(Length::Fixed(8.0)))
        .push(save_btn);

    container(col)
        .padding(12)
        .width(Length::Fixed(200.0))
        .style(container::rounded_box)
        .into()
}

fn loaded_view(loaded: &Loaded, selected_day: Option<usize>) -> Element<'_, Message> {
    let ForecastView {
        current,
        hours,
        days,
    } = &loaded.view;

    let mut col = column![].spacing(16);

    if let Some(c) = current {
        col = col.push(current_card(c));
    }
    col = col
        .push(section_title("Next 24 Hours"))
        .push(hourly_strip(hours))
        .push(section_title("7-Day Forecast (click a day for details)"))
        .push(daily_list(days, selected_day));

    scrollable(col).height(Length::Fill).into()
}

fn current_card(c: &weatherman_core::CurrentView) -> Element<'_, Message> {
    let content = column![
        row![
            text(c.emoji).size(48),
            column![
                text(c.location_line.clone()).size(20).color(ACCENT),
                text(c.condition.clone()).size(16).color(tone_color(c.tone)),
            ]
            .spacing(2),
            Space::new().width(Length::Fill),
            text(c.temperature.clone()).size(44),
        ]
        .spacing(16)
        .align_y(Alignment::Center),
        row![
            stat("Feels like", c.feels_like.clone()),
            stat("Humidity", c.humidity.clone()),
            stat("Wind", c.wind.clone()),
            stat("Local time", c.local_time.clone()),
        ]
        .spacing(24),
    ]
    .spacing(14);

    card(content.into())
}

fn hourly_strip(hours: &[HourRow]) -> Element<'_, Message> {
    let mut r = row![].spacing(10);

    for h in hours {
        let cell = column![
            text(h.time.clone()).size(13).color(MUTED),
            text(h.emoji).size(24),
            text(h.temperature.clone()).size(15),
            text(format!("💧{}%", h.pop_percent))
                .size(12)
                .color(tone_color(h.tone)),
        ]
        .spacing(4)
        .align_x(Alignment::Center);
        r = r.push(card(cell.into()));
    }

    scrollable(r)
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::new(),
        ))
        .width(Length::Fill)
        .into()
}

fn daily_list(days: &[DayRow], selected: Option<usize>) -> Element<'_, Message> {
    let mut col = column![].spacing(8);

    for (i, d) in days.iter().enumerate() {
        let is_open = selected == Some(i);
        let indicator = if is_open { "▾" } else { "▸" };
        let summary = row![
            text(indicator).size(14).color(MUTED),
            text(d.label.clone())
                .size(15)
                .width(Length::Fixed(96.0))
                .color(ACCENT),
            text(d.emoji).size(20),
            text(d.condition.clone())
                .size(14)
                .width(Length::Fixed(120.0))
                .color(tone_color(d.tone)),
            Space::new().width(Length::Fill),
            text(format!("💧 {}%", d.pop_percent)).size(14).color(MUTED),
            Space::new().width(Length::Fixed(24.0)),
            text(d.temp_high_low.clone()).size(15),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let row_button = button(summary)
            .on_press(Message::DayToggled(i))
            .style(button::text)
            .width(Length::Fill)
            .padding(6);

        if is_open {
            col = col.push(
                container(column![row_button, day_detail(&d.detail)].spacing(4))
                    .style(container::rounded_box)
                    .padding(4),
            );
        } else {
            col = col.push(container(row_button).style(container::rounded_box));
        }
    }

    col.into()
}

fn day_detail(detail: &DayDetail) -> Element<'_, Message> {
    let temps = row![
        detail_stat("Morning", detail.temp_morning.clone()),
        detail_stat("Day", detail.temp_day.clone()),
        detail_stat("Evening", detail.temp_evening.clone()),
        detail_stat("Night", detail.temp_night.clone()),
    ]
    .spacing(20);

    let feels = row![
        detail_stat("Feels (day)", detail.feels_like_day.clone()),
        detail_stat("Feels (night)", detail.feels_like_night.clone()),
        detail_stat("🌅 Sunrise", detail.sunrise.clone()),
        detail_stat("🌇 Sunset", detail.sunset.clone()),
    ]
    .spacing(20);

    let extra = row![
        detail_stat("Wind", detail.wind.clone()),
        detail_stat("UV index", detail.uv_index.clone()),
        detail_stat("Precipitation", detail.precipitation.clone()),
    ]
    .spacing(20);

    container(column![temps, feels, extra].spacing(10))
        .padding(12)
        .into()
}

fn detail_stat(label: &str, value: String) -> Element<'_, Message> {
    column![
        text(label.to_string()).size(11).color(MUTED),
        text(value).size(15),
    ]
    .spacing(2)
    .into()
}

fn stat<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    column![text(label).size(12).color(MUTED), text(value).size(16),]
        .spacing(2)
        .into()
}

fn section_title(title: &str) -> Element<'_, Message> {
    text(title.to_string()).size(18).color(ACCENT).into()
}

fn card(content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .padding(12)
        .style(container::rounded_box)
        .into()
}

fn centered(msg: &str) -> Element<'_, Message> {
    container(text(msg.to_string()).size(18).color(MUTED))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn error_view(err: &str) -> Element<'_, Message> {
    container(
        column![
            text("⚠ Could not load weather").size(20),
            text(err.to_string()).size(14).color(MUTED),
            button(text("Retry")).on_press(Message::Search).padding(8),
        ]
        .spacing(12)
        .align_x(Alignment::Center),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
