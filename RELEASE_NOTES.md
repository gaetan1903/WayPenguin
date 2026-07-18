# Release Draft

Tag: v0.3.0
Version: 0.3.0
Title: WayPenguin v0.3.0 - COSMIC Support & Pet Scaling
Date: 2026-07-18

## Summary
v0.3.0 adds runtime support for COSMIC desktop and introduces pet scaling configuration via both pack manifests and command-line override.

## Compatibility
- Supported now: KDE Plasma 6 and COSMIC on Wayland.
- Not supported yet: GNOME and Hyprland runtime backends (stubs remain, planned for v0.4/v0.5).

## Release Notes
- Added: COSMIC backend runtime support with daemon fallback sequence including COSMIC.
- Added: `scale` field in `pack.toml` to define per-pack default pet size multiplier.
- Added: `--scale <n>` (`-s`) CLI flag to override pack scale at launch.
- Added: scale-aware rendering and window sizing across rasterization, hit detection, and compositing.
- Improved: documentation for desktop support status, scale configuration, and CLI examples.
- Breaking changes: none.

## Assets to Upload
- waypenguin-daemon-linux-x86_64
- waypenguin-daemon-linux-arm64
- waypenguin-daemon-0.3.0-linux-x86_64.tar.gz
- waypenguin-daemon-0.3.0-linux-arm64.tar.gz
- waypenguin_0.3.0_amd64.deb
- waypenguin_0.3.0_arm64.deb
- waypenguin-daemon-0.3.0-1.x86_64.rpm
- waypenguin-daemon-0.3.0-1.aarch64.rpm

## Build Commands
- Build all configured release artifacts: ./scripts/build-releases.sh
- Build only x86_64: RELEASE_ARCHES="x86_64" ./scripts/build-releases.sh
- Build only arm64: RELEASE_ARCHES="arm64" ./scripts/build-releases.sh

## Verification Checklist
- Build script completed successfully for intended architectures.
- Daemon starts and runs correctly on KDE Plasma and COSMIC Wayland sessions.
- `--scale` works as expected (examples: `--scale 0.5`, `--scale 2.0`).
- `pack.toml` `scale` default is applied when `--scale` is not passed.
- CLI `--scale` overrides `pack.toml` scale when both are defined.
- All uploaded artifact names match this document.
- GitHub release body copied from this file and published.

---

# Previous Releases

Tag: v0.2.0
Version: 0.2.0
Title: WayPenguin v0.2.0 - Themes, CLI & Pack System
Date: 2026-07-12

## Summary
v0.2.0 adds a full pet pack system with 6 built-in themes, command-line configuration flags, and custom user pack support.

## Compatibility
- Supported now: KDE Plasma 6 on Wayland (KWin backend).
- Not supported yet: GNOME and Hyprland runtime backends (stubs remain, planned for v0.4/v0.5).

## Release Notes
- Added: `--pack <id>` flag to select a built-in or user-installed pet pack at launch.
- Added: `--count <n>` flag to set the number of pets (1–50, default 5).
- Added: `--data <path>` flag to point the daemon at a custom pets directory.
- Added: `--list` flag to enumerate all discovered packs and exit.
- Added: 6 built-in pet packs — `tux-alpha`, `beetle-azure`, `beetle-gold`, `beetle-jade`, `beetle-void`, `ladybug-classic`.
- Added: custom user pack support via `pack.toml` manifest (drop a directory into the pets folder).
- Added: `pack.toml` format with `[pack]` metadata and `[activities]` SVG mapping.
- Improved: release packaging reliability for x86_64 and arm64 builds.
- Breaking changes: none.

## Assets to Upload
- waypenguin-daemon-linux-x86_64
- waypenguin-daemon-linux-arm64
- waypenguin-daemon-0.2.0-linux-x86_64.tar.gz
- waypenguin-daemon-0.2.0-linux-arm64.tar.gz
- waypenguin_0.2.0_amd64.deb
- waypenguin_0.2.0_arm64.deb
- waypenguin-daemon-0.2.0-1.x86_64.rpm
- waypenguin-daemon-0.2.0-1.aarch64.rpm

## Build Commands
- Build all configured release artifacts: ./scripts/build-releases.sh
- Build only x86_64: RELEASE_ARCHES="x86_64" ./scripts/build-releases.sh
- Build only arm64: RELEASE_ARCHES="arm64" ./scripts/build-releases.sh

## Verification Checklist
- Build script completed successfully for intended architectures.
- Daemon starts and runs correctly on KDE Plasma Wayland.
- `--list` output shows all 6 built-in packs.
- `--pack beetle-azure --count 1` spawns one beetle correctly.
- All uploaded artifact names match this document.
- GitHub release body copied from this file and published.

Tag: v0.1.0
Version: 0.1.0
Title: WayPenguin v0.1.0 - First Public Release
Date: 2026-07-12

## Summary
First public release of WayPenguin: native Wayland desktop pet with KDE Plasma 6 support.

## Compatibility
- Supported now: KDE Plasma 6 on Wayland (KWin backend).
- Not supported yet: GNOME and Hyprland runtime backends (present as stubs, planned for future releases).

## Release Notes
- Added: initial WayPenguin daemon with AI-driven pet behavior and multiple activities.
- Added: native Wayland overlay integration using smithay-client-toolkit.
- Added: procedural vector asset pipeline and themed activity rendering.
- Added: release build automation script for executable, tar.gz, deb, and rpm outputs.
- Improved: walker pose updated with clearer silhouette and weird/cute head styling.
- Breaking changes: none.

## Assets to Upload
- waypenguin-daemon-linux-x86_64
- waypenguin-daemon-linux-arm64
- waypenguin-daemon-0.1.0-linux-x86_64.tar.gz
- waypenguin-daemon-0.1.0-linux-arm64.tar.gz
- waypenguin_0.1.0_amd64.deb
- waypenguin_0.1.0_arm64.deb
- waypenguin-daemon-0.1.0-1.x86_64.rpm
- waypenguin-daemon-0.1.0-1.aarch64.rpm

## Build Commands
- Build all configured release artifacts: ./scripts/build-releases.sh
- Build only x86_64: RELEASE_ARCHES="x86_64" ./scripts/build-releases.sh
- Build only arm64: RELEASE_ARCHES="arm64" ./scripts/build-releases.sh

## Verification Checklist
- Build script completed successfully for intended architectures.
- Daemon starts and runs correctly on KDE Plasma Wayland.
- All uploaded artifact names match this document.
- GitHub release body copied from this file and published.
