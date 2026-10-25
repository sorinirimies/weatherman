//! Single-page, scrollable weather view rendering (no animations).

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use tui_nuances::Theme;
use weatherman_core::{
    temp_unit_label, ConditionTone, CurrentView, DayRow, ForecastView, HourRow, Location,
    WeatherConfig,
};

/// Map a weather condition tone to a colour of `theme`.
pub fn tone_color(theme: &Theme, tone: ConditionTone) -> Color {
    match tone {
        ConditionTone::Sunny => theme.warning,
        ConditionTone::Cloudy | ConditionTone::Neutral => theme.muted,
        ConditionTone::Wet => theme.info,
        ConditionTone::Storm => theme.secondary,
        ConditionTone::Snow => theme.foreground,
        ConditionTone::Fog => theme.disabled,
    }
}

fn section_header<'a>(theme: &Theme, title: &str) -> Line<'a> {
    Line::from(Span::styled(
        title.to_string(),
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    ))
}

fn missing<'a>(theme: &Theme, msg: &str) -> Line<'a> {
    Line::from(Span::styled(
        msg.to_string(),
        Style::default().fg(theme.error),
    ))
}

/// Build the current-conditions section from a pre-computed view.
pub fn current_lines<'a>(theme: &Theme, current: Option<&CurrentView>) -> Vec<Line<'a>> {
    let mut lines = vec![section_header(theme, "🌡  Current Conditions")];

    let Some(c) = current else {
        lines.push(missing(theme, "No current weather data available."));
        return lines;
    };

    lines.push(Line::from(vec![
        Span::raw(format!("{} ", c.emoji)),
        Span::styled(
            c.condition.clone(),
            Style::default().fg(tone_color(theme, c.tone)),
        ),
        Span::raw("   "),
        Span::styled(
            c.temperature.clone(),
            Style::default()
                .fg(theme.foreground)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("  (feels {})", c.feels_like)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Local time: ", Style::default().fg(theme.muted)),
        Span::raw(c.local_time.clone()),
        Span::raw("   "),
        Span::styled("Humidity: ", Style::default().fg(theme.muted)),
        Span::raw(c.humidity.clone()),
        Span::raw("   "),
        Span::styled("Wind: ", Style::default().fg(theme.muted)),
        Span::raw(c.wind.clone()),
    ]));

    lines
}

/// Build the next-24-hours section from pre-computed rows.
pub fn hourly_lines<'a>(theme: &Theme, hours: &[HourRow]) -> Vec<Line<'a>> {
    let mut lines = vec![section_header(theme, "🕓  Next 24 Hours")];

    if hours.is_empty() {
        lines.push(missing(theme, "No hourly forecast data available."));
        return lines;
    }

    for h in hours {
        lines.push(Line::from(vec![
            Span::styled(format!("{}  ", h.time), Style::default().fg(theme.accent)),
            Span::raw(format!("{} ", h.emoji)),
            Span::styled(
                format!("{:>7}", h.temperature),
                Style::default().fg(theme.foreground),
            ),
            Span::raw("   "),
            Span::styled(
                format!("💧 {:>3}%", h.pop_percent),
                Style::default().fg(theme.info),
            ),
            Span::raw("   "),
            Span::styled(
                h.condition.clone(),
                Style::default().fg(tone_color(theme, h.tone)),
            ),
        ]));
    }

    lines
}

/// Build the 7-day forecast section from pre-computed rows.
pub fn daily_lines<'a>(theme: &Theme, days: &[DayRow]) -> Vec<Line<'a>> {
    let mut lines = vec![section_header(theme, "📅  7-Day Forecast")];

    if days.is_empty() {
        lines.push(missing(theme, "No daily forecast data available."));
        return lines;
    }

    for d in days {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<10}", d.label),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{:<6}", d.date), Style::default().fg(theme.muted)),
            Span::raw(format!("{} ", d.emoji)),
            Span::styled(
                format!("{:<13}", d.condition),
                Style::default().fg(tone_color(theme, d.tone)),
            ),
            Span::styled(
                format!("{:>13}", d.temp_high_low),
                Style::default().fg(theme.foreground),
            ),
            Span::raw("   "),
            Span::styled(
                format!("💧 {:>3}%", d.pop_percent),
                Style::default().fg(theme.info),
            ),
        ]));
    }

    lines
}

/// Assemble the whole scrollable page from a forecast view-model.
pub fn build_page<'a>(theme: &Theme, view: &ForecastView) -> Text<'a> {
    let mut lines: Vec<Line> = Vec::new();
    lines.extend(current_lines(theme, view.current.as_ref()));
    lines.push(Line::from(""));
    lines.extend(hourly_lines(theme, &view.hours));
    lines.push(Line::from(""));
    lines.extend(daily_lines(theme, &view.days));
    Text::from(lines)
}

/// Render the full frame: title, scrollable body, help bar.
pub fn render(
    f: &mut Frame,
    theme: &Theme,
    view: &ForecastView,
    location: &Location,
    config: &WeatherConfig,
    scroll: u16,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    render_title(f, theme, chunks[0], location, config);

    let body = Paragraph::new(build_page(theme, view))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Forecast")
                .style(Style::default().fg(theme.accent)),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(body, chunks[1]);

    render_help(f, theme, chunks[2]);
}

fn render_title(
    f: &mut Frame,
    theme: &Theme,
    area: Rect,
    location: &Location,
    config: &WeatherConfig,
) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("Weather Man - {}", location.name),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("[{}]", location.country),
            Style::default().fg(theme.muted),
        ),
        Span::raw("  "),
        Span::styled(
            format!("({})", temp_unit_label(&config.units)),
            Style::default().fg(theme.warning),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(theme.accent)),
    );
    f.render_widget(title, area);
}

fn render_help(f: &mut Frame, theme: &Theme, area: Rect) {
    let help = Paragraph::new(Line::from(vec![
        Span::styled("Keys: ", Style::default().fg(theme.accent)),
        Span::styled("↑/↓", Style::default().fg(theme.warning)),
        Span::raw(" Scroll | "),
        Span::styled("PgUp/PgDn", Style::default().fg(theme.warning)),
        Span::raw(" Page | "),
        Span::styled("g/G", Style::default().fg(theme.warning)),
        Span::raw(" Top/Bottom | "),
        Span::styled("q/ESC", Style::default().fg(theme.warning)),
        Span::raw(" Quit | "),
        Span::styled("t/T", Style::default().fg(theme.warning)),
        Span::raw(format!(" Theme ({})", theme.name)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(theme.accent)),
    );
    f.render_widget(help, area);
}

/// Total number of content lines (used to clamp scrolling).
pub fn page_line_count(view: &ForecastView) -> u16 {
    build_page(&Theme::default(), view).lines.len() as u16
}
