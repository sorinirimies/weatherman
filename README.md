<div align="center">

# 🌤️ Weather Man

**A weather app written entirely in Rust — desktop GUI & terminal UI**

[![Crates.io](https://img.shields.io/crates/v/weatherman.svg)](https://crates.io/crates/weatherman)
[![GUI Downloads](https://img.shields.io/crates/d/weatherman?label=GUI%20downloads)](https://crates.io/crates/weatherman)
[![TUI Downloads](https://img.shields.io/crates/d/weatherman-tui?label=TUI%20downloads)](https://crates.io/crates/weatherman-tui)
[![docs.rs](https://docs.rs/weatherman-core/badge.svg)](https://docs.rs/weatherman-core)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Audit](https://github.com/sorinirimies/weatherman/actions/workflows/security.yml/badge.svg)](https://github.com/sorinirimies/weatherman/actions/workflows/security.yml)

</div>

---

Weather Man ships two front-ends from a single Rust workspace, backed by a shared,
framework-free core that doubles as a **drop-in weather API provider library**.

| Crate | Description |
|-------|-------------|
| [`weatherman`](crates/weatherman-gui) | Desktop **GUI** (Iced) — search a city, current conditions, hourly + 7-day forecast |
| [`weatherman-tui`](crates/weatherman-tui) | Terminal **UI** (Ratatui) — single scrollable page, keyboard-driven, great over SSH |
| [`weatherman-core`](crates/weatherman-core) | Shared **core** — domain models, `WeatherProvider` trait, Open-Meteo backend, geocoding |

Weather data comes from [Open-Meteo](https://open-meteo.com/) (no API key required),
with geocoding via [Nominatim/OpenStreetMap](https://nominatim.openstreetmap.org/).

## Preview

### Core library demo (`weatherman-core`)

The shared core fetches and formats a full forecast with no UI framework:

![Core library demo](crates/weatherman-core/examples/vhs/generated/core-demo.gif)

### Terminal UI (`weatherman-tui`)

![TUI Demo](crates/weatherman-tui/examples/vhs/generated/tui-demo.gif)

Imperial units (°F, mph):

![TUI Imperial](crates/weatherman-tui/examples/vhs/generated/tui-imperial.gif)

> Demo GIFs are stored with [Git LFS](https://git-lfs.com/). Run `git lfs install`
> after cloning to fetch them, or regenerate everything with `just vhs-all`.

### Desktop GUI (`weatherman`)

Run `weatherman` to launch the Iced desktop app — a city search, a current-
conditions card, a horizontal hourly strip, and a 7-day list with a °C/°F toggle.

## Install

```bash
# Desktop GUI
cargo install weatherman

# Terminal UI
cargo install weatherman-tui
```

## Usage

### GUI

```bash
weatherman
```

- Type a city and press Enter to search; toggle °C/°F with the units button.
- **Click any day** in the 7-day forecast to expand full details (day-part
  temperatures, feels-like, sunrise/sunset, wind, UV index, precipitation).
- **Save locations** to the sidebar with “＋ Save current”, then click to switch
  between them. Saved locations and your unit preference persist across sessions
  (`<config-dir>/weatherman/settings.json`).

### TUI

```bash
weatherman-tui                        # auto-detected location
weatherman-tui --location "New York"  # specific city
weatherman-tui --units imperial       # imperial units
weatherman-tui --json                 # JSON output, no TUI
```

Keys: `↑`/`↓` or `j`/`k` scroll · `PgUp`/`PgDn` page · `g`/`G` top/bottom · `q`/`Esc` quit.

## Use the core as a library

`weatherman-core` has no GUI/TUI dependencies. The high-level [`load_report`]
resolves a location and fetches its forecast, and [`ForecastView`] turns the raw
data into ready-to-render, UI-agnostic strings + condition tones:

```rust,no_run
use weatherman_core::{load_report, ForecastView, WeatherConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = WeatherConfig::default();
    let report = load_report(&config, Some("Berlin")).await?;
    let view = ForecastView::build(
        report.current.as_ref(),
        &report.hourly,
        &report.daily,
        &report.location,
        &config,
    );
    if let Some(c) = &view.current {
        println!("{} {} {}", c.emoji, c.condition, c.temperature);
    }
    Ok(())
}
```

You can also plug in a different backend by implementing the `WeatherProvider`
trait — see the runnable examples below.

## Examples

```bash
# Fetch and pretty-print a forecast using only the core library (network)
cargo run -p weatherman-core --example report -- Berlin
just example-report Berlin

# Offline demo: a custom WeatherProvider backed by canned data (no network)
cargo run -p weatherman-core --example custom_provider
just example-provider
```

## Development

This repo uses [`just`](https://github.com/casey/just) as a task runner and
[`nushell`](https://www.nushell.sh) for release scripts.

```bash
just             # list all tasks
just build       # build the workspace
just run-gui     # launch the GUI
just run-tui     # launch the TUI
just test        # run all Rust tests
just test-nu     # run the Nushell script tests
just test-all-nu # run both Rust and Nushell tests
just check-all   # fmt + clippy + test + nu
just vhs-all     # regenerate demo GIFs (needs vhs)
```

### Release

```bash
just release-preview   # show unreleased commits
just release 0.3.1     # bump, changelog, commit, tag, push → triggers Release workflow
```

Changelogs are generated with [git-cliff](https://github.com/orhun/git-cliff)
from [Conventional Commits](https://www.conventionalcommits.org/).

## License

MIT — see [LICENSE](LICENSE).

## Acknowledgments

- Weather data by [Open-Meteo](https://open-meteo.com/)
- Geocoding by [Nominatim/OpenStreetMap](https://nominatim.openstreetmap.org/)
