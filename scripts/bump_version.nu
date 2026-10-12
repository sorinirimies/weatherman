#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# weatherman — Bump workspace version
# ──────────────────────────────────────────────────────────────────────────────
# Usage:
#   nu scripts/bump_version.nu [--yes] [--skip-checks] <new_version>
#
# What it does:
#   1. Validates the supplied semantic version string.
#   2. Updates `workspace.package.version` in the root Cargo.toml.
#   3. Updates the `weatherman-core` dependency version.
#   4. Runs `cargo fmt`, `cargo clippy`, and `cargo test`.
#   5. Refreshes Cargo.lock and CHANGELOG.md (git-cliff, if installed).
#   6. Creates a Git commit and an annotated tag.
# ──────────────────────────────────────────────────────────────────────────────

def validate_version [version: string] {
    if not (is_valid_version $version) {
        print $"(ansi red)Error:(ansi reset) '($version)' is not a valid semantic version."
        exit 1
    }
}

# ── Pure helpers (exported for testing) ───────────────────────────────────────

# Return true when the string is a valid MAJOR.MINOR.PATCH(-prerelease) semver.
export def is_valid_version [version: string]: nothing -> bool {
    let pattern = '^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$'
    ($version | find --regex $pattern | is-not-empty)
}

# Replace the workspace.package.version in a Cargo.toml string. Pure transform.
export def set_workspace_version [cargo: string, version: string]: nothing -> string {
    $cargo | str replace --regex 'version\s*=\s*"[^"]+"' $'version    = "($version)"'
}

# Replace the weatherman-core dependency version in a Cargo.toml string. Pure.
export def set_core_dep_version [cargo: string, version: string]: nothing -> string {
    $cargo
    | lines
    | each {|line|
        if ($line | find --regex '^weatherman-core\s*=' | is-not-empty) {
            $line | str replace --regex 'version\s*=\s*"[^"]+"' $'version = "($version)"'
        } else {
            $line
        }
    }
    | str join "\n"
}

def update_workspace_version [version: string] {
    let cargo = (open Cargo.toml --raw)
    set_workspace_version $cargo $version | save --force Cargo.toml
    print $"(ansi green)✓(ansi reset) Updated workspace.package.version → ($version)"
}

def update_core_dep_version [version: string] {
    let cargo = (open Cargo.toml --raw)
    set_core_dep_version $cargo $version | save --force Cargo.toml
    print $"(ansi green)✓(ansi reset) Updated weatherman-core dependency → ($version)"
}

def main [
    new_version: string,  # New version in X.Y.Z format
    --yes (-y),           # Skip confirmation prompt (non-interactive)
    --skip-checks,        # Skip fmt/clippy/test (caller already ran the quality gate)
] {
    print ""
    print $"(ansi cyan)══════════════════════════════════════════════════════════════(ansi reset)"
    print $"(ansi cyan)  weatherman — Bump Version(ansi reset)"
    print $"(ansi cyan)══════════════════════════════════════════════════════════════(ansi reset)"
    print ""

    let current_version = (open Cargo.toml | get workspace.package.version)
    print $"  Current version : (ansi yellow)($current_version)(ansi reset)"
    print $"  New version     : (ansi green)($new_version)(ansi reset)"
    print ""

    if $current_version == $new_version {
        print $"(ansi yellow)⚠(ansi reset) Version is already ($new_version). Nothing to do."
        exit 0
    }

    validate_version $new_version
    print $"(ansi green)✓(ansi reset) Version string validated."

    update_workspace_version $new_version
    update_core_dep_version $new_version

    if not $skip_checks {
        print ""
        print $"(ansi cyan)── cargo fmt ───────────────────────────────────────────────(ansi reset)"
        run-external "cargo" "fmt" "--all"

        print ""
        print $"(ansi cyan)── cargo clippy ────────────────────────────────────────────(ansi reset)"
        run-external "cargo" "clippy" "--workspace" "--all-targets" "--" "-D" "warnings"

        print ""
        print $"(ansi cyan)── cargo test ──────────────────────────────────────────────(ansi reset)"
        run-external "cargo" "test" "--workspace"

    }

    print ""
    print $"(ansi cyan)── cargo update ────────────────────────────────────────────(ansi reset)"
    run-external "cargo" "update" "-p" "weatherman-core" "-p" "weatherman" "-p" "weatherman-tui"

    print ""
    print $"(ansi cyan)── changelog ───────────────────────────────────────────────(ansi reset)"
    if (which git-cliff | is-not-empty) {
        run-external "git-cliff" "--output" "CHANGELOG.md" "--tag" $"v($new_version)"
        print $"(ansi green)✓(ansi reset) CHANGELOG.md updated via git-cliff."
    } else {
        print $"(ansi yellow)⚠(ansi reset) git-cliff not found — skipping changelog generation."
    }

    print ""
    print $"(ansi cyan)── git commit & tag ────────────────────────────────────────(ansi reset)"
    run-external "git" "add" "-A"
    run-external "git" "commit" "-m" $"chore: bump version to ($new_version)"
    run-external "git" "tag" "-a" $"v($new_version)" "-m" $"Release v($new_version)"
    print $"(ansi green)✓(ansi reset) Committed and tagged v($new_version)."

    print ""
    print $"(ansi green)  weatherman version bumped to ($new_version) 🚀(ansi reset)"
    print "  Next: git push --follow-tags origin main"
    print ""
}
