use waypenguin_core::AnimationFrame;

fn alpha_blend(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0 {
        return dst;
    }
    if sa >= 0xFE {
        return src;
    }
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let da = (dst >> 24) & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let a = sa + da * (255 - sa) / 255;
    let inv_sa = 255 - sa;
    let r = (sr * sa + dr * da * inv_sa / 255) / a.max(1);
    let g = (sg * sa + dg * da * inv_sa / 255) / a.max(1);
    let b = (sb * sa + db * da * inv_sa / 255) / a.max(1);
    (a.min(255) << 24) | (r.min(255) << 16) | (g.min(255) << 8) | b.min(255)
}

fn argb_f32(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let ra = (a.clamp(0.0, 1.0) * 255.0) as u32;
    let rr = (r.clamp(0.0, 1.0) * 255.0) as u32;
    let rg = (g.clamp(0.0, 1.0) * 255.0) as u32;
    let rb = (b.clamp(0.0, 1.0) * 255.0) as u32;
    (ra << 24) | (rr << 16) | (rg << 8) | rb
}

/// Renders a soft contact shadow below the pet.
///
/// `shadow_width` / `shadow_height` control the ellipse dimensions (as fraction of buffer).
/// `opacity` 0.0 = invisible, 1.0 = fully opaque.
/// `offset_y` shifts the shadow up/down within the buffer (negative = higher).
pub fn render_contact_shadow(
    target: &mut [u32],
    target_w: u32,
    target_h: u32,
    shadow_width: f32,
    shadow_height: f32,
    opacity: f32,
    offset_y: f32,
) {
    let cx = target_w as f32 / 2.0;
    let cy = target_h as f32 + offset_y - 1.0;
    let rx = (target_w as f32 * shadow_width) / 2.0;
    let ry = target_h as f32 * shadow_height;

    if rx <= 0.0 || ry <= 0.0 || opacity <= 0.0 {
        return;
    }

    let y0 = (cy - ry).max(0.0) as u32;
    let y1 = (cy + ry).min(target_h as f32 - 1.0) as u32;

    for y in y0..=y1 {
        let dy = (y as f32 - cy) / ry;
        let row_radius = rx * (1.0 - dy * dy).sqrt();
        if row_radius <= 0.0 {
            continue;
        }
        let x0 = (cx - row_radius).max(0.0) as u32;
        let x1 = (cx + row_radius).min(target_w as f32 - 1.0) as u32;

        for x in x0..=x1 {
            let dx = (x as f32 - cx) / rx;
            let dist = (dx * dx + dy * dy).sqrt();
            let alpha = (1.0 - dist).max(0.0) * opacity;
            let shadow_color = argb_f32(0.0, 0.0, 0.0, alpha * 0.35);
            let idx = (y * target_w + x) as usize;
            if idx < target.len() {
                target[idx] = alpha_blend(shadow_color, target[idx]);
            }
        }
    }
}

/// Renders a soft ambient shadow around the character for depth.
pub fn render_ambient_shadow(
    target: &mut [u32],
    target_w: u32,
    target_h: u32,
    offset_y: f32,
    extent: f32,
    opacity: f32,
) {
    let cx = target_w as f32 / 2.0;
    let cy = target_h as f32 / 2.0 + offset_y;
    let rx = target_w as f32 * extent / 2.0;
    let ry = target_h as f32 * extent / 2.0;

    if rx <= 0.0 || ry <= 0.0 || opacity <= 0.0 {
        return;
    }

    let y0 = (cy - ry).max(0.0) as u32;
    let y1 = (cy + ry).min(target_h as f32 - 1.0) as u32;

    for y in y0..=y1 {
        let dy = (y as f32 - cy) / ry;
        let row_radius = rx * (1.0 - dy * dy).sqrt();
        if row_radius <= 0.0 {
            continue;
        }
        let x0 = (cx - row_radius).max(0.0) as u32;
        let x1 = (cx + row_radius).min(target_w as f32 - 1.0) as u32;

        for x in x0..=x1 {
            let dx = (x as f32 - cx) / rx;
            let dist = (dx * dx + dy * dy).sqrt();
            let alpha = (1.0 - dist * dist).max(0.0) * opacity;
            let shadow_color = argb_f32(0.0, 0.0, 0.0, alpha * 0.15);
            let idx = (y * target_w + x) as usize;
            if idx < target.len() {
                target[idx] = alpha_blend(shadow_color, target[idx]);
            }
        }
    }
}

/// Composite a frame onto an existing buffer with optional offset.
#[allow(clippy::too_many_arguments)]
pub fn composite_frame(
    source_pixels: &[u32],
    source_w: u32,
    _source_h: u32,
    frame: &AnimationFrame,
    target_pixels: &mut [u32],
    target_w: u32,
    target_h: u32,
    flip_horizontal: bool,
    offset_x: i32,
    offset_y: i32,
) {
    let draw_w = frame.width.min(target_w);
    let draw_h = frame.height.min(target_h);

    for y in 0..draw_h {
        let src_y = frame.y + y;
        let dst_y = (y as i32 + offset_y) as u32;
        if dst_y >= target_h {
            continue;
        }

        for x in 0..draw_w {
            let src_x = if flip_horizontal {
                frame.x + (frame.width.saturating_sub(1) - x)
            } else {
                frame.x + x
            };
            let dst_x = (x as i32 + offset_x) as u32;
            if dst_x >= target_w {
                continue;
            }

            let src_idx = (src_y * source_w + src_x) as usize;
            if src_idx < source_pixels.len() {
                let pixel = source_pixels[src_idx];
                let dst_idx = (dst_y * target_w + dst_x) as usize;
                if dst_idx < target_pixels.len() {
                    target_pixels[dst_idx] = alpha_blend(pixel, target_pixels[dst_idx]);
                }
            }
        }
    }
}

/// Render a frame (clears buffer first, no offset).
#[allow(clippy::too_many_arguments)]
pub fn render_frame(
    source_pixels: &[u32],
    source_w: u32,
    source_h: u32,
    frame: &AnimationFrame,
    target_pixels: &mut [u32],
    target_w: u32,
    target_h: u32,
    flip_horizontal: bool,
) {
    for pixel in target_pixels.iter_mut() {
        *pixel = 0;
    }
    composite_frame(
        source_pixels,
        source_w,
        source_h,
        frame,
        target_pixels,
        target_w,
        target_h,
        flip_horizontal,
        0,
        0,
    );
}

/// Compute breathing Y offset based on elapsed time.
pub fn breathing_offset(elapsed_ms: u32) -> f32 {
    let phase = (elapsed_ms as f32 / 2200.0) * std::f32::consts::TAU;
    phase.sin() * 1.0
}

/// Compute squash/stretch scale factors from speed and vertical velocity.
pub fn squash_stretch(speed: f32, vy: f32) -> (f32, f32) {
    let stretch = 1.0 + (speed * 0.012).min(0.12);
    let squash = 1.0 - (speed * 0.006).min(0.06);
    let air = if vy.abs() > 0.5 {
        1.0 + vy.abs() * 0.015
    } else {
        1.0
    };
    (squash / air, stretch * air)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_frame_simple() {
        let source = vec![0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFFFF];
        let mut target = vec![0; 4];
        let frame = AnimationFrame {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };

        render_frame(&source, 2, 2, &frame, &mut target, 2, 2, false);
        assert_eq!(target, vec![0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFFFF]);

        let mut target2 = vec![0; 4];
        render_frame(&source, 2, 2, &frame, &mut target2, 2, 2, true);
        assert_eq!(
            target2,
            vec![0xFF00FF00, 0xFFFF0000, 0xFFFFFFFF, 0xFF0000FF]
        );
    }

    #[test]
    fn test_shadow_does_not_panic() {
        let mut buf = vec![0u32; 90 * 90];
        render_contact_shadow(&mut buf, 90, 90, 0.7, 0.08, 0.6, 0.0);
        assert!(buf.iter().any(|&p| p != 0));
    }

    #[test]
    fn test_breathing_offset() {
        let off = breathing_offset(0);
        assert!((off - 0.0).abs() < 0.001);
        let off2 = breathing_offset(550);
        assert!(off2.abs() > 0.0);
    }
}
