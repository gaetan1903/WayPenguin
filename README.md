# WayPenguin

> A modern, native Wayland desktop pet — lightweight, GPU-accelerated, and built in Rust.

Tux the penguin walks, runs, sleeps, and follows your cursor on KDE Plasma 6, with zero desktop interference.

**Status Badges:**

| Desktop Support | Status |
|---|---|
| **KDE Plasma 6** | ✓ Working |
| **GNOME** | Planned (help wanted) |
| **Hyprland** | Planned |
| **Sway** | Planned |

| Project Status | Value |
|---|---|
| **Version** | v0.2.0 |
| **Maturity** | Alpha |
| **Language** | Rust (edition 2021) |
| **License** | MIT |

---

## Features

- **Native Wayland** — wlr-layer-shell overlay, no XWayland, no X11
- **Click-through** — never steals focus, never blocks input
- **AI-driven** — Tux sleeps when idle, wakes on cursor movement, runs when you move fast
- **Multi-screen** — detects all outputs and moves across them
- **HiDPI** — respects output scale factors
- **Lightweight** — `< 20 MB RAM`, `< 1% CPU`, no Electron, no Qt, no GTK
- **Procedural asset** — Tux is generated in code; no external files needed
- **Backend abstraction** — KDE Plasma works now; GNOME, Hyprland, Sway are prepared

---


[Screencast_DEMO.webm](https://github.com/user-attachments/assets/554b307e-105b-46cd-a0b8-67eaba17d803)


## Quick start

```bash
# Build and run the daemon on KDE Plasma 6 (Wayland session)
cargo run --bin waypenguin-daemon
```

---

## Command-line options

```
waypenguin-daemon [OPTIONS]
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--count <N>` | `-n` | `5` | Number of pets to spawn (1–50) |
| `--pack <id>` | `-p` | *(built-in default)* | Pet pack to use, by pack ID |
| `--data <path>` | `-d` | *(XDG data dir)* | Custom path to the pets directory |
| `--list` | `-l` | — | List all discovered packs and exit |

### Examples

```bash
# Spawn 1 pet using the ladybug pack
waypenguin-daemon --count 1 --pack ladybug-classic

# List all available packs (built-in and user-installed)
waypenguin-daemon --list

# Use a custom pets folder
waypenguin-daemon --data ~/.local/share/my-pets --pack my-custom-pack
```

---

## Themes (pet packs)

Use `--pack <id>` to select a theme. Run `--list` to see all packs installed on your system.

| ID | Name | Preview | Description |
|----|------|---------|-------------|
| `tux-alpha` | Tux (Alpha) | ![tux-alpha walker](waypenguin-assets/svg/tux-alpha/walker.svg) | The classic Linux mascot, hand-drawn as clean vector poses. |
| `beetle-azure` | Beetle (Azure) | ![beetle-azure walker](waypenguin-assets/svg/beetle-azure/walker.svg) | Cyberpunk black beetle with glowing cyan eyes. |
| `beetle-gold` | Beetle (Gold) | ![beetle-gold walker](waypenguin-assets/svg/beetle-gold/walker.svg) | Cinematic amber-gold beetle with dark spot markings. |
| `beetle-jade` | Beetle (Jade) | ![beetle-jade walker](waypenguin-assets/svg/beetle-jade/walker.svg) | Iridescent emerald beetle with gold-tinted specular. |
| `beetle-void` | Beetle (Void) | ![beetle-void walker](waypenguin-assets/svg/beetle-void/walker.svg) | Matte obsidian beetle with dark metallic finish. |
| `ladybug-classic` | Ladybug (Classic) | ![ladybug-classic walker](waypenguin-assets/svg/ladybug-classic/walker.svg) | Cinematic red ladybug with gloss dome shell and black spots. |

### Custom packs

Drop a directory containing a `pack.toml` and SVG pose files into your pets folder (`~/.local/share/waypenguin/pets/` by default, or the path set with `--data`). The `walker` activity is required; all others are optional. Run `--list` to confirm the pack is discovered.

```toml
# pack.toml — minimal example
[pack]
id          = "my-pet"
name        = "My Custom Pet"
author      = "You"
license     = "CC-BY-4.0"
version     = "1.0.0"
description = "A custom desktop pet."

[activities]
walker = "walker.svg"
```

## Install from GitHub Releases

Download a pre-built executable for your architecture and add it to your PATH.

### 1. Download

```bash
TAG="v0.1.0"
ARCH="x86_64"  # or aarch64 for arm64
curl -fLO "https://github.com/<owner>/<repo>/releases/download/${TAG}/waypenguin-daemon-linux-${ARCH}"
```

### 2. Install to PATH

Make executable and move to a directory in your `PATH`:

```bash
chmod +x waypenguin-daemon-linux-${ARCH}
sudo mv waypenguin-daemon-linux-${ARCH} /usr/local/bin/waypenguin-daemon
```

Or use your home directory PATH:

```bash
mkdir -p ~/.local/bin
mv waypenguin-daemon-linux-${ARCH} ~/.local/bin/waypenguin-daemon
export PATH="$HOME/.local/bin:$PATH"
```

### 3. Run

```bash
waypenguin-daemon
```

### Runtime dependencies

Most modern Linux distributions include these. If you encounter missing libraries:

```bash
# Debian / Ubuntu
sudo apt install -y libwayland-client0 libwayland-cursor0 libxkbcommon0

# Fedora / RHEL / Rocky
sudo dnf install -y wayland libxkbcommon
```

### Update

Download the new executable and repeat steps 2-3.

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

See [ROADMAP.md](./ROADMAP.md) for details.

---

## GNOME Support — Help Wanted

The GNOME backend crate exists ([`waypenguin-backends/gnome/`](./waypenguin-backends/gnome/)) but requires **runtime validation and fixes**. If you use GNOME Wayland, we need help with:

- **Skeleton in place** — basic layer-shell, pointer, and output handling
- **Testing on real GNOME sessions** — verify cursor tracking, window positioning, layer-shell anchoring
- **Debugging** — layer-shell protocol differences between GNOME/Mutter and KWin
- **Compositor flags** — document any special Mutter properties or environment variables needed
- **Edge cases** — multi-monitor setups, HiDPI scaling, Wayland protocol version differences

**To help:** Open an issue describing your GNOME version, Wayland session details, and any errors you encounter. Patches welcome.

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
