use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

type Cache = HashMap<String, RenderedActivity>;

static CACHE: LazyLock<Mutex<Cache>> = LazyLock::new(|| Mutex::new(HashMap::new()));

struct RenderedActivity {
    /// One pixel buffer per frame (ARGB8888)
    frames: Vec<Vec<u32>>,
    width: u32,
    height: u32,
}

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

fn frame_layout(name: &str) -> Option<(u32, u32, usize)> {
    Some(match name {
        "action0" => (30, 30, 12),
        "walker" => (30, 30, 8),
        "climber" => (30, 30, 8),
        "faller" => (30, 30, 8),
        "tumbler" => (30, 30, 8),
        "floater" => (30, 30, 8),
        "splatted" => (32, 32, 12),
        "angel" => (46, 30, 4),
        _ => return None,
    })
}

fn render_activity(name: &str) -> Option<RenderedActivity> {
    let svg_xml = load_svg(name)?;
    let (frame_w, frame_h, frame_count) = frame_layout(name)?;

    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_xml.as_bytes(), &opt).ok()?;

    // Render the full spritesheet
    let size = tree.size();
    let sheet_w = size.width() as u32;
    let sheet_h = size.height() as u32;
    let mut pixmap = tiny_skia::Pixmap::new(sheet_w, sheet_h)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // Convert RGBA premultiplied pixmap data to ARGB pixel slices per frame
    let raw = pixmap.data();
    let frame_pixel_count = (frame_w * frame_h) as usize;
    let sheet_w_usize = sheet_w as usize;
    let frame_w_usize = frame_w as usize;
    let frame_h_usize = frame_h as usize;
    let mut frames: Vec<Vec<u32>> = Vec::with_capacity(frame_count);

    for fi in 0..frame_count {
        let mut frame = Vec::with_capacity(frame_pixel_count);
        let x_off = fi * frame_w_usize;

        for y in 0..frame_h_usize {
            for x in 0..frame_w_usize {
                let sx = x_off + x;
                let src_idx = (y * sheet_w_usize + sx) * 4;
                if src_idx + 3 >= raw.len() {
                    frame.push(0);
                    continue;
                }
                let r = raw[src_idx] as u32;
                let g = raw[src_idx + 1] as u32;
                let b = raw[src_idx + 2] as u32;
                let a = raw[src_idx + 3] as u32;
                let argb = (a << 24) | (r << 16) | (g << 8) | b;
                frame.push(argb);
            }
        }
        frames.push(frame);
    }

    Some(RenderedActivity {
        frames,
        width: frame_w,
        height: frame_h,
    })
}

/// Returns the rendered frames for an activity.
/// Each frame is a `Vec<u32>` of ARGB8888 pixels.
/// Returns `None` if the activity is unknown.
pub fn get_activity_frames(name: &str) -> Option<(Vec<Vec<u32>>, u32, u32)> {
    let mut cache = CACHE.lock().ok()?;
    let entry = cache
        .entry(name.to_string())
        .or_insert_with(|| render_activity(name).expect("failed to render activity"));
    Some((entry.frames.clone(), entry.width, entry.height))
}

/// Number of frames in an activity.
pub fn activity_frame_count(name: &str) -> Option<usize> {
    let (_, _, count) = frame_layout(name)?;
    Some(count)
}

/// Dimensions of a single frame.
pub fn activity_dimensions(name: &str) -> Option<(u32, u32)> {
    let (w, h, _) = frame_layout(name)?;
    Some((w, h))
}

/// Nearest-neighbor upscale a single frame from base resolution to target size.
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
