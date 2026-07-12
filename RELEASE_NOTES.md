# Release Draft

Tag: v0.2.1
Version: 0.2.1
Title: WayPenguin v0.2.1 - Movement Stability Patch
Date: 2026-07-12

## Summary
v0.2.1 is a patch release focused on movement reliability: reduced bottom freeze behavior, stronger cursor avoidance, and better corner distribution.

## Compatibility
- Supported now: KDE Plasma 6 on Wayland (KWin backend).
- Not supported yet: GNOME and Hyprland runtime backends (stubs remain, planned for v0.4/v0.5).

## Release Notes
- Fixed: pets now flee when cursor overlaps their body area, not only when nearby at floor level.
- Fixed: edge-clamped flee targets now choose an opposite-side fallback to prevent apparent frozen run states.
- Fixed: idle/sleep watchdog reduces long immobile periods at bottom by auto-waking and forcing movement bursts.
- Improved: wander target selection explicitly includes both corners, reducing right-corner clustering.
- Breaking changes: none.

## Assets to Upload
- waypenguin-daemon-linux-x86_64
- waypenguin-daemon-linux-arm64
- waypenguin-daemon-0.2.1-linux-x86_64.tar.gz
- waypenguin-daemon-0.2.1-linux-arm64.tar.gz
- waypenguin_0.2.1_amd64.deb
- waypenguin_0.2.1_arm64.deb
- waypenguin-daemon-0.2.1-1.x86_64.rpm
- waypenguin-daemon-0.2.1-1.aarch64.rpm

## Build Commands
- Build all configured release artifacts: ./scripts/build-releases.sh
- Build only x86_64: RELEASE_ARCHES="x86_64" ./scripts/build-releases.sh
- Build only arm64: RELEASE_ARCHES="arm64" ./scripts/build-releases.sh

## Verification Checklist
- Build script completed successfully for intended architectures.
- Daemon starts and runs correctly on KDE Plasma Wayland.
- Pets flee immediately when cursor is directly over them.
- No prolonged bottom freeze under multi-pet load (example: `-n 13`).
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
