# Changelog

All notable changes to this project will be documented in this file.

## [0.5.1] - 2026-10-13
### 🐛 Bug Fixes
- patch h2/rustls, ignore build-only quick-xml advisories, fix cargo upgrade flag; justfile multi-remote
### 📚 Documentation
- normalise historical crate names to weatherman in changelog
### 🔄 CI
- replace hustcer/setup-nu with direct binary install
- add nightly dependency-update + auto-release pipeline
- add cargo-audit workflow and just audit recipe
### 🔧 Chores
- point repository URLs at renamed github repo (weatherman)
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.5.0...v0.5.1
## [0.5.0] - 2026-07-13
### ✨ Features
- add DayDetail view-model and persistent AppSettings
- expandable day details and saved-locations sidebar
### 📚 Documentation
- document GUI day details and saved locations
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.4.2...v0.5.0
## [0.4.2] - 2026-07-10
### 📦 Build
- update all workspace dependencies
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.4.1...v0.4.2
## [0.4.1] - 2026-07-10
### 📚 Documentation
- add crates.io GUI and TUI download badges
### 🔄 CI
- install nushell via cargo instead of setup-nu action
- pin nushell to 0.111.0 for the script test job
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.4.0...v0.4.1
## [0.4.0] - 2026-07-10
### ♻️  Refactor
- centralize weather presentation logic in core; reuse across GUI and TUI
- rename crates to weatherman / weatherman-tui / weatherman-core **[BREAKING]**
### 📚 Documentation
- add core library examples and demo GIFs (Git LFS)
### 🧪 Testing
- add Nushell script test suite
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.3.0...v0.4.0
## [0.3.0] - 2026-07-10
### ✨ Features
- restructure into a Cargo workspace with a shared weatherman-core library **[BREAKING]**
- add single-page Ratatui TUI (weatherman-tui)
- add Iced desktop GUI (weatherman)
### 🐛 Bug Fixes
- parse naive Open-Meteo timestamps so hourly forecast populates
### 📚 Documentation
- rewrite README for the GUI + TUI + core workspace
- update CHANGELOG for v0.3.0
- add generated TUI demo GIF
- refresh CHANGELOG for v0.3.0
### 📦 Build
- add justfile and nushell release scripts
### 🔄 CI
- add git-cliff changelog config and VHS demo tapes
- rework release workflow for the workspace and add CI workflow
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.2.10...v0.3.0
## [0.0.7] - 2025-05-31
### 🔧 Chores
- update version to 0.0.6
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.0.6...v0.0.7
## [0.0.5] - 2025-05-31
### 🔄 CI
- Fix GitHub release token permissions
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.0.4...v0.0.5
## [0.0.4] - 2025-05-31
### 🔄 CI
- Simplify CI build and release workflow
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.0.3...v0.0.4
## [0.0.3] - 2025-05-31
### 💄 Style
- Fix formatting issues
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.0.2...v0.0.3
## [0.1.4] - 2025-05-31
### 💄 Style
- Fix formatting issues
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.1.3...v0.1.4
## [0.1.3] - 2025-05-31
### 🐛 Bug Fixes
- Remove unused imports and fix clippy warnings
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.1.2...v0.1.3
## [0.1.2] - 2025-05-31
### 💄 Style
- Format code with cargo fmt
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.1.1...v0.1.2
## [0.1.1] - 2025-05-31
### 🐛 Bug Fixes
- Handle edge cases in degrees_to_direction function
### 🔄 CI
- Update release workflow to run tests before publishing
**Full Changelog**: https://github.com/sorinirimies/weatherman/compare/v0.1.0...v0.1.1
## [0.1.0] - 2025-05-31
