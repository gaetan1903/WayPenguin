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

## Install from GitHub Releases (DEB/RPM)

Use these steps when you publish release packages on GitHub.

1. Open the Releases page and choose a tag (example: v0.1.0).
2. Download the package matching your distro and CPU architecture.
3. Install with your package manager.

### Debian / Ubuntu (.deb)

Install runtime dependencies (first time only):

```bash
sudo apt update
sudo apt install -y libwayland-client0 libwayland-cursor0 libxkbcommon0
```

Download and install the release package:

```bash
TAG="v0.1.0"
curl -fLO "https://github.com/<owner>/<repo>/releases/download/${TAG}/waypenguin_${TAG#v}_amd64.deb"
sudo apt install -y "./waypenguin_${TAG#v}_amd64.deb"
```

### Fedora / RHEL / Rocky / AlmaLinux (.rpm)

Install runtime dependencies (first time only):

```bash
sudo dnf install -y wayland libxkbcommon
```

Download and install the release package:

```bash
TAG="v0.1.0"
curl -fLO "https://github.com/<owner>/<repo>/releases/download/${TAG}/waypenguin-${TAG#v}-1.x86_64.rpm"
sudo dnf install -y "./waypenguin-${TAG#v}-1.x86_64.rpm"
```

### Run after installation

```bash
waypenguin-daemon
```

### Update and uninstall

```bash
# Debian / Ubuntu
sudo apt install -y ./waypenguin_<version>_amd64.deb
sudo apt remove -y waypenguin

# Fedora / RHEL
sudo dnf upgrade -y ./waypenguin-<version>-1.x86_64.rpm
sudo dnf remove -y waypenguin
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
| V0.1 | First public release (KDE Plasma Wayland support, AI states, multi-pet, procedural pose assets) |
| V0.2 | Packaging polish (binary + DEB/RPM flow), config file, CLI basics |
| V0.3 | Multi-screen and HiDPI improvements |
| V0.4 | GNOME backend implementation |
| V0.5 | Hyprland backend implementation |
| V0.6 | Sway or generic wlr fallback backend |
| V1.0 | Stable multi-backend Linux release and distribution channels (Flatpak/AppImage/DEB/RPM) |

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
