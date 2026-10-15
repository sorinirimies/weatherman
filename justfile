# weatherman workspace — task runner
# Install just:      cargo install just
# Install vhs:       brew install vhs  OR  go install github.com/charmbracelet/vhs@latest
# Install git-cliff: cargo install git-cliff
# Install nushell:   cargo install nu
# Usage: just <task>

# ── Default ───────────────────────────────────────────────────────────────────

default:
    @just --list

# ── Tool checks ───────────────────────────────────────────────────────────────

_check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { \
        echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; \
    }

_check-nu:
    @command -v nu >/dev/null 2>&1 || { \
        echo "❌ nu (nushell) not found. Install: https://www.nushell.sh"; exit 1; \
    }

_check-vhs:
    @command -v vhs >/dev/null 2>&1 || { \
        echo "❌ vhs not found."; \
        echo "   macOS:      brew install vhs"; \
        echo "   Any:        go install github.com/charmbracelet/vhs@latest"; \
        exit 1; \
    }

# Install all recommended development tools
install-tools:
    @echo "Installing development tools…"
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff --locked
    @command -v nu >/dev/null 2>&1 || cargo install nu --locked
    @echo "✅ All tools installed!"

# ── Build ─────────────────────────────────────────────────────────────────────

# Build the entire workspace (dev)
build:
    cargo build --workspace

# Build only the core library (dev)
build-core:
    cargo build -p weatherman-core

# Build only the GUI crate (dev)
build-gui:
    cargo build -p weatherman

# Build only the TUI crate (dev)
build-tui:
    cargo build -p weatherman-tui

# Build release binaries for GUI and TUI
build-release:
    cargo build --release -p weatherman -p weatherman-tui

# ── Run ───────────────────────────────────────────────────────────────────────

# Launch the Iced desktop GUI
run-gui:
    cargo run -p weatherman

# Launch the Ratatui terminal UI
run-tui:
    cargo run -p weatherman-tui

# Alias: default run launches the GUI
run: run-gui

# ── Test ──────────────────────────────────────────────────────────────────────

# Run the full workspace test suite
test:
    cargo test --workspace --locked --all-features --all-targets

# Test only the core library
test-core:
    cargo test -p weatherman-core --all-features

# Test only the TUI crate
test-tui:
    cargo test -p weatherman-tui --all-features

# Run the Nushell script tests
test-nu: _check-nu
    nu scripts/tests/run_all.nu

# Run both the Rust and Nushell test suites
test-all-nu: test test-nu
    @echo "✅ All Rust and Nushell tests passed!"

# ── Examples / demos ──────────────────────────────────────────────────────────

# Print a forecast using only the core library (network). e.g. just example-report Berlin
example-report location="":
    cargo run -p weatherman-core --example report -- "{{ location }}"

# Run the offline custom-provider library demo (no network)
example-provider:
    cargo run -p weatherman-core --example custom_provider

# ── Code quality ──────────────────────────────────────────────────────────────

# Check without building
check:
    cargo check --workspace

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run clippy across the workspace
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all quality checks (format, clippy, test, nu) — must pass before a release.
check-all: fmt clippy test test-nu
    @echo "🔍 Verifying formatting is clean…"
    cargo fmt --all -- --check
    @echo "✅ All checks passed!"

# Full pre-release quality gate — everything in check-all plus a release build.
check-release: check-all build-release
    @echo "✅ Release quality gate passed (fmt + clippy + test + release build)!"

# ── VHS Demo GIFs ─────────────────────────────────────────────────────────────

GUI_VHS := "crates/weatherman-gui/examples/vhs"
TUI_VHS := "crates/weatherman-tui/examples/vhs"
CORE_VHS := "crates/weatherman-core/examples/vhs"
GUI_VHS_GENERATED := "crates/weatherman-gui/examples/vhs/generated"
TUI_VHS_GENERATED := "crates/weatherman-tui/examples/vhs/generated"
CORE_VHS_GENERATED := "crates/weatherman-core/examples/vhs/generated"

# Generate all VHS demo GIFs (GUI + TUI)
vhs-all: vhs-core vhs-gui vhs-tui

# Generate the core library demo GIF
vhs-core: _check-vhs
    #!/usr/bin/env sh
    set -e
    mkdir -p {{ CORE_VHS_GENERATED }}
    for tape in {{ CORE_VHS }}/*.tape; do
        [ -f "$tape" ] || continue
        echo "▶  $tape"
        vhs "$tape" || echo "❌ Failed: $tape"
    done
    echo "✅ Core demos done → {{ CORE_VHS_GENERATED }}/"

# Generate only the GUI demo GIFs
vhs-gui: _check-vhs
    #!/usr/bin/env sh
    set -e
    mkdir -p {{ GUI_VHS_GENERATED }}
    for tape in {{ GUI_VHS }}/*.tape; do
        [ -f "$tape" ] || continue
        echo "▶  $tape"
        vhs "$tape" || echo "❌ Failed: $tape"
    done
    echo "✅ GUI demos done → {{ GUI_VHS_GENERATED }}/"

# Generate only the TUI demo GIFs
vhs-tui: _check-vhs
    #!/usr/bin/env sh
    set -e
    mkdir -p {{ TUI_VHS_GENERATED }}
    for tape in {{ TUI_VHS }}/*.tape; do
        [ -f "$tape" ] || continue
        echo "▶  $tape"
        vhs "$tape" || echo "❌ Failed: $tape"
    done
    echo "✅ TUI demos done → {{ TUI_VHS_GENERATED }}/"

# Render a single tape by name (e.g. just vhs-tape tui-demo)
vhs-tape name: _check-vhs
    #!/usr/bin/env sh
    if [ -f "{{ GUI_VHS }}/{{ name }}.tape" ]; then
        vhs "{{ GUI_VHS }}/{{ name }}.tape" && echo "✅ Done."
    elif [ -f "{{ TUI_VHS }}/{{ name }}.tape" ]; then
        vhs "{{ TUI_VHS }}/{{ name }}.tape" && echo "✅ Done."
    elif [ -f "{{ CORE_VHS }}/{{ name }}.tape" ]; then
        vhs "{{ CORE_VHS }}/{{ name }}.tape" && echo "✅ Done."
    else
        echo "❌ Tape not found: {{ name }}.tape"; just vhs-list; exit 1
    fi

# List all available VHS tapes
vhs-list:
    #!/usr/bin/env sh
    echo "GUI tapes  →  {{ GUI_VHS }}/"
    ls {{ GUI_VHS }}/*.tape 2>/dev/null | sed 's|.*/||; s|\.tape||' | sed 's/^/  /' || echo "  (none)"
    echo ""
    echo "TUI tapes  →  {{ TUI_VHS }}/"
    ls {{ TUI_VHS }}/*.tape 2>/dev/null | sed 's|.*/||; s|\.tape||' | sed 's/^/  /' || echo "  (none)"
    echo ""
    echo "Core tapes →  {{ CORE_VHS }}/"
    ls {{ CORE_VHS }}/*.tape 2>/dev/null | sed 's|.*/||; s|\.tape||' | sed 's/^/  /' || echo "  (none)"

# ── Documentation ─────────────────────────────────────────────────────────────

# Generate docs for the full workspace (no browser)
doc:
    cargo doc --no-deps --workspace

# ── Changelog ─────────────────────────────────────────────────────────────────

# Regenerate the full CHANGELOG.md from all tags
changelog: _check-git-cliff
    git-cliff --output CHANGELOG.md
    @echo "✅ CHANGELOG.md updated."

# Preview changelog for the next release without writing the file
changelog-preview: _check-git-cliff
    @git-cliff --unreleased

# ── Version bump ─────────────────────────────────────────────────────────────

# Bump the workspace version, regenerate Cargo.lock + CHANGELOG.md, commit and tag.
bump version: check-release _check-git-cliff _check-nu
    nu scripts/bump_version.nu --yes {{ version }}

# Show the current workspace version
version: _check-nu
    @nu scripts/version.nu

# ── Publish (crates.io) ───────────────────────────────────────────────────────

# Run the full pre-publish readiness check
check-publish: _check-nu
    nu scripts/check_publish.nu

# Dry-run publish for all three crates (in dependency order)
publish-dry: check-all
    cargo publish --dry-run -p weatherman-core
    cargo publish --dry-run -p weatherman
    cargo publish --dry-run -p weatherman-tui

# Publish all three in dependency order: core → gui → tui.
publish: check-all publish-core publish-gui publish-tui
    @echo "✅ weatherman-core, weatherman, and weatherman-tui published!"

# Publish weatherman-core (required by gui and tui)
publish-core:
    @echo "📦 Publishing weatherman-core…"
    cargo publish -p weatherman-core
    @echo "⏳ Waiting 30 s for the index to propagate…"
    sleep 30

# Publish weatherman (GUI)
publish-gui:
    @echo "📦 Publishing weatherman (GUI)…"
    cargo publish -p weatherman

# Publish weatherman-tui
publish-tui:
    @echo "📦 Publishing weatherman-tui…"
    cargo publish -p weatherman-tui

# ── Housekeeping ──────────────────────────────────────────────────────────────

# Remove build artifacts
clean:
    cargo clean

# Update all dependencies (Cargo.lock only)
update:
    cargo update

# Upgrade dependencies via cargo-edit and cross-check (nu script)
upgrade-deps: _check-nu
    nu scripts/upgrade_deps.nu

# Show outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# ── Git remotes & pushing ────────────────────────────────────────────────────
# weatherman lives on GitHub (origin) and the Gitea Starscream instance. Every
# *-all recipe loops over REMOTES and keeps going when one fails.

REMOTES := "origin gitea-starscream"

# Push the current branch to GitHub (origin)
push:
    git push origin main

# Push the current branch to Gitea Starscream
push-gitea-starscream:
    git push gitea-starscream main

# Push the current branch to every remote (continues on failure)
push-all:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push "$r" main || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to push to:$failed"; exit 1; fi
    echo "✅ Pushed to: {{ REMOTES }}"

# Force-push the current branch to every remote (leases are NOT used — be sure!)
push-all-force:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push --force "$r" main || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to force-push to:$failed"; exit 1; fi
    echo "✅ Force-pushed to: {{ REMOTES }}"

# Pull the current branch from GitHub (origin)
pull:
    git pull origin main

# Pull the current branch from Gitea Starscream
pull-gitea-starscream:
    git pull gitea-starscream main

# Pull the current branch from every remote (continues on failure)
pull-all:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git pull "$r" main || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to pull from:$failed"; exit 1; fi
    echo "✅ Pulled from: {{ REMOTES }}"

# Push all tags to GitHub
push-tags:
    git push origin --tags

# Push all tags to every remote (continues on failure)
push-tags-all:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push "$r" --tags || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to push tags to:$failed"; exit 1; fi
    echo "✅ Tags pushed to: {{ REMOTES }}"

# Force-sync Gitea Starscream with GitHub (main + tags)
sync-gitea-starscream:
    git push gitea-starscream main --force
    git push gitea-starscream --tags --force
    @echo "✅ Gitea Starscream force-synced."

# Add the Gitea Starscream remote if it is missing
setup-remotes:
    #!/usr/bin/env sh
    git remote get-url origin >/dev/null 2>&1 || git remote add origin git@github.com:sorinirimies/weatherman.git
    git remote get-url gitea-starscream >/dev/null 2>&1 || git remote add gitea-starscream gitea@192.168.1.44:sorin/weatherman.git
    git remote -v

# ── Release workflow ──────────────────────────────────────────────────────────

# Show what would be released without making any changes
release-preview: _check-git-cliff _check-nu
    @echo "Current version: $(nu scripts/version.nu)"
    @echo ""
    @echo "Unreleased commits:"
    @git-cliff --unreleased

# Bump, commit, tag, then push to every remote — triggers the Release workflows.
release version: (bump version)
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push --follow-tags "$r" main || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Release v{{ version }} failed to push to:$failed"; exit 1; fi
    echo "✅ Release v{{ version }} pushed to: {{ REMOTES }}"

# Bump, commit, tag, then push to GitHub only.
release-github version: (bump version)
    git push --follow-tags origin main

# Bump, commit, tag, then push to Gitea Starscream only.
release-gitea-starscream version: (bump version)
    git push --follow-tags gitea-starscream main

# Push latest commit and tags to every remote (no bump).
push-release-all: check-all
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push --follow-tags "$r" main || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to push to:$failed"; exit 1; fi

# Security: dependency vulnerability scan (needs `cargo install cargo-audit`)
audit:
    cargo audit

# Security: licenses/advisories/bans/sources (needs `cargo install cargo-deny --locked`)
deny:
    cargo deny check
