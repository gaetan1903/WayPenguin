# WayPenguin Roadmap

**Status key:** ✅ Done | 🔧 In progress | ⏳ Planned | 📝 Backlog

---

## V0.1 — Foundation *(MVP complete)*

- [x] Workspace structure with all 10 crates
- [x] Core types (`Pet`, `PetState`, `AnimationFrame`, `SpriteSheet`, `AnimationSpec`)
- [x] Backend trait definitions (`DesktopBackend`, `DesktopWindow`)
- [x] KDE/KWin backend via `wlr-layer-shell`
- [x] Transparent overlay window (Overlay layer, click-through via empty input region)
- [x] SHM pool buffer management
- [x] Procedural Tux spritesheet generation (4 frames)
- [x] Frame renderer with horizontal flip
- [x] Main daemon loop (Wayland dispatch, render, present)
- [x] `cargo fmt` / `cargo clippy` clean

## V0.2 — Cursor tracking

- [x] Seat binding (`wl_seat`)
- [x] Pointer event handling (enter / motion / leave)
- [x] Relative pointer tracking (`zwp_relative_pointer_manager_v1`)
- [x] Cursor position accumulation across screen boundaries
- [ ] Fallback when `zwp_relative_pointer` is unavailable
- [ ] Cursor position smoothing / filtering

## V0.3 — AI behavior *(done)*

- [x] State machine: Idle ↔ Walk ↔ Run ↔ Sleep
- [x] Sleep after 10s cursor inactivity
- [x] Wake on cursor movement
- [x] Run on fast cursor
- [x] Walk toward cursor when distant
- [x] Screen edge confinement
- [x] Unit tests for AI transitions

## V0.4 — Animation system *(done)*

- [x] Walking animation (4-phase swaying cycle)
- [x] Running animation (4-phase fast cycle with open beak)
- [x] Sleeping animation (crouched, closed eyes, Zzz bubbles)
- [x] Wake-up transition animation
- [x] Procedural wing / beak movement per state

## V0.5 — Multi-pet engine *(done)*

- [x] N pets with independent AI and animation
- [x] Spawn position distribution across screen bottom
- [x] Pet-avoidance via repulsion force
- [x] Command-line arg for pet count (`-n 5`, default 5)
- [x] Per-pet window creation and rendering

## V0.6 — Falling & physics

- [x] Spawn Tux at top of screen and fall with gravity
- [x] Landing bounce animation
- [ ] Screen edge collision (walk into wall = turn around)
- [ ] Slight floor unevenness (walk bob)
- [ ] Fun physics: slide on fast cursor move

## V0.7 — Autonomous behaviors *(done)*

- [x] Idle exploration (random walk within bounds)
- [x] Reaction to other pets (wave, group)
- [x] Cursor attraction radius (not just follow)
- [x] Peck at screen edges / corners
- [x] Random look-around while idle

## V0.8 — Archetypes, death & dynamic movement *(done)*

- [x] PetArchetype system: ground-dweller, floater, climber, tumbler, explorer, balanced
- [x] Random archetype per pet at spawn
- [x] Float/levitate: hover with gentle bobbing at random heights
- [x] Tumble: fast rolling on the floor, bounce off edges
- [x] Climb: walk up screen edge → across top → down other side
- [x] Stomp: walk across TOP of screen (simulate window-top walking)
- [x] Archetype-weighted idle behavior selection (pick random activity based on personality)
- [x] Archetype affects speed, attraction radius, sleep timeout
- [x] Squish: click pet → Squish → Splat (blood) or Angel (ascend) → respawn from top
- [x] Short idle times: pets are always moving, rarely still
- [x] Pointer re-enabled on overlay for click interaction
- [x] DesktopBackend trait: get_last_click, clear_last_click, get_window_geometries

## V0.9 — Multi-screen

- [ ] Pets can move across monitor boundaries
- [ ] Per-monitor scale factor support
- [ ] DPI-aware rendering
- [ ] Pets distributed across all screens

## V0.10 — Theme system

- [ ] `manifest.json` theme format
- [ ] Sprite sheet loading from PNG
- [ ] Animation metadata parsing
- [ ] Theme directory (`~/.config/waypenguin/themes/`)
- [ ] Procedural → file-based asset fallback
- [ ] Sound effect support (idle chirps, footsteps)

## V0.11 — Settings & configuration

- [ ] JSON config file (`~/.config/waypenguin/config.json`)
- [ ] Configurable pet count, size, speed, idle timeout
- [ ] `waypenguin-cli` commands: `start`, `stop`, `config`, `list-themes`
- [ ] Settings GUI application (`waypenguin-settings`)

## V0.12 — Backends (GNOME, Hyprland, Sway)

- [ ] GNOME/Mutter implementation
- [ ] Hyprland implementation
- [ ] Sway implementation
- [ ] Generic wlr-compositor fallback

## V0.13 — Packaging & CI

- [ ] Flatpak packaging
- [ ] RPM / DEB / AppImage / AUR / Nix
- [ ] GitHub CI (build, test, lint on all targets)
- [ ] Performance benchmarks in CI

---

*V1.0 will be declared after community validation on GitHub — no date set.*

---

## Post-V1.0 — Ecosystem

| Feature | Priority |
|---------|----------|
| Marketplace / theme sharing | Medium |
| Multiple pets simultaneously | Medium |
| Pet-to-pet interaction | Low |
| Physics engine (ragdoll, bouncing) | Low |
| Weather integration | Low |
| Day/night cycle | Low |
| MPRIS music reactions | Low |
| System notifications | Low |
| Mini-games | Low |
| Plugin system (WASI / Lua) | Very low |

---

## How versions are chosen

- **V0.x** — Pre-stable, API may break, single-backend focus
- **V1.0** — First stable release with multi-backend support, packaging, and documentation
- Minor bumps after V1.0 follow semver

No release dates. Quality over speed.
