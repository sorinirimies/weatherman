#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# Weatherman — What a release ships
# ──────────────────────────────────────────────────────────────────────────────
# The release and CI scripts read these (build_release, package, publish_crate,
# release_notes, apt, check_static). Edit them when the workspace gains a binary
# or a published crate; test_project.nu checks BINS and PUBLISH against cargo.
#
# Usage:
#   nu scripts/project.nu                  # print what a release ships
#   nu scripts/project.nu matrix           # matrix=<json> for the release workflow
#   nu scripts/project.nu gitea-targets    # the Linux targets Gitea builds
# ──────────────────────────────────────────────────────────────────────────────

# Binaries a release packages, each with the workspace crate that builds it.
export const BINS = [[crate bin]; ["weatherman" "weatherman"] ["weatherman-tui" "weatherman-tui"]]

# Crates published to crates.io, dependencies first.
export const PUBLISH = ["weatherman-core" "weatherman" "weatherman-tui"]

# System packages (apt) every Linux job that compiles installs first, e.g. the
# headers a GUI toolkit links against: ["libxkbcommon-dev"].
export const APT = ["libxkbcommon-dev"]

# Linux release binaries: "musl" (static, any distro, cargo-zigbuild) or "gnu"
# (glibc, built natively; for GUI apps, which can not link statically).
export const LINUX = "gnu"

# The GitHub release build matrix for a LINUX flavour, one row per target.
export def release_matrix [linux: string]: nothing -> list<record> {
    let lin = if $linux == "gnu" {
        [
            {runner: ubuntu-latest, target: x86_64-unknown-linux-gnu, zig: false}
            {runner: ubuntu-24.04-arm, target: aarch64-unknown-linux-gnu, zig: false}
        ]
    } else {
        [
            {runner: ubuntu-latest, target: x86_64-unknown-linux-musl, zig: true}
            {runner: ubuntu-latest, target: aarch64-unknown-linux-musl, zig: true}
        ]
    }
    [
        {runner: macos-latest, target: aarch64-apple-darwin, zig: false}
        {runner: macos-latest, target: x86_64-apple-darwin, zig: false}
        ...$lin
        {runner: windows-latest, target: x86_64-pc-windows-msvc, zig: false}
    ]
}

# The Linux targets Gitea (one x86-64 runner) builds for a LINUX flavour.
export def gitea_targets [linux: string]: nothing -> list<string> {
    if $linux == "gnu" { [x86_64-unknown-linux-gnu] } else { [x86_64-unknown-linux-musl aarch64-unknown-linux-musl] }
}

# Crates that own a shipped binary, each once.
export def bin_crates []: nothing -> list<string> {
    $BINS | get -o crate | default [] | uniq
}

# `matrix=<json>` for $GITHUB_OUTPUT.
def "main matrix" [] {
    print $"matrix=({include: (release_matrix $LINUX)} | to json --raw)"
}

def "main gitea-targets" [] {
    print (gitea_targets $LINUX | str join " ")
}

def main [] {
    let bins = $BINS | get -o bin | default []
    print $"binaries=($bins | str join ',')"
    print $"crates_io=($PUBLISH | str join ',')"
}
