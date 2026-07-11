# Contributing to WayPenguin

First off, thank you for your interest. WayPenguin aims to be the reference desktop pet for Wayland — every contribution, from a typo fix to a new backend, is valued.

---

## Before you start

- Read [ROADMAP.md](./ROADMAP.md) to see what's planned.
- Read [AGENTS.md](./AGENTS.md) if you use AI-assisted tooling.
- Open an issue or discussion for **significant** changes before writing code.

---

## Code of conduct

Be excellent to each other. No harassment, no entitlement, no drama. This is a hobby project built for fun.

---

## How to contribute

### 1. Find something to work on

- Check the [ROADMAP.md](./ROADMAP.md) for upcoming milestones.
- Look for issues tagged `good first issue` or `help wanted`.
- If nothing fits, open an issue describing what you want to build.

### 2. Set up your environment

```bash
git clone https://github.com/your-org/waypenguin
cd waypenguin
cargo build
cargo test
```

You need a **KDE Plasma 6 Wayland session** to run the daemon. All other development (core, renderer, backends) can be done on any system with `cargo test`.

### 3. Make your changes

- Follow the [coding guidelines](#coding-guidelines) below.
- Write tests for new logic.
- Run `cargo fmt` and `cargo clippy` before committing.
- Keep the diff focused on one concern per commit.

### 4. Submit a PR

- Use a descriptive title (e.g., `feat: add walking animation for Run state`).
- Reference the issue number if applicable.
- In the description, explain **what** and **why** (not how — the diff shows that).
- Every PR must include:
  - Code changes
  - Tests (unless trivial)
  - Updated documentation if relevant

---

## Coding guidelines

### Rust style

- Follow standard Rust idioms. The codebase uses `edition 2021`.
- **No unnecessary comments.** Code should be self-documenting. Use doc comments (`///`) only for public API surfaces.
- Use `cargo fmt` — the CI will reject incorrectly formatted code.
- Run `cargo clippy` — zero warnings required.

### Architecture rules

- **Core never depends on a compositor.** `waypenguin-core` must remain backend-agnostic.
- **Backends implement traits.** Never add compositor-specific code outside `waypenguin-backends/<name>/`.
- **No KDE-specific code in generic crates.** The `DesktopBackend` trait exists precisely to avoid this.
- **No Electron. No Qt. No GTK.** Rendering is done via SHM buffers; the UI via layer-shell.

### Module isolation

Each functional unit is a separate crate:

| Crate | Responsibility | May depend on |
|-------|---------------|---------------|
| `waypenguin-core` | Types, AI, state machine | Nothing Wayland-specific |
| `waypenguin-renderer` | Pixel composition | `waypenguin-core` |
| `waypenguin-backends` | Traits | Nothing |
| `waypenguin-*` backends | Compositor glue | `waypenguin-backends`, Wayland crates |
| `waypenguin-assets` | Asset loading / generation | Nothing |
| `waypenguin-daemon` | Main binary | All of the above |

### Testing

- `#[cfg(test)] mod tests { ... }` in every crate.
- Tests must be fast (`< 100 ms`).
- New AI logic needs a unit test for each transition.
- New rendering logic needs a pixel-level test.
- Backend tests require a Wayland compositor — they are run manually.

### Dependencies

- **Minimize dependencies.** Every new `[dependencies]` entry must be justified.
- Prefer `smithay-client-toolkit` over raw `wayland-client` for complex compositor interactions.
- Avoid pull-in of large frameworks (Qt, GTK, Electron, etc.).

---

## Commit messages

Follow conventional commits:

```
<type>: <short summary>

<optional body>
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `perf`, `chore`, `ci`.

Examples:

```
feat: add walking animation for Run state

fix: cursor position reset on screen edge

docs: explain output state in KDE backend

test: cover Sleep → Wake → Run transition
```

---

## Review process

1. CI must pass (build + test + clippy + fmt).
2. At least one maintainer reviews the code.
3. Address review comments or explain why you disagree.
4. Squash commits before merge (we use `--squash`).

---

## Questions?

Open a [GitHub Discussion](https://github.com/your-org/waypenguin/discussions).
