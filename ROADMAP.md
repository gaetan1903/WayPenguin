# WayPenguin Roadmap

Status key: Done | In progress | Planned

## Current state

- Runtime support: KDE Plasma 6 on Wayland (KWin)
- GNOME and Hyprland backend crates exist but are not runtime-ready yet

## Planned work

### Core features
- [x] Native Wayland overlay daemon for KDE
- [x] Core pet state machine and animation loop
- [x] Multi-pet support with basic interactions
- [x] Procedural/vector activity asset pipeline
- [x] Release build script for executable
- [x] Built-in multi-theme pet packs (tux-alpha, beetle-*, ladybug-classic)
- [x] Custom user pack support via pack.toml manifest
- [x] CLI flags: --count, --pack, --data, --list

### Stability & packaging
- [x] Improve release packaging reliability for x86_64 and arm64
- [x] Configuration surface (count, pet pack selection, custom data path via CLI flags)
- [ ] Improve runtime resilience and error handling

### Display & movement quality
- [ ] Better multi-screen behavior
- [ ] HiDPI handling and scaling consistency
- [ ] Edge and floor motion polish (turning, bobbing, smoothing)

### Compositor support
- [ ] GNOME Wayland backend (runtime parity with KDE)
- [ ] Hyprland backend (runtime parity with KDE)
- [ ] Sway or generic wlr fallback backend
- [ ] Cross-compositor test matrix

### Stability & production readiness
- [ ] Multi-backend support production-ready
- [ ] Installation and update channels stabilized
- [ ] Documentation and contributor workflow

## Notes

- No fixed release dates.
- Scope may be adjusted from real-world testing feedback.
