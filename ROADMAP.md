# WayPenguin Roadmap

Status key: Done | In progress | Planned

## Current state

- Version: v0.1.0 (first public release)
- Runtime support: KDE Plasma 6 on Wayland (KWin)
- GNOME and Hyprland backend crates exist but are not runtime-ready yet

## v0.1.0 - First public release (Done)

- Native Wayland overlay daemon for KDE
- Core pet state machine and animation loop
- Multi-pet support with basic interactions
- Procedural/vector activity asset pipeline
- Release build script for executable, tar.gz, deb, and rpm artifacts

## v0.2 - Stability and packaging polish (In progress)

- Improve release packaging reliability for x86_64 and arm64
- Add clearer install docs and release notes workflow
- Add/validate configuration surface (count, size, behavior tuning)
- Improve runtime resilience and error handling around backend events

## v0.3 - Display and movement quality (Planned)

- Better multi-screen behavior
- Better HiDPI handling and scaling consistency
- Edge and floor motion polish (turning, bobbing, movement smoothing)

## v0.4 - GNOME backend (Planned)

- Implement GNOME/Mutter backend to runtime parity with KDE
- Validate feature parity for pointer, window management, and rendering

## v0.5 - Hyprland backend (Planned)

- Implement Hyprland backend to runtime parity with KDE

## v0.6 - Wider compositor coverage (Planned)

- Add Sway or generic wlr fallback backend
- Start cross-compositor test matrix

## v1.0 - Stable Linux release (Planned)

- Multi-backend support considered production-ready
- Installation channels polished (at least binary/tarball + package manager options)
- Documentation and contributor workflow stabilized

## Notes

- No fixed release dates.
- Scope may be adjusted from real-world testing feedback.
