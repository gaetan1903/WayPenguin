# Pet packs

A **pet pack** is a directory of vector SVG poses plus a `pack.toml` manifest.
WayPenguin ships one built-in pack, [`tux-alpha/`](tux-alpha/), and can load
additional packs from disk at runtime.

## Directory layout

```
my-pet/
├── pack.toml        # manifest (required)
├── walker.svg       # required activity
├── faller.svg       # optional activities …
├── climber.svg
├── tumbler.svg
├── floater.svg
├── action0.svg
├── angel.svg
└── splatted.svg
```

## `pack.toml`

```toml
[pack]
id          = "my-pet"        # unique slug, must equal the directory name
name        = "My Pet"        # display name for the settings app
author      = "you"
license     = "CC-BY-SA-4.0"
version     = "1.0.0"
description = "A short blurb."

[activities]
walker   = "walker.svg"       # required — used as the fallback for omitted activities
faller   = "faller.svg"       # everything else is optional
climber  = "climber.svg"
tumbler  = "tumbler.svg"
floater  = "floater.svg"
action0  = "action0.svg"
angel    = "angel.svg"
splatted = "splatted.svg"
```

## Drawing the SVGs

- Use a **square `0 0 100 100` viewBox** and a **transparent background**.
  The renderer rasterises each pose at the display size, so use real vector
  shapes (`<path>`, `<ellipse>`, …) — not a grid of 1×1 rectangles.
- Keep art inside the viewBox; the daemon fits it into the pet window.
- Poses are currently **single-frame** (one static pose per activity).

### Activities

| activity   | when it plays                    |
|------------|----------------------------------|
| `walker`   | walking / running / moving (also the fallback) |
| `faller`   | falling                          |
| `climber`  | climbing a wall                  |
| `tumbler`  | tumbling / landing               |
| `floater`  | drifting down                    |
| `action0`  | idle / sleep / ambient action    |
| `angel`    | expired, floating away           |
| `splatted` | expired, splat                   |

Only `walker` is required; any activity you leave out falls back to `walker`.

## Installing a pack

Copy the pack directory into the user pets folder:

```
$XDG_DATA_HOME/waypenguin/pets/<id>/      # or ~/.local/share/waypenguin/pets/<id>/
```

WayPenguin discovers packs there on start (selection UI in the settings app is
planned). The built-in `tux-alpha` pack is always available as the default.

## Regenerating `tux-alpha`

The Tux poses are generated from a small part-kit:

```
python3 tux-alpha/generate_poses.py    # rewrites tux-alpha/*.svg
```
