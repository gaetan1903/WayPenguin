# Release Draft

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
