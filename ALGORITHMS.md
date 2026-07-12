# WayPenguin Algorithms

This file documents the current behavior implemented in code (v0.1.x line).

## Runtime scope

- Active runtime backend: KDE Plasma Wayland (KWin) via `waypenguin-kde`.
- GNOME and Hyprland backend crates exist but currently return `Unsupported` for window creation.

## Pet state machine

States used by `PetState`:

- Idle, Walk, Run, Sleep, Wake, LookAround, FollowCursor
- Fall, Land, Celebrate, Wander, Peck
- Float, Tumble, Climb, Stomp
- Squish, Splat, Angel, Action

Main transitions:

- Fall -> Land when `y >= floor_y`, then Land -> Idle after ~300 ms.
- Cursor moved:
- Run if cursor speed is high (`> 0.3` px/ms) or pet was sleeping.
- Walk if cursor is far (`dist_to_cursor > 100`).
- Idle if cursor is near.
- Idle timeout -> Sleep after archetype-specific timeout.
- Idle cooldown -> LookAround, then back to Idle after ~800 ms.
- Idle behavior picker can choose Wander / Float / Tumble / Climb / Stomp / Idle.
- Squish -> (Splat 70% or Angel 30%) -> respawn (about 2 s path).

Locomotion safety:

- `arrive_at_target()` forces Walk/Run/Wander/Wake/FollowCursor back to Idle at destination.
- This prevents frozen locomotion poses when already on target.

## Movement and physics

- Tick uses scaled delta: `dt = dt_ms / 16.67`.
- Gravity while airborne: `vy += 0.4 * dt`.
- Ground is `floor_y = screen_h - 130`.
- Horizontal bounds use logical pet width (`self.width`, set to 90 in daemon).
- Near-target ease-out:
- If `dist < 30`, movement step is scaled by `dist / 30`.
- Stop threshold is `dist <= 0.5`.

Special movement:

- Float: sinusoidal bob (`sin(phase) * 15`), timer 3-8 s, then switches to Fall.
- Tumble: fast horizontal floor movement; retargets on edges.
- Climb: waypoint path around vertical/horizontal screen edges.
- Stomp: top-of-screen traverse, then return to floor Idle.

## Multi-pet separation and reactions

- Constant in daemon: `AVOID_DIST = 120`.
- `apply_avoidance()` pushes both target and current position away from nearby pets.
- Repulsion strength:
- base 6x when inside min distance.
- boosted 12x when very close (`dist < min_dist * 0.3`).
- Idle/LookAround drift nudges pets apart if neighbors are within 150 px.
- Social reaction: if another pet is closer than 40 px and cooldown elapsed, enter Celebrate for ~600 ms.

## Archetypes

Archetypes influence behavior weights and movement parameters:

- ground-dweller
- floater
- climber
- tumbler
- explorer
- balanced

Each archetype defines:

- idle/wander/float/tumble/climb/stomp weights
- speed multiplier
- sleep timeout
- attraction radius
- bounce energy

Archetype is chosen randomly on spawn.

## Render pipeline (current)

The daemon uses CPU SHM composition per pet window (`PET_SIZE = 90`):

1. `waypenguin-assets::get_activity_frames(activity, 90)` rasterizes vector activities.
2. Frames are concatenated into cached per-activity sheets at startup.
3. Per tick: select activity from `PetState`, compute frame index from fps/state.
4. Clear pet buffer.
5. Render contact shadow (`render_contact_shadow`) with air fade and run stretch.
6. Apply breathing vertical offset for still states (`Idle`, `LookAround`, `Sleep`, `Action`).
7. Composite frame with optional horizontal flip (`composite_frame`).
8. Present ARGB pixels to layer-shell surface.

Renderer notes:

- `alpha_blend()` is used for proper ARGB composition.
- `render_ambient_shadow()` exists but is not used in daemon main loop.
- `squash_stretch()` exists in renderer but is currently not consumed by daemon.

## State -> visual activity mapping

Current mapping in daemon:

- `action0`: Idle, Sleep, Celebrate, Peck, Action
- `walker`: Walk, Run, Wander, LookAround, Wake, FollowCursor
- `faller`: Fall
- `tumbler`: Land, Tumble
- `floater`: Float
- `climber`: Climb, Stomp
- `splatted`: Squish, Splat
- `angel`: Angel

If one activity is missing, daemon falls back to `walker`, then `action0`.
