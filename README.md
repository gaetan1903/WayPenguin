# WayPenguin

> A modern, native Wayland desktop pet — lightweight, GPU-accelerated, and built in Rust.

Tux the penguin walks, runs, sleeps, and follows your cursor on KDE Plasma 6, with zero desktop interference.

---

## Features

- **Native Wayland** — wlr-layer-shell overlay, no XWayland, no X11
- **Click-through** — never steals focus, never blocks input
- **AI-driven** — Tux sleeps when idle, wakes on cursor movement, runs when you move fast
- **Multi-screen** — detects all outputs and moves across them
- **HiDPI** — respects output scale factors
- **Lightweight** — `< 30 MB RAM`, `< 1% CPU`, no Electron, no Qt, no GTK
- **Procedural asset** — Tux is generated in code; no external files needed
- **Backend abstraction** — KDE Plasma works now; GNOME, Hyprland, Sway are prepared

---

## Quick start

```bash
# Build and run the daemon on KDE Plasma 6 (Wayland session)
cargo run --bin waypenguin-daemon
```

### Requirements

- Linux with a **KDE Plasma 6** Wayland session (KWin)
- Rust toolchain (edition 2021)
- `libwayland-client`, `libwayland-cursor`, `libxkbcommon`

---

## Architecture

```
waypenguin-core/          # Types, AI engine, animation state machine
waypenguin-renderer/      # Pixel rendering & frame compositing
waypenguin-backends/      # DesktopBackend / DesktopWindow traits
  kde/                # KDE/KWin backend (functional)
  gnome/              # GNOME/Mutter backend (stub, ready for implementation)
  hyprland/           # Hyprland backend (stub, ready for implementation)
waypenguin-assets/        # Procedural spritesheet generation (Tux)
waypenguin-daemon/        # Main loop: backend, AI, render, present
waypenguin-cli/           # CLI tool (placeholder)
waypenguin-settings/      # Settings GUI (placeholder)
```

### Backend trait

Every compositor backend implements `DesktopBackend` + `DesktopWindow`:

```rust
pub trait DesktopBackend {
    fn get_screens(&self) -> Vec<ScreenInfo>;
    fn get_cursor_position(&self) -> (i32, i32);
    fn create_window(…) -> Result<Box<dyn DesktopWindow>, BackendError>;
}
```

Adding a new compositor = impl the trait. No core changes needed.

---

## Roadmap

| Version | Milestone |
|---------|-----------|
| V0.1 | Transparent overlay, Tux display, idle animation |
| V0.2 | Movement & cursor tracking |
| V0.3 | AI behavior (sleep/wake/walk/run) |
| V0.4 | Settings application |
| V0.5 | Multi-screen & HiDPI polish |
| V0.6 | GNOME backend |
| V0.7 | Hyprland + Sway backends |
| V1.0 | Flatpak, RPM, DEB, AppImage release |

See [ROADMAP.md](./ROADMAP.md) for details.

---

## Tech stack

| Concern | Choice | Why |
|---------|--------|-----|
| Language | **Rust** | Performance, memory safety, zero-cost abstractions |
| Windowing | **smithay-client-toolkit** | Idiomatic Wayland in Rust, layer-shell support |
| Rendering | **SHM + software** | Zero GPU deps for MVP; Vulkan/Metal planned |
| Config | **serde + JSON** | Portable, human-editable, well-known |

---

## Contributing

Please read [CONTRIBUTING.md](./CONTRIBUTING.md) and [AGENTS.md](./AGENTS.md) before submitting PRs.

---

## License

MIT — see [LICENSE](./LICENSE).
