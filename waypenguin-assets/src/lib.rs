use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// Activities shipped as hand-drawn vector poses. Each is a single-pose SVG on
/// a square `0 0 100 100` viewBox, rasterised on demand at the display size.
pub const ACTIVITIES: &[&str] = &[
    "action0", "walker", "climber", "faller", "tumbler", "floater", "splatted", "angel",
];

#[derive(Clone)]
struct Rendered {
    /// Straight-alpha ARGB8888 pixels.
    pixels: Vec<u32>,
    width: u32,
    height: u32,
}

/// Cache keyed by (activity, target size) so each display size is rendered once.
type Cache = HashMap<(String, u32), Rendered>;
static CACHE: LazyLock<Mutex<Cache>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn load_svg(name: &str) -> Option<&'static str> {
    Some(match name {
        "action0" => include_str!("../svg/action0.svg"),
        "walker" => include_str!("../svg/walker.svg"),
        "climber" => include_str!("../svg/climber.svg"),
        "faller" => include_str!("../svg/faller.svg"),
        "tumbler" => include_str!("../svg/tumbler.svg"),
        "floater" => include_str!("../svg/floater.svg"),
        "splatted" => include_str!("../svg/splatted.svg"),
        "angel" => include_str!("../svg/angel.svg"),
        _ => return None,
    })
}

/// Rasterise a pose's vector SVG so its largest dimension equals `target` px,
/// preserving the aspect ratio. Because the source is a true vector, this is
/// crisp at any requested size.
fn render(name: &str, target: u32) -> Option<Rendered> {
    let svg_xml = load_svg(name)?;
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

/// Returns the rendered frames for an activity at the given display size.
///
/// Poses are single-frame, so the returned vector always holds exactly one
/// frame of `width * height` ARGB8888 pixels. Returns `None` for unknown
/// activities.
pub fn get_activity_frames(name: &str, target: u32) -> Option<(Vec<Vec<u32>>, u32, u32)> {
    let key = (name.to_string(), target);
    let mut cache = CACHE.lock().ok()?;
    if !cache.contains_key(&key) {
        let rendered = render(name, target)?;
        cache.insert(key.clone(), rendered);
    }
    let entry = cache.get(&key)?;
    Some((vec![entry.pixels.clone()], entry.width, entry.height))
}

/// Number of frames in an activity. Poses are single-frame.
pub fn activity_frame_count(name: &str) -> Option<usize> {
    load_svg(name).map(|_| 1)
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
