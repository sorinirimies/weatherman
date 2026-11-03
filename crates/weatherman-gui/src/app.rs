//! Application state, messages, and update logic for the Iced GUI.

use iced::Task;
use weatherman_core::{
    AppSettings, ForecastView, WeatherConfig, load_report, load_settings, save_settings,
};

/// A successfully loaded, ready-to-render forecast.
#[derive(Debug, Clone)]
pub struct Loaded {
    pub location_name: String,
    pub view: ForecastView,
}

/// Messages driving the application.
#[derive(Debug, Clone)]
pub enum Message {
    /// The search text input changed.
    QueryChanged(String),
    /// Trigger a fetch for the current query (or auto-detect when empty).
    Search,
    /// Toggle between metric and imperial units (re-fetches).
    ToggleUnits,
    /// A weather fetch finished.
    Fetched(Result<Box<Loaded>, String>),
    /// Expand/collapse the detail panel for a day (by index).
    DayToggled(usize),
    /// Save the currently displayed location to the sidebar.
    SaveCurrentLocation,
    /// Switch to a saved location (by name) and fetch it.
    SelectSavedLocation(String),
    /// Remove a saved location by its index.
    RemoveSavedLocation(usize),
}

/// Loading status of the current view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Loading,
    Ready,
    Error(String),
}

/// The root application state.
#[derive(Debug)]
pub struct App {
    pub config: WeatherConfig,
    pub query: String,
    pub status: Status,
    pub loaded: Option<Loaded>,
    pub settings: AppSettings,
    pub selected_day: Option<usize>,
}

impl App {
    /// Build the initial state and kick off the first fetch.
    pub fn new() -> (Self, Task<Message>) {
        let settings = load_settings();
        let config = WeatherConfig {
            units: settings.units.clone(),
            ..Default::default()
        };
        // Open on the first saved location if there is one, else auto-detect.
        let initial_query = settings.locations.first().cloned();
        let app = Self {
            config: config.clone(),
            query: String::new(),
            status: Status::Loading,
            loaded: None,
            settings,
            selected_day: None,
        };
        (
            app,
            Task::perform(fetch(config, initial_query), Message::Fetched),
        )
    }

    /// Window title.
    pub fn title(&self) -> String {
        match &self.loaded {
            Some(l) => format!("Weather Man — {}", l.location_name),
            None => "Weather Man".to_string(),
        }
    }

    /// Handle a message and optionally schedule a task.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QueryChanged(q) => {
                self.query = q;
                Task::none()
            }
            Message::Search => self.fetch_query(self.current_query()),
            Message::ToggleUnits => {
                self.config.units = if self.config.units == "imperial" {
                    "metric".to_string()
                } else {
                    "imperial".to_string()
                };
                self.settings.units = self.config.units.clone();
                let _ = save_settings(&self.settings);
                let query = self.active_location();
                self.fetch_query(query)
            }
            Message::Fetched(Ok(loaded)) => {
                self.status = Status::Ready;
                self.selected_day = None;
                self.loaded = Some(*loaded);
                Task::none()
            }
            Message::Fetched(Err(err)) => {
                self.status = Status::Error(err);
                Task::none()
            }
            Message::DayToggled(index) => {
                self.selected_day = if self.selected_day == Some(index) {
                    None
                } else {
                    Some(index)
                };
                Task::none()
            }
            Message::SaveCurrentLocation => {
                if let Some(loaded) = &self.loaded
                    && self.settings.add_location(&loaded.location_name)
                {
                    let _ = save_settings(&self.settings);
                }
                Task::none()
            }
            Message::SelectSavedLocation(name) => {
                self.query = name.clone();
                self.fetch_query(Some(name))
            }
            Message::RemoveSavedLocation(index) => {
                self.settings.remove_location(index);
                let _ = save_settings(&self.settings);
                Task::none()
            }
        }
    }

    /// Kick off a fetch for `query`, moving into the loading state.
    fn fetch_query(&mut self, query: Option<String>) -> Task<Message> {
        self.status = Status::Loading;
        self.selected_day = None;
        Task::perform(fetch(self.config.clone(), query), Message::Fetched)
    }

    fn current_query(&self) -> Option<String> {
        let q = self.query.trim();
        if q.is_empty() {
            None
        } else {
            Some(q.to_string())
        }
    }

    /// The location to re-fetch (current search, else the loaded location).
    fn active_location(&self) -> Option<String> {
        self.current_query()
            .or_else(|| self.loaded.as_ref().map(|l| l.location_name.clone()))
    }
}

/// Fetch weather for an optional named location and build its view-model.
async fn fetch(config: WeatherConfig, query: Option<String>) -> Result<Box<Loaded>, String> {
    let report = load_report(&config, query.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    let view = ForecastView::build(
        report.current.as_ref(),
        &report.hourly,
        &report.daily,
        &report.location,
        &config,
    );

    Ok(Box::new(Loaded {
        location_name: report.location.name,
        view,
    }))
}
