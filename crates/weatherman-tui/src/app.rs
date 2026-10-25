//! The terminal application: owns state, the event loop, and terminal setup.

use crate::view;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use tui_nuances::Theme;
use weatherman_core::{ForecastView, Location, WeatherConfig, WeatherReport};

/// The single-page terminal weather UI.
pub struct WeatherTui {
    view: ForecastView,
    location: Location,
    config: WeatherConfig,
    scroll: u16,
    theme: Theme,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl WeatherTui {
    /// Create a new TUI from a fully-resolved weather report.
    pub fn new(report: WeatherReport, config: WeatherConfig) -> Result<Self> {
        Self::with_theme(report, config, Theme::default())
    }

    /// Like [`WeatherTui::new`] with an explicit `tui-nuances` theme.
    pub fn with_theme(report: WeatherReport, config: WeatherConfig, theme: Theme) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let view = ForecastView::build(
            report.current.as_ref(),
            &report.hourly,
            &report.daily,
            &report.location,
            &config,
        );

        Ok(Self {
            view,
            location: report.location,
            config,
            scroll: 0,
            theme,
            terminal,
        })
    }

    /// Run the event loop until the user quits.
    pub fn run(&mut self) -> Result<()> {
        loop {
            let total = view::page_line_count(&self.view);
            let scroll = self.scroll;

            self.terminal.draw(|f| {
                view::render(
                    f,
                    &self.theme,
                    &self.view,
                    &self.location,
                    &self.config,
                    scroll,
                );
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.scroll = scroll.saturating_add(1).min(total);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.scroll = scroll.saturating_sub(1);
                    }
                    KeyCode::PageDown => {
                        self.scroll = scroll.saturating_add(10).min(total);
                    }
                    KeyCode::PageUp => {
                        self.scroll = scroll.saturating_sub(10);
                    }
                    KeyCode::Char('t') => self.theme = self.theme.next(),
                    KeyCode::Char('T') => self.theme = self.theme.prev(),
                    KeyCode::Char('g') | KeyCode::Home => self.scroll = 0,
                    KeyCode::Char('G') | KeyCode::End => self.scroll = total,
                    _ => {}
                }
            }
        }

        self.restore();
        Ok(())
    }

    fn restore(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

impl Drop for WeatherTui {
    fn drop(&mut self) {
        self.restore();
        println!();
    }
}
