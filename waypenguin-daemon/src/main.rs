use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use waypenguin_backends::DesktopBackend;
use waypenguin_core::{AnimationFrame, Pet, PetState};
use waypenguin_kde::KdeBackend;

const PET_SIZE: u32 = 90;
const AVOID_DIST: f32 = 120.0;

struct CachedSheet {
    pixels: Vec<u32>,
    sheet_w: u32,
    sheet_h: u32,
}

/// Pre-rendered activity spritesheets and frame metadata.
struct ThemeRenderer {
    activities: HashMap<String, CachedSheet>,
    frame_lists: HashMap<String, Vec<AnimationFrame>>,
}

impl ThemeRenderer {
    fn load() -> Option<Self> {
        let activity_names = [
            "action0", "walker", "climber", "faller", "tumbler", "floater", "splatted", "angel",
        ];

        let mut activities = HashMap::new();
        let mut frame_lists = HashMap::new();

        for &name in &activity_names {
            let (frames, base_w, base_h) = waypenguin_assets::get_activity_frames(name)?;

            let frame_count = frames.len();

            // Scale to fit within PET_SIZE, preserving aspect ratio
            let scale = (PET_SIZE as f32 / base_h as f32).min(PET_SIZE as f32 / base_w as f32);
            let frame_w = (base_w as f32 * scale).ceil() as u32;
            let frame_h = (base_h as f32 * scale).ceil() as u32;

            // Upscale each frame and concatenate into one spritesheet
            let sheet_w = frame_w * frame_count as u32;
            let sheet_h = frame_h;
            let mut pixels = vec![0u32; (sheet_w * sheet_h) as usize];

            for (i, frame) in frames.iter().enumerate() {
                let upscaled =
                    waypenguin_assets::upscale_frame(frame, base_w, base_h, frame_w, frame_h);
                let x_off = i as u32 * frame_w;
                for y in 0..frame_h {
                    let src_begin = (y * frame_w) as usize;
                    let dst_begin = (y * sheet_w + x_off) as usize;
                    pixels[dst_begin..dst_begin + frame_w as usize]
                        .copy_from_slice(&upscaled[src_begin..src_begin + frame_w as usize]);
                }
            }

            let frame_list: Vec<AnimationFrame> = (0..frame_count)
                .map(|i| AnimationFrame {
                    x: i as u32 * frame_w,
                    y: 0,
                    width: frame_w,
                    height: frame_h,
                })
                .collect();

            println!(
                "  {}: {} frames, {}×{} → {}×{} (scale {:.2})",
                name, frame_count, base_w, base_h, frame_w, frame_h, scale
            );

            activities.insert(
                name.to_string(),
                CachedSheet {
                    pixels,
                    sheet_w,
                    sheet_h,
                },
            );
            frame_lists.insert(name.to_string(), frame_list);
        }

        if activities.is_empty() {
            eprintln!("No theme activities loaded");
            return None;
        }

        println!("Loaded {} activities from theme", activities.len());
        Some(Self {
            activities,
            frame_lists,
        })
    }

    fn activity_sheet(&self, activity: &str) -> &CachedSheet {
        self.activities
            .get(activity)
            .or_else(|| self.activities.get("walker"))
            .or_else(|| self.activities.get("action0"))
            .expect("No fallback activity available")
    }

    fn activity_frames(&self, activity: &str) -> &[AnimationFrame] {
        self.frame_lists
            .get(activity)
            .or_else(|| self.frame_lists.get("walker"))
            .or_else(|| self.frame_lists.get("action0"))
            .expect("No fallback activity available")
    }
}

fn fps_for_state(state: PetState) -> u32 {
    match state {
        PetState::Run => 10,
        PetState::Walk => 6,
        PetState::Wake => 8,
        PetState::Celebrate => 6,
        PetState::Fall => 4,
        PetState::Land => 6,
        PetState::Peck => 8,
        PetState::Float => 4,
        PetState::Tumble => 10,
        PetState::Climb => 5,
        PetState::Stomp => 8,
        PetState::Squish => 4,
        PetState::Splat => 2,
        PetState::Angel => 4,
        PetState::Idle | PetState::LookAround | PetState::Action => 2,
        PetState::Sleep => 2,
        _ => 4,
    }
}

fn state_to_activity(state: PetState) -> &'static str {
    match state {
        PetState::Idle => "action0",
        PetState::LookAround | PetState::Wake | PetState::FollowCursor => "walker",
        PetState::Walk | PetState::Wander => "walker",
        PetState::Action => "action0",
        PetState::Run => "walker",
        PetState::Sleep => "action0",
        PetState::Fall => "faller",
        PetState::Land => "tumbler",
        PetState::Celebrate | PetState::Peck => "action0",
        PetState::Float => "floater",
        PetState::Tumble => "tumbler",
        PetState::Climb | PetState::Stomp => "climber",
        PetState::Squish | PetState::Splat => "splatted",
        PetState::Angel => "angel",
    }
}

fn parse_args() -> u32 {
    let args: Vec<String> = std::env::args().collect();
    for i in 1..args.len() {
        if args[i] == "-n" || args[i] == "--n" {
            if let Some(n) = args.get(i + 1).and_then(|s| s.parse::<u32>().ok()) {
                return n.clamp(1, 50);
            }
        }
    }
    5
}

struct PetInstance {
    pet: Pet,
    window: Box<dyn waypenguin_backends::DesktopWindow>,
    frame_buffer: Vec<u32>,
}

fn spawn_pets(
    backend: &mut KdeBackend,
    count: u32,
    screen_w: f32,
    screen_h: f32,
) -> Vec<PetInstance> {
    let spacing = screen_w / (count + 1) as f32;
    let mut instances = Vec::new();

    for i in 0..count {
        let x = spacing * (i + 1) as f32 - (PET_SIZE as f32 / 2.0);
        let window_y = -40 - (i as i32 * 20);

        let window = backend
            .create_window(PET_SIZE, PET_SIZE, x as i32, window_y)
            .expect("Failed to create pet window");

        let mut pet = Pet::new_falling(x, screen_w, screen_h);
        pet.width = PET_SIZE as f32;
        pet.vx = (i as f32 - count as f32 / 2.0) * 0.3;
        pet.idle_time_ms = i * 3000;

        instances.push(PetInstance {
            pet,
            window,
            frame_buffer: vec![0u32; (PET_SIZE * PET_SIZE) as usize],
        });
    }

    instances
}

fn main() {
    let pet_count = parse_args();
    println!("Starting WayPenguin Daemon V0.1 ({} pets)", pet_count);

    let mut backend = match KdeBackend::new() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to initialize KDE backend: {:?}", e);
            std::process::exit(1);
        }
    };

    let screens = backend.get_screens();
    let (screen_w, screen_h) = if let Some(primary) = screens.first() {
        println!(
            "Detected screen: {} ({}x{})",
            primary.name, primary.width, primary.height
        );
        (primary.width as f32, primary.height as f32)
    } else {
        println!("No screens detected, defaulting to 1920x1080");
        (1920.0, 1080.0)
    };

    let theme_renderer = ThemeRenderer::load();
    let mut pets = spawn_pets(&mut backend, pet_count, screen_w, screen_h);

    println!("Starting main event loop...");
    let mut last_tick = Instant::now();

    loop {
        if let Err(e) = backend.event_queue.dispatch_pending(&mut backend.state) {
            eprintln!("Wayland dispatch error: {:?}", e);
            break;
        }

        let now = Instant::now();
        let dt_ms = now.duration_since(last_tick).as_millis() as u32;

        if dt_ms >= 16 {
            last_tick = now;

            let (cursor_x, cursor_y) = backend.get_cursor_position();
            let cursor_known = cursor_x != 0 || cursor_y != 0;

            let positions: Vec<(f32, f32)> = pets.iter().map(|p| (p.pet.x, p.pet.y)).collect();
            let last_click = backend.get_last_click();

            if last_click.is_some() {
                backend.clear_last_click();
            }

            for instance in &mut pets {
                let prev_state = instance.pet.state;

                // Build filtered list of other pet positions on the stack
                let mut other_positions = [(0.0f32, 0.0f32); 50];
                let mut other_count = 0;
                for &pos in &positions {
                    if pos != (instance.pet.x, instance.pet.y) {
                        other_positions[other_count] = pos;
                        other_count += 1;
                    }
                }
                let others = &other_positions[..other_count];

                instance.pet.update_ai(
                    dt_ms,
                    cursor_x as f32,
                    cursor_y as f32,
                    cursor_known,
                    others,
                );
                instance.pet.apply_avoidance(others, AVOID_DIST);

                if let Some((cx, cy)) = last_click {
                    let px = instance.pet.x as i32;
                    let py = instance.pet.y as i32;
                    let ps = PET_SIZE as i32;
                    if cx >= px && cx < px + ps && cy >= py && cy < py + ps {
                        instance.pet.squish();
                    }
                }
                instance.pet.update_movement(dt_ms);

                let theme_ref = theme_renderer.as_ref();
                let (frames, src_pixels, src_w, src_h) = if let Some(svg) = theme_ref {
                    let activity = state_to_activity(instance.pet.state);
                    let frames = svg.activity_frames(activity);
                    let sheet = svg.activity_sheet(activity);
                    (frames, &sheet.pixels[..], sheet.sheet_w, sheet.sheet_h)
                } else {
                    static NO_FRAMES: &[AnimationFrame] = &[AnimationFrame {
                        x: 0,
                        y: 0,
                        width: 90,
                        height: 90,
                    }];
                    static NO_PIXELS: &[u32] = &[0u32; 8100];
                    (NO_FRAMES, NO_PIXELS, 90u32, 90u32)
                };

                // Compute frame index directly — no AnimationSpec allocation
                let fps = fps_for_state(instance.pet.state);
                instance.pet.elapsed_ms += dt_ms;
                if !frames.is_empty() {
                    let frame_duration = 1000 / fps;
                    let total_duration = frame_duration * frames.len() as u32;
                    if total_duration > 0 {
                        instance.pet.elapsed_ms %= total_duration;
                    }
                    instance.pet.frame_index = (instance.pet.elapsed_ms / frame_duration)
                        .min(frames.len() as u32 - 1)
                        as usize;
                }

                if instance.pet.state != prev_state {
                    instance.pet.frame_index = 0;
                    instance.pet.elapsed_ms = 0;
                }

                if let Err(e) = instance
                    .window
                    .set_position(instance.pet.x as i32, instance.pet.y as i32)
                {
                    eprintln!("Failed to set window position: {:?}", e);
                }

                let frame_idx = instance.pet.frame_index.min(frames.len() - 1);
                let frame = &frames[frame_idx];

                // Clear buffer
                instance.frame_buffer.fill(0);

                // Contact shadow — fades when airborne, stretches when running
                let air = 1.0
                    - ((instance.pet.floor_y - instance.pet.y).max(0.0) / 300.0).clamp(0.0, 1.0);
                let shadow_opacity = if instance.pet.grounded {
                    0.55
                } else {
                    0.55 * air
                };
                let run_stretch = if instance.pet.state == PetState::Run {
                    0.15
                } else {
                    0.0
                };
                if shadow_opacity > 0.01 {
                    waypenguin_renderer::render_contact_shadow(
                        &mut instance.frame_buffer,
                        PET_SIZE,
                        PET_SIZE,
                        0.55 + run_stretch,
                        0.06,
                        shadow_opacity,
                        -3.0,
                    );
                }

                // Breathing offset (gentle bob when standing still)
                let breath_y = if matches!(
                    instance.pet.state,
                    PetState::Idle | PetState::LookAround | PetState::Sleep | PetState::Action
                ) {
                    waypenguin_renderer::breathing_offset(instance.pet.elapsed_ms) as i32
                } else {
                    0
                };

                let off_x = ((PET_SIZE as i32 - frame.width as i32) / 2).max(0);
                let off_y = breath_y + ((PET_SIZE as i32 - frame.height as i32) / 2).max(0);

                waypenguin_renderer::composite_frame(
                    src_pixels,
                    src_w,
                    src_h,
                    frame,
                    &mut instance.frame_buffer,
                    PET_SIZE,
                    PET_SIZE,
                    instance.pet.facing_left,
                    off_x,
                    off_y,
                );

                if let Err(e) = instance.window.present_pixels(&instance.frame_buffer) {
                    eprintln!("Failed to present pixels: {:?}", e);
                }
            }
        }

        let _ = backend.connection.flush();
        thread::sleep(Duration::from_millis(16));
    }
}
