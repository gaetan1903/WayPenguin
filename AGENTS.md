# WayPenguin Agent Rules

This file defines conventions and constraints for AI-assisted coding (opencode, Copilot, Cursor, etc.).  
Human contributors may also find it useful.

---

## 1. Project identity

- Project name: **WayPenguin**
- Language: **Rust (edition 2021)**
- Windowing: **smithay-client-toolkit + wayland-client** (no X11, no XWayland, no Electron, no Qt, no GTK)
- Rendering: **SHM buffers** (CPU), composited via layer-shell overlay
- Config: **serde + JSON**

---

## 2. Architecture constraints

### 2.1 Core is sacred

`waypenguin-core` must never depend on:
- Any Wayland crate
- Any compositor-specific crate
- Any GUI toolkit

It may depend on `serde` and `serde_json`.

### 2.2 Backend abstraction

Every compositor backend is a separate crate in `waypenguin-backends/<name>/`.

All backends implement `DesktopBackend` and `DesktopWindow` from `waypenguin-backends`.

Never add compositor-specific logic outside a backend crate.

Never add `#[cfg(target_os = "...")]` in core crates — that belongs in backends.

### 2.3 Crate dependency flow

```
waypenguin-daemon
  -> waypenguin-core
  -> waypenguin-renderer   (-> waypenguin-core)
  -> waypenguin-backends
  -> waypenguin-<backend>  (-> waypenguin-backends)
  -> waypenguin-assets
```

`waypenguin-renderer` may depend on `waypenguin-core`.  
Backend crates may depend on `waypenguin-backends` and Wayland crates.  
No circular dependencies.

### 2.4 Asset loading

Assets (sprites, sounds, themes) are loaded via `waypenguin-assets`.  
Procedural generation is preferred for the MVP.  
File-based assets (PNG, JSON manifests) are added later.

---

## 3. Code style

### 3.1 No unnecessary comments

The codebase follows the philosophy: **the code IS the documentation**.

❌ Avoid:
```rust
// Increment x by 1
x += 1;
```

✅ Acceptable:
```rust
/// Returns the interpolated position after applying delta-time movement.
pub fn update_movement(&mut self, dt_ms: u32) { ... }
```

Public API items get doc comments (`///`).  
Internal logic gets no comments unless the algorithm is genuinely non-obvious.

### 3.2 Formatting

- `cargo fmt` is mandatory before every commit.
- Line length: default rustfmt (100 chars).
- Indentation: 4 spaces.

### 3.3 Naming

- Types: `PascalCase` (`PetState`, `AnimationFrame`, `KdeBackend`)
- Functions/methods: `snake_case` (`update_ai`, `get_cursor_position`)
- Variables: `snake_case` (`pet_window_x`, `frame_buffer`)
- Private items: no leading underscore unless needed for unused-parameter silencing
- Avoid abbreviations unless universally understood (`shm`, `wl`, `sctk`)

### 3.4 Match statements

Exhaustive matches are preferred.  
When adding a new variant to `PetState`, the compiler will catch every match — add the new arm everywhere.

### 3.5 Error handling

- Use `BackendError` for backend operations.
- Use `anyhow` only in binaries (daemon, CLI). Libraries return domain-specific errors.
- Prefer `map_err` over `.expect()` or `.unwrap()`.
- `unwrap()` is only acceptable in tests.

---

## 4. Testing rules

- Every new function in `waypenguin-core` should have a unit test.
- AI state transitions must have at least one test per transition pair.
- Renderer tests must verify pixel output at the buffer level.
- Tests must be fast (under 100 ms per crate).
- Run `cargo test` before every commit.

---

## 5. documentation

- `README.md` — project overview, quick start, architecture
- `ROADMAP.md` — version milestones and feature tracking
- `CONTRIBUTING.md` — how humans contribute
- `AGENTS.md` — this file, for AI tooling constraints
- API docs via `///` on `pub` items only

No other Markdown files without explicit user request.

---

## 6. AI-specific rules

### 6.1 When generating code

1. **Read before writing.** Always read the file you are about to edit.
2. **Follow existing patterns.** Use the same imports, same macro usage, same error handling as neighboring code.
3. **Don't add unused dependencies.** Cargo.toml changes must be justified.
4. **Don't add `#[allow(...)]`** unless truly unavoidable (and explain why).
5. **Test before declaring done.** After writing code, run `cargo build` and `cargo test`.

### 6.2 When refactoring

- Prefer small, focused edits over full-file rewrites.
- If a full rewrite is necessary, explain why in a comment at the top of the file.
- Use the `edit` tool for targeted changes, `write` only when the file is new.

### 6.3 Security

- Never commit secrets, tokens, or keys.
- Never introduce `unsafe` code unless absolutely required (e.g., `SlotPool` buffer access).
- Every `unsafe` block must have a `// SAFETY:` comment.

### 6.4 Dependency etiquette

- Check `Cargo.lock` before adding a new crate — it may already be a transitive dep.
- Prefer widening an existing dependency version over adding a duplicate.
- If a new dependency is needed, add it to the specific crate's `Cargo.toml`, not the workspace root.

---

## 7. Git conventions

- Commits follow [conventional commits](https://www.conventionalcommits.org/).
- Keep commits small and focused.
- Don't commit `Cargo.lock` changes unless dependencies changed.
- Don't commit build artifacts (`target/`).

---

## 8. CI expectations

The CI (once set up) will:

1. `cargo build` — all targets
2. `cargo test` — all crates
3. `cargo clippy -- -D warnings` — zero warnings
4. `cargo fmt --check` — formatting compliance

PRs that fail any of these will not be merged.
