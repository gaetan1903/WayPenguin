//! Pet-pack loading and rendering.
//!
//! A **pet pack** is a directory containing a `pack.toml` manifest plus one
//! vector SVG per activity (see `svg/README.md`). All packs found under
//! `waypenguin-assets/svg/` are embedded at compile time; additional packs
//! are discovered from the user pets directory at runtime.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use include_dir::{include_dir, Dir};
use serde::Deserialize;

/// Canonical activity keys the daemon knows how to play.
pub const ACTIVITIES: &[&str] = &[
    "action0", "walker", "climber", "faller", "tumbler", "floater", "splatted", "angel",
];

/// The activity every pack must provide; used as the fallback for omitted ones.
pub const REQUIRED_ACTIVITY: &str = "walker";

/// The id of the built-in default pack.
pub const DEFAULT_PACK: &str = "tux-alpha";

/// All packs under `svg/` embedded at compile time — the whole directory tree
/// is compiled in so the binary works with no assets on disk.
static BUILTIN_SVG: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/svg");

// ---------------------------------------------------------------------------
// Manifest
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Manifest {
    pack: PackMeta,
    #[serde(default)]
    activities: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct PackMeta {
    id: String,
    name: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    license: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    scale: Option<f32>,
}

// ---------------------------------------------------------------------------
// Pack source (embedded or on-disk) + loaded pack
// ---------------------------------------------------------------------------

/// Where a pack's files are read from.
#[derive(Debug, Clone)]
enum Source {
    /// Embedded in the binary. `root` is the parent `svg/` dir; `pack_path` is
    /// the subdirectory name (e.g. `"tux-alpha"`) used to build root-relative
    /// lookup paths (`"tux-alpha/pack.toml"`) that `include_dir` stores.
    Builtin(&'static Dir<'static>, String),
    /// A directory on disk.
    Directory(PathBuf),
}

impl Source {
    fn read(&self, file: &str) -> Option<String> {
        match self {
            Source::Builtin(root, pack_path) => root
                .get_file(format!("{pack_path}/{file}"))
                .and_then(|f| f.contents_utf8())
                .map(str::to_owned),
            Source::Directory(path) => std::fs::read_to_string(path.join(file)).ok(),
        }
    }
}

/// Metadata describing a pet pack (for the settings app to list/select).
#[derive(Debug, Clone)]
pub struct PackInfo {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub version: String,
    pub description: String,
    /// `true` for the embedded default pack, `false` for user packs on disk.
    pub builtin: bool,
    /// Scale multiplier for pet size (default: 1.0).
    pub scale: f32,
}

/// A loaded pet pack: metadata plus each activity's SVG source.
#[derive(Debug, Clone)]
pub struct PetPack {
    pub info: PackInfo,
    /// activity key -> SVG XML.
    activities: HashMap<String, String>,
}

impl PetPack {
    fn load(source: Source, builtin: bool) -> Option<PetPack> {
        let manifest_text = source.read("pack.toml")?;
        let manifest: Manifest = toml::from_str(&manifest_text).ok()?;

        let mut activities = HashMap::new();
        for (activity, file) in &manifest.activities {
            if let Some(svg) = source.read(file) {
                activities.insert(activity.clone(), svg);
            } else {
                eprintln!(
                    "pack '{}': activity '{activity}' references missing file '{file}'",
                    manifest.pack.id
                );
            }
        }

        if !activities.contains_key(REQUIRED_ACTIVITY) {
            eprintln!(
                "pack '{}': missing required '{REQUIRED_ACTIVITY}' activity — skipping",
                manifest.pack.id
            );
            return None;
        }

        Some(PetPack {
            info: PackInfo {
                id: manifest.pack.id,
                name: manifest.pack.name,
                author: manifest.pack.author,
                license: manifest.pack.license,
                version: manifest.pack.version,
                description: manifest.pack.description,
                builtin,
                scale: manifest.pack.scale.unwrap_or(1.0).clamp(0.1, 3.0),
            },
            activities,
        })
    }

    /// Activity keys this pack provides, sorted for stable output.
    pub fn activities(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.activities.keys().map(String::as_str).collect();
        keys.sort_unstable();
        keys
    }

    /// The SVG for `activity`, falling back to [`REQUIRED_ACTIVITY`] when absent.
    /// Returns the resolved activity key and its SVG source.
    fn svg_for(&self, activity: &str) -> Option<(String, &str)> {
        if let Some(svg) = self.activities.get(activity) {
            return Some((activity.to_string(), svg));
        }
        self.activities
            .get(REQUIRED_ACTIVITY)
            .map(|svg| (REQUIRED_ACTIVITY.to_string(), svg.as_str()))
    }

    /// Rasterise an activity pose at `target` px (largest dimension), returning
    /// `(frames, width, height)`. Poses are single-frame, so `frames.len() == 1`.
    pub fn render(&self, activity: &str, target: u32) -> Option<(Vec<Vec<u32>>, u32, u32)> {
        let (resolved, svg) = self.svg_for(activity)?;
        let key = (self.info.id.clone(), resolved, target);

        let mut cache = CACHE.lock().ok()?;
        if !cache.contains_key(&key) {
            let rendered = rasterize(svg, target)?;
            cache.insert(key.clone(), rendered);
        }
        let entry = cache.get(&key)?;
        Some((vec![entry.pixels.clone()], entry.width, entry.height))
    }
}

// ---------------------------------------------------------------------------
// Discovery / registry
// ---------------------------------------------------------------------------

/// User pets directory.
///
/// Resolution order:
/// 1. `WAYPENGUIN_PETS_DIR` — explicit override (useful during development to
///    point directly at `waypenguin-assets/svg/`).
/// 2. `$XDG_DATA_HOME/waypenguin/pets`
/// 3. `~/.local/share/waypenguin/pets`
///
/// Returns `None` if no usable path can be determined.
pub fn pets_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("WAYPENGUIN_PETS_DIR") {
        return Some(PathBuf::from(dir));
    }
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))?;
    Some(base.join("waypenguin").join("pets"))
}

/// The built-in default pack (`tux-alpha`), always available.
pub fn builtin_pack() -> Option<PetPack> {
    BUILTIN_SVG
        .dirs()
        .find(|d| d.path().to_str() == Some(DEFAULT_PACK))
        .and_then(|dir| {
            let pack_path = dir.path().to_str()?.to_owned();
            PetPack::load(Source::Builtin(&BUILTIN_SVG, pack_path), true)
        })
        .or_else(|| all_builtin_packs().into_iter().next())
}

/// All packs embedded at compile time (one per `svg/<id>/` subdirectory).
fn all_builtin_packs() -> Vec<PetPack> {
    BUILTIN_SVG
        .dirs()
        .filter_map(|dir| {
            let pack_path = dir.path().to_str()?.to_owned();
            PetPack::load(Source::Builtin(&BUILTIN_SVG, pack_path), true)
        })
        .collect()
}

/// Load a pack from a specific directory on disk.
pub fn load_pack_from_dir(dir: impl AsRef<Path>) -> Option<PetPack> {
    PetPack::load(Source::Directory(dir.as_ref().to_path_buf()), false)
}

/// All available packs: every pack embedded at compile time, plus any valid
/// packs found under [`pets_dir`]. Later duplicates of an id are ignored
/// (built-in wins).
pub fn discover_packs() -> Vec<PetPack> {
    let mut packs = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for pack in all_builtin_packs() {
        seen.insert(pack.info.id.clone());
        packs.push(pack);
    }

    if let Some(dir) = pets_dir() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.join("pack.toml").is_file() {
                    continue;
                }
                if let Some(pack) = load_pack_from_dir(&path) {
                    if seen.insert(pack.info.id.clone()) {
                        packs.push(pack);
                    }
                }
            }
        }
    }

    packs
}

/// Find a pack by id among the discovered packs.
pub fn load_pack(id: &str) -> Option<PetPack> {
    discover_packs().into_iter().find(|p| p.info.id == id)
}

/// The active pack, cached. Reads `WAYPENGUIN_PACK` at first use to select a
/// pack by id; falls back to the built-in default when the variable is unset
/// or the requested id is not found.
fn active_pack() -> &'static PetPack {
    static ACTIVE: LazyLock<PetPack> = LazyLock::new(|| {
        if let Ok(id) = std::env::var("WAYPENGUIN_PACK") {
            if let Some(pack) = load_pack(&id) {
                return pack;
            }
            eprintln!("waypenguin: WAYPENGUIN_PACK={id:?} not found, falling back to built-in");
        }
        builtin_pack().expect("built-in pack must load")
    });
    &ACTIVE
}

// ---------------------------------------------------------------------------
// Rasterisation
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Rendered {
    /// Straight-alpha ARGB8888 pixels.
    pixels: Vec<u32>,
    width: u32,
    height: u32,
}

/// Cache keyed by (pack id, activity, target) so each size is rendered once.
type Cache = HashMap<(String, String, u32), Rendered>;
static CACHE: LazyLock<Mutex<Cache>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Rasterise an SVG so its largest dimension equals `target` px, preserving the
/// aspect ratio. Because the source is a true vector this is crisp at any size.
fn rasterize(svg_xml: &str, target: u32) -> Option<Rendered> {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_xml.as_bytes(), &opt).ok()?;

    let size = tree.size();
    let scale = target as f32 / size.width().max(size.height());
    let width = ((size.width() * scale).round() as u32).max(1);
    let height = ((size.height() * scale).round() as u32).max(1);

    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // tiny-skia stores premultiplied RGBA; demultiply back to straight-alpha
    // ARGB so the compositor's alpha blending sees clean, halo-free edges.
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for px in pixmap.pixels() {
        let c = px.demultiply();
        let (r, g, b, a) = (
            c.red() as u32,
            c.green() as u32,
            c.blue() as u32,
            c.alpha() as u32,
        );
        pixels.push((a << 24) | (r << 16) | (g << 8) | b);
    }

    Some(Rendered {
        pixels,
        width,
        height,
    })
}

// ---------------------------------------------------------------------------
// Convenience API (renders from the active pack)
// ---------------------------------------------------------------------------

/// Returns the rendered frames for an activity from the active pack at the
/// given display size. Single-frame, so the vector holds exactly one frame of
/// `width * height` ARGB8888 pixels. `None` for unknown activities.
pub fn get_activity_frames(name: &str, target: u32) -> Option<(Vec<Vec<u32>>, u32, u32)> {
    active_pack().render(name, target)
}

/// Number of frames in an activity. Poses are single-frame.
pub fn activity_frame_count(name: &str) -> Option<usize> {
    active_pack().svg_for(name).map(|_| 1)
}

/// Nearest-neighbor rescale of a single frame (kept for callers that need to
/// resize an already-rendered frame without re-rasterising the vector).
pub fn upscale_frame(
    frame_pixels: &[u32],
    src_w: u32,
    src_h: u32,
    dest_w: u32,
    dest_h: u32,
) -> Vec<u32> {
    let mut out = vec![0u32; (dest_w * dest_h) as usize];
    for dy in 0..dest_h {
        let sy = (dy * src_h / dest_h).min(src_h - 1);
        for dx in 0..dest_w {
            let sx = (dx * src_w / dest_w).min(src_w - 1);
            let src_idx = (sy * src_w + sx) as usize;
            let dst_idx = (dy * dest_w + dx) as usize;
            out[dst_idx] = frame_pixels[src_idx];
        }
    }
    out
}

/// Fallback: generate a single solid-color frame for when no activity is available.
pub fn fallback_frame(width: u32, height: u32) -> Vec<u32> {
    let c: u32 = 0xFF_2D2D2D;
    vec![c; (width * height) as usize]
}
