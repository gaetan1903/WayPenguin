use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PetState {
    Idle,
    Walk,
    Run,
    Sleep,
    Wake,
    LookAround,
    FollowCursor,
    Fall,
    Land,
    Celebrate,
    Wander,
    Peck,
    Float,
    Tumble,
    Climb,
    Stomp,
    Squish,
    Splat,
    Angel,
    Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnimationFrame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpriteSheet {
    pub image_path: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frames: Vec<AnimationFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnimationSpec {
    pub state: PetState,
    pub sprite_sheet: SpriteSheet,
    pub fps: u32,
    pub loop_anim: bool,
}

#[derive(Debug, Clone)]
pub struct PetArchetype {
    pub name: &'static str,
    pub idle_weight: f32,
    pub wander_weight: f32,
    pub float_weight: f32,
    pub tumble_weight: f32,
    pub climb_weight: f32,
    pub stomp_weight: f32,
    pub speed_multiplier: f32,
    pub sleep_timeout_ms: u32,
    pub attraction_radius: f32,
    pub bounce_energy: f32,
}

impl PetArchetype {
    pub fn ground_dweller() -> Self {
        Self {
            name: "ground-dweller",
            idle_weight: 1.0,
            wander_weight: 4.0,
            float_weight: 2.0,
            tumble_weight: 2.0,
            climb_weight: 0.0,
            stomp_weight: 0.0,
            speed_multiplier: 1.0,
            sleep_timeout_ms: 10_000,
            attraction_radius: 400.0,
            bounce_energy: 0.5,
        }
    }

    pub fn floater() -> Self {
        Self {
            name: "floater",
            idle_weight: 0.5,
            wander_weight: 1.0,
            float_weight: 6.0,
            tumble_weight: 0.5,
            climb_weight: 0.0,
            stomp_weight: 0.0,
            speed_multiplier: 0.8,
            sleep_timeout_ms: 15_000,
            attraction_radius: 300.0,
            bounce_energy: 0.3,
        }
    }

    pub fn climber() -> Self {
        Self {
            name: "climber",
            idle_weight: 0.5,
            wander_weight: 2.0,
            float_weight: 0.5,
            tumble_weight: 0.5,
            climb_weight: 5.0,
            stomp_weight: 3.0,
            speed_multiplier: 1.2,
            sleep_timeout_ms: 12_000,
            attraction_radius: 350.0,
            bounce_energy: 0.8,
        }
    }

    pub fn tumbler() -> Self {
        Self {
            name: "tumbler",
            idle_weight: 0.5,
            wander_weight: 1.0,
            float_weight: 1.0,
            tumble_weight: 5.0,
            climb_weight: 0.5,
            stomp_weight: 0.5,
            speed_multiplier: 1.5,
            sleep_timeout_ms: 8_000,
            attraction_radius: 500.0,
            bounce_energy: 1.0,
        }
    }

    pub fn explorer() -> Self {
        Self {
            name: "explorer",
            idle_weight: 0.5,
            wander_weight: 4.0,
            float_weight: 2.0,
            tumble_weight: 1.0,
            climb_weight: 2.0,
            stomp_weight: 1.0,
            speed_multiplier: 1.0,
            sleep_timeout_ms: 20_000,
            attraction_radius: 250.0,
            bounce_energy: 0.7,
        }
    }

    pub fn balanced() -> Self {
        Self {
            name: "balanced",
            idle_weight: 1.0,
            wander_weight: 2.0,
            float_weight: 3.0,
            tumble_weight: 2.0,
            climb_weight: 2.0,
            stomp_weight: 1.0,
            speed_multiplier: 1.0,
            sleep_timeout_ms: 10_000,
            attraction_radius: 400.0,
            bounce_energy: 0.6,
        }
    }

    pub fn random() -> Self {
        match fastrand::u32(0..6) {
            0 => Self::ground_dweller(),
            1 => Self::floater(),
            2 => Self::climber(),
            3 => Self::tumbler(),
            4 => Self::explorer(),
            _ => Self::balanced(),
        }
    }

    pub fn pick_idle_behavior(&self, _at_edge: bool) -> PetState {
        let weights: Vec<(PetState, f32)> = vec![
            (PetState::Idle, self.idle_weight),
            (PetState::Wander, self.wander_weight),
            (PetState::Float, self.float_weight),
            (PetState::Tumble, self.tumble_weight),
            (PetState::Climb, self.climb_weight),
            (PetState::Stomp, self.stomp_weight),
        ];
        let total: f32 = weights.iter().map(|(_, w)| w).sum();
        if total <= 0.0 {
            return PetState::Idle;
        }
        let r = fastrand::f32() * total;
        let mut acc = 0.0;
        for (state, weight) in &weights {
            acc += weight;
            if r < acc {
                return *state;
            }
        }
        PetState::Idle
    }
}

pub struct Pet {
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub state: PetState,
    pub frame_index: usize,
    pub elapsed_ms: u32,
    pub facing_left: bool,
    pub speed: f32,
    pub idle_time_ms: u32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_known: bool,
    pub screen_w: f32,
    pub screen_h: f32,
    pub floor_y: f32,
    pub vy: f32,
    pub vx: f32,
    pub grounded: bool,
    pub land_time_ms: u32,
    pub cursor_attraction_radius: f32,
    pub wander_timer_ms: u32,
    pub look_around_cooldown_ms: u32,
    pub look_around_timer_ms: u32,
    pub edge_linger_ms: u32,
    pub peck_time_ms: u32,
    pub pet_reaction_cooldown_ms: u32,
    pub celebrate_time_ms: u32,
    pub archetype: PetArchetype,
    pub float_phase: f32,
    pub float_base_y: f32,
    pub tumble_direction: f32,
    pub climb_waypoints: Vec<(f32, f32)>,
    pub climb_waypoint_index: usize,
    pub squish_time_ms: u32,
    pub float_timer_ms: u32,
    pub action_time_ms: u32,
    pub width: f32,
}

impl Pet {
    pub fn new(x: f32, y: f32) -> Self {
        let archetype = PetArchetype::random();
        Self {
            x,
            y,
            target_x: x,
            target_y: y,
            state: PetState::Idle,
            frame_index: 0,
            elapsed_ms: 0,
            facing_left: false,
            speed: 2.0 * archetype.speed_multiplier,
            idle_time_ms: 0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            cursor_known: false,
            screen_w: 1920.0,
            screen_h: 1080.0,
            floor_y: 1050.0,
            vy: 0.0,
            vx: 0.0,
            grounded: true,
            land_time_ms: 0,
            cursor_attraction_radius: archetype.attraction_radius,
            wander_timer_ms: 0,
            look_around_cooldown_ms: 5000,
            look_around_timer_ms: 0,
            edge_linger_ms: 0,
            peck_time_ms: 0,
            pet_reaction_cooldown_ms: 0,
            celebrate_time_ms: 0,
            archetype,
            float_phase: fastrand::f32() * std::f32::consts::TAU,
            float_base_y: 0.0,
            tumble_direction: 1.0,
            climb_waypoints: Vec::new(),
            climb_waypoint_index: 0,
            squish_time_ms: 0,
            float_timer_ms: 0,
            action_time_ms: 0,
            width: 64.0,
        }
    }

    pub fn new_falling(x: f32, screen_w: f32, screen_h: f32) -> Self {
        let archetype = PetArchetype::random();
        Self {
            x,
            y: -40.0,
            target_x: x,
            target_y: screen_h - 130.0,
            state: PetState::Fall,
            frame_index: 0,
            elapsed_ms: 0,
            facing_left: x < screen_w / 2.0,
            speed: 0.0,
            idle_time_ms: 0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            cursor_known: false,
            screen_w,
            screen_h,
            floor_y: screen_h - 130.0,
            vy: 1.0,
            vx: 0.0,
            grounded: false,
            land_time_ms: 0,
            cursor_attraction_radius: archetype.attraction_radius,
            wander_timer_ms: 0,
            look_around_cooldown_ms: 5000,
            look_around_timer_ms: 0,
            edge_linger_ms: 0,
            peck_time_ms: 0,
            pet_reaction_cooldown_ms: 0,
            celebrate_time_ms: 0,
            archetype,
            float_phase: fastrand::f32() * std::f32::consts::TAU,
            float_base_y: 0.0,
            tumble_direction: 1.0,
            climb_waypoints: Vec::new(),
            climb_waypoint_index: 0,
            squish_time_ms: 0,
            float_timer_ms: 0,
            action_time_ms: 0,
            width: 64.0,
        }
    }

    pub fn squish(&mut self) {
        if self.state == PetState::Splat || self.state == PetState::Angel {
            return;
        }
        self.state = PetState::Squish;
        self.squish_time_ms = 0;
        self.speed = 0.0;
    }

    pub fn update_animation(&mut self, dt_ms: u32, spec: &AnimationSpec) {
        self.elapsed_ms += dt_ms;
        #[allow(clippy::manual_checked_ops)]
        let frame_duration_ms = if spec.fps > 0 { 1000 / spec.fps } else { 1000 };
        if self.elapsed_ms >= frame_duration_ms {
            let frames_to_advance = self.elapsed_ms / frame_duration_ms;
            self.elapsed_ms %= frame_duration_ms;

            let num_frames = spec.sprite_sheet.frames.len();
            if num_frames > 0 {
                if spec.loop_anim {
                    self.frame_index = (self.frame_index + frames_to_advance as usize) % num_frames;
                } else {
                    self.frame_index =
                        (self.frame_index + frames_to_advance as usize).min(num_frames - 1);
                }
            }
        }
    }

    pub fn update_movement(&mut self, dt_ms: u32) {
        let dt = dt_ms as f32 / 16.67;

        if !self.grounded {
            let gravity = 0.4;
            self.vy += gravity * dt;
            self.x += self.vx * dt;
            self.y += self.vy * dt;

            if self.x < 0.0 {
                self.x = 0.0;
                self.vx = -self.vx * 0.5;
            }
            if self.x > self.screen_w - self.width {
                self.x = self.screen_w - self.width;
                self.vx = -self.vx * 0.5;
            }

            if self.y >= self.floor_y {
                self.y = self.floor_y;
                if self.vy > 4.0 {
                    self.vy = -self.vy * 0.3;
                    if self.vy.abs() < 1.0 {
                        self.vy = 0.0;
                        self.grounded = true;
                        self.state = PetState::Land;
                        self.land_time_ms = 0;
                    }
                } else {
                    self.vy = 0.0;
                    self.grounded = true;
                    self.state = PetState::Land;
                    self.land_time_ms = 0;
                }
            }
            return;
        }

        match self.state {
            PetState::Land => {
                self.land_time_ms += dt_ms;
                if self.land_time_ms > 300 {
                    self.state = PetState::Idle;
                    self.speed = 2.0 * self.archetype.speed_multiplier;
                }
                return;
            }
            PetState::Peck => {
                self.peck_time_ms += dt_ms;
                if self.peck_time_ms > 400 {
                    self.state = PetState::Idle;
                    self.peck_time_ms = 0;
                }
                return;
            }
            PetState::Celebrate => {
                self.celebrate_time_ms += dt_ms;
                if self.celebrate_time_ms > 600 {
                    self.state = PetState::Idle;
                    self.celebrate_time_ms = 0;
                }
                return;
            }
            PetState::LookAround => {
                self.look_around_timer_ms += dt_ms;
                if self.look_around_timer_ms > 800 {
                    self.state = PetState::Idle;
                    self.look_around_timer_ms = 0;
                    self.look_around_cooldown_ms = 5000 + fastrand::u32(0..4000);
                }
                return;
            }
            PetState::Squish => {
                self.squish_time_ms += dt_ms;
                if self.squish_time_ms > 300 {
                    self.state = if fastrand::f32() < 0.3 {
                        PetState::Angel
                    } else {
                        PetState::Splat
                    };
                    self.squish_time_ms = 0;
                }
                return;
            }
            PetState::Splat => {
                self.squish_time_ms += dt_ms;
                if self.squish_time_ms > 2000 {
                    self.respawn();
                }
                return;
            }
            PetState::Angel => {
                self.y -= 1.0 * dt;
                self.squish_time_ms += dt_ms;
                if self.squish_time_ms > 2000 || self.y < -100.0 {
                    self.respawn();
                }
                return;
            }
            PetState::Float => {
                self.float_phase += dt * 0.003;
                let bob = self.float_phase.sin() * 15.0;
                let target_y = self.float_base_y + bob;
                let dy = target_y - self.y;
                self.y += dy * 0.1;

                let dx = self.target_x - self.x;
                let dist = dx.abs();
                if dist > 10.0 {
                    let step = self.speed * dt * 0.5;
                    let dir = dx.signum();
                    self.x += dir * step.min(dist.abs());
                    if self.x > self.screen_w - self.width {
                        self.x = self.screen_w - self.width;
                        self.target_x = self.x - 100.0 - fastrand::f32() * 200.0;
                    }
                    if self.x < 0.0 {
                        self.x = 0.0;
                        self.target_x = self.x + 100.0 + fastrand::f32() * 200.0;
                    }
                    self.facing_left = dx < 0.0;
                }
                self.float_timer_ms = self.float_timer_ms.saturating_sub(dt_ms);
                if self.float_timer_ms == 0 {
                    self.grounded = false;
                    self.vy = 0.0;
                    self.state = PetState::Fall;
                }
                return;
            }
            PetState::Tumble => {
                let dx = self.target_x - self.x;
                let dist = dx.abs();
                if dist > 5.0 {
                    let step = self.speed * dt * 1.5;
                    let dir = dx.signum();
                    self.x += dir * step.min(dist.abs());
                    self.y = self.floor_y;
                    self.facing_left = dx < 0.0;

                    if self.x <= 5.0 || self.x >= self.screen_w - self.width - 5.0 {
                        self.target_x = if self.x < self.screen_w / 2.0 {
                            self.screen_w - self.width - 20.0
                        } else {
                            20.0
                        };
                        self.facing_left = self.target_x < self.x;
                    }
                } else {
                    self.state = PetState::Idle;
                    self.speed = 0.0;
                }
                return;
            }
            PetState::Climb => {
                self.update_climb(dt);
                return;
            }
            PetState::Stomp => {
                let dx = self.target_x - self.x;
                let dist = dx.abs();
                if dist > 5.0 {
                    let step = self.speed * dt;
                    let dir = dx.signum();
                    self.x += dir * step.min(dist.abs());
                    self.y = 10.0;
                    self.facing_left = dx < 0.0;
                } else {
                    self.state = PetState::Idle;
                    self.speed = 0.0;
                    self.y = self.floor_y;
                }
                return;
            }
            PetState::Action => {
                self.action_time_ms = self.action_time_ms.saturating_sub(dt_ms);
                if self.action_time_ms == 0 {
                    self.state = PetState::Idle;
                }
                return;
            }
            _ => {}
        }

        let dx = self.target_x - self.x;
        let dy = self.target_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.5 {
            let step = self.speed * dt;
            let ease_dist = 30.0;
            let factor = if dist < ease_dist {
                dist / ease_dist
            } else {
                1.0
            };
            let smooth_step = step * factor.max(0.03);

            if smooth_step >= dist {
                self.x = self.target_x;
                self.y = self.target_y;
                self.arrive_at_target();
            } else {
                self.x += (dx / dist) * smooth_step;
                self.y += (dy / dist) * smooth_step;
            }

            if dx < 0.0 {
                self.facing_left = true;
            } else if dx > 0.0 {
                self.facing_left = false;
            }

            if self.state == PetState::Sleep {
                self.state = PetState::Walk;
            }
        } else {
            // Already at the target: don't linger in a locomotion pose, or the
            // pet gets stuck standing in the walk sprite forever.
            self.arrive_at_target();
        }
    }

    /// Return a ground pet to `Idle` once it reaches its movement target, so no
    /// locomotion state (Walk/Run/Wander/…) is left frozen in place.
    fn arrive_at_target(&mut self) {
        match self.state {
            PetState::Wander => {
                self.state = PetState::Idle;
                self.wander_timer_ms = fastrand::u32(500..2000);
            }
            PetState::Walk | PetState::Run | PetState::Wake | PetState::FollowCursor => {
                self.state = PetState::Idle;
                self.speed = 0.0;
            }
            _ => {}
        }
    }

    fn respawn(&mut self) {
        self.x = fastrand::f32() * (self.screen_w - self.width);
        self.y = -40.0;
        self.vy = 0.5 + fastrand::f32() * 1.0;
        self.vx = 0.0;
        self.grounded = false;
        self.state = PetState::Fall;
        self.squish_time_ms = 0;
        self.speed = 0.0;
    }

    fn update_climb(&mut self, dt: f32) {
        if self.climb_waypoints.is_empty() {
            self.init_climb_waypoints();
        }

        if self.climb_waypoint_index >= self.climb_waypoints.len() {
            self.state = PetState::Idle;
            self.speed = 0.0;
            self.y = self.floor_y;
            return;
        }

        let (tx, ty) = self.climb_waypoints[self.climb_waypoint_index];
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 8.0 {
            self.x = tx;
            self.y = ty;
            self.climb_waypoint_index += 1;
            return;
        }

        let step = self.speed * dt;
        self.x += (dx / dist) * step;
        self.y += (dy / dist) * step;
        self.facing_left = dx < 0.0;
    }

    fn init_climb_waypoints(&mut self) {
        let m = 20.0;
        let floor = self.floor_y;
        let right = self.screen_w - self.width;

        if fastrand::f32() < 0.4 {
            // Stomp variant: climb up one side, stomp across top, drop down
            let start_on_left = self.x < self.screen_w / 2.0;
            if start_on_left {
                self.climb_waypoints = vec![(m, floor), (m, 10.0), (right, 10.0), (right, floor)];
            } else {
                self.climb_waypoints = vec![(right, floor), (right, 10.0), (m, 10.0), (m, floor)];
            }
        } else {
            let start_on_left = self.x < self.screen_w / 2.0;
            if start_on_left {
                self.climb_waypoints = vec![(m, floor), (m, m), (right, m), (right, floor)];
            } else {
                self.climb_waypoints = vec![(right, floor), (right, m), (m, m), (m, floor)];
            }
        }
        self.climb_waypoint_index = 0;
        self.speed = 2.0 * self.archetype.speed_multiplier;
    }

    pub fn update_ai(
        &mut self,
        dt_ms: u32,
        cursor_x: f32,
        cursor_y: f32,
        cursor_known: bool,
        others: &[(f32, f32)],
    ) {
        if !self.grounded
            || self.state == PetState::Land
            || self.state == PetState::Peck
            || self.state == PetState::Celebrate
            || self.state == PetState::Climb
            || self.state == PetState::Tumble
            || self.state == PetState::Squish
            || self.state == PetState::Splat
            || self.state == PetState::Angel
        {
            return;
        }

        self.check_pet_reactions(others, dt_ms);
        if self.state == PetState::Celebrate {
            return;
        }

        let dx_cursor = cursor_x - self.cursor_x;
        let dy_cursor = cursor_y - self.cursor_y;
        let cursor_moved =
            cursor_known && (dx_cursor * dx_cursor + dy_cursor * dy_cursor).sqrt() > 1.0;

        if cursor_moved {
            self.idle_time_ms = 0;
            self.wander_timer_ms = 1000 + fastrand::u32(0..2000);
            self.cursor_x = cursor_x;
            self.cursor_y = cursor_y;

            let dist_to_cursor = {
                let dx = self.cursor_x - self.x;
                let dy = self.cursor_y - self.y;
                (dx * dx + dy * dy).sqrt()
            };

            let cursor_speed =
                (dx_cursor * dx_cursor + dy_cursor * dy_cursor).sqrt() / (dt_ms as f32).max(1.0);

            if cursor_speed > 0.3 || self.state == PetState::Sleep {
                self.speed = 6.0 * self.archetype.speed_multiplier;
                self.state = PetState::Run;
                self.target_x = self.cursor_x;
                self.target_y = self.floor_y;
                self.float_base_y = 0.0;
            } else if dist_to_cursor > 100.0 {
                self.speed = 2.0 * self.archetype.speed_multiplier;
                self.state = PetState::Walk;
                self.target_x = self.cursor_x;
                self.target_y = self.floor_y;
                self.float_base_y = 0.0;
            } else {
                self.speed = 0.0;
                self.state = PetState::Idle;
                self.target_x = self.x;
                self.target_y = self.y;
                self.float_base_y = 0.0;
            }
        } else {
            if cursor_known {
                self.cursor_x = cursor_x;
                self.cursor_y = cursor_y;
            }

            if self.state == PetState::Idle || self.state == PetState::Wander {
                if self.state == PetState::Wander {
                    self.wander_timer_ms = self.wander_timer_ms.saturating_sub(dt_ms);
                    if self.wander_timer_ms == 0 {
                        self.state = PetState::Idle;
                    }
                } else if self.wander_timer_ms == 0 {
                    let behavior = self.archetype.pick_idle_behavior(self.at_edge());
                    match behavior {
                        PetState::Float => {
                            let margin = 40.0;
                            self.float_base_y =
                                margin + fastrand::f32() * (self.floor_y * 0.5 - margin);
                            self.y = self.float_base_y;
                            self.target_x = margin
                                + fastrand::f32() * (self.screen_w - 2.0 * margin - self.width);
                            self.speed = 1.0 * self.archetype.speed_multiplier;
                            self.state = PetState::Float;
                            self.float_phase = fastrand::f32() * std::f32::consts::TAU;
                            self.float_timer_ms = 3000 + fastrand::u32(0..5000);
                        }
                        PetState::Tumble => {
                            let dir = if fastrand::f32() < 0.5 { -1.0 } else { 1.0 };
                            let dist = 150.0 + fastrand::f32() * 300.0;
                            self.target_x =
                                (self.x + dir * dist).clamp(5.0, self.screen_w - self.width - 5.0);
                            self.y = self.floor_y;
                            self.speed = 3.0 * self.archetype.speed_multiplier;
                            self.state = PetState::Tumble;
                            self.tumble_direction = dir;
                        }
                        PetState::Climb => {
                            self.climb_waypoints.clear();
                            self.climb_waypoint_index = 0;
                            self.init_climb_waypoints();
                            self.state = PetState::Climb;
                        }
                        PetState::Stomp => {
                            let target = if self.x < self.screen_w / 2.0 {
                                self.screen_w - self.width
                            } else {
                                0.0
                            };
                            self.target_x = target;
                            self.target_y = 10.0;
                            self.speed = 3.0 * self.archetype.speed_multiplier;
                            self.state = PetState::Stomp;
                        }
                        PetState::Wander => {
                            let margin = 40.0;
                            let rx = self.screen_w - 2.0 * margin - self.width;
                            // Pick position farthest from other pets
                            let mut best_tx = margin + fastrand::f32() * rx;
                            let mut best_dist = 0.0f32;
                            for _ in 0..6 {
                                let tx = margin + fastrand::f32() * rx;
                                let min_dist = others
                                    .iter()
                                    .map(|&(ox, oy)| {
                                        let dx = tx - ox;
                                        let dy = self.floor_y - oy;
                                        dx * dx + dy * dy
                                    })
                                    .fold(f32::MAX, f32::min);
                                if min_dist > best_dist {
                                    best_dist = min_dist;
                                    best_tx = tx;
                                }
                            }
                            self.target_x = best_tx;
                            self.target_y = self.floor_y;
                            self.speed =
                                (1.5 + fastrand::f32() * 1.5) * self.archetype.speed_multiplier;
                            self.state = PetState::Wander;
                            self.wander_timer_ms = 2500 + fastrand::u32(0..3500);
                        }
                        _ => {
                            self.wander_timer_ms = 500 + fastrand::u32(0..2000);
                        }
                    }
                } else {
                    self.wander_timer_ms = self.wander_timer_ms.saturating_sub(dt_ms);
                }
            }

            if self.state == PetState::Idle && self.at_edge() {
                self.edge_linger_ms += dt_ms;
                if self.edge_linger_ms > 1000 {
                    if fastrand::f32() < 0.3 {
                        self.state = PetState::Peck;
                        self.peck_time_ms = 0;
                        self.edge_linger_ms = 0;
                    } else if self.archetype.climb_weight + self.archetype.stomp_weight > 2.0 {
                        self.climb_waypoints.clear();
                        self.climb_waypoint_index = 0;
                        self.init_climb_waypoints();
                        self.state = PetState::Climb;
                        self.edge_linger_ms = 0;
                    } else {
                        self.edge_linger_ms = 0;
                        // Quick bounce off edge
                        let dir = if self.x < self.screen_w / 2.0 {
                            1.0
                        } else {
                            -1.0
                        };
                        self.target_x = self.x + dir * 200.0;
                        self.target_x =
                            self.target_x.clamp(20.0, self.screen_w - self.width - 20.0);
                        self.target_y = self.floor_y;
                        self.speed = 3.0 * self.archetype.speed_multiplier;
                        self.state = PetState::Wander;
                        self.wander_timer_ms = 1500 + fastrand::u32(0..1500);
                    }
                }
            } else if self.state != PetState::Idle {
                self.edge_linger_ms = 0;
            }
        }

        if self.state == PetState::Idle || self.state == PetState::LookAround {
            let mut near_dx = 0.0f32;
            let mut near_dy = 0.0f32;
            let mut near_count = 0u32;
            for &(ox, oy) in others {
                let dx = self.x - ox;
                let dy = self.y - oy;
                let d2 = dx * dx + dy * dy;
                if d2 > 0.0 && d2 < 150.0 * 150.0 {
                    near_dx += dx;
                    near_dy += dy;
                    near_count += 1;
                }
            }
            if near_count > 0 && near_dx * near_dx + near_dy * near_dy > 500.0 {
                self.target_x = self.x + near_dx / near_count as f32 * 2.0;
                self.target_y = self.y + near_dy / near_count as f32 * 2.0;
                self.speed = 1.0;
            }
        }

        if self.state == PetState::Idle {
            self.look_around_cooldown_ms = self.look_around_cooldown_ms.saturating_sub(dt_ms);
            if self.look_around_cooldown_ms == 0 {
                self.state = PetState::LookAround;
                self.look_around_timer_ms = 0;
            }
        }

        if self.state == PetState::Idle {
            self.idle_time_ms += dt_ms;
            if self.idle_time_ms > self.archetype.sleep_timeout_ms {
                self.state = PetState::Sleep;
                self.speed = 0.0;
            }
        }

        self.constrain_to_screen();
    }

    pub fn check_pet_reactions(&mut self, others: &[(f32, f32)], dt_ms: u32) {
        if self.state == PetState::Celebrate {
            return;
        }
        self.pet_reaction_cooldown_ms = self.pet_reaction_cooldown_ms.saturating_sub(dt_ms);
        if self.pet_reaction_cooldown_ms > 0 {
            return;
        }
        for &(ox, oy) in others {
            let dx = self.x - ox;
            let dy = self.y - oy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 40.0 {
                self.state = PetState::Celebrate;
                self.celebrate_time_ms = 0;
                self.speed = 0.0;
                self.pet_reaction_cooldown_ms = 5000;
                break;
            }
        }
    }

    fn at_edge(&self) -> bool {
        let margin = 15.0;
        self.x < margin || self.x > self.screen_w - margin - self.width
    }

    fn constrain_to_screen(&mut self) {
        let margin = 20.0;
        if self.x < margin {
            self.x = margin;
            self.facing_left = false;
        }
        if self.x > self.screen_w - margin {
            self.x = self.screen_w - margin;
            self.facing_left = true;
        }
        if self.y > self.floor_y && self.state != PetState::Float {
            self.y = self.floor_y;
        }
    }

    pub fn apply_avoidance(&mut self, others: &[(f32, f32)], min_dist: f32) {
        if !self.grounded {
            return;
        }
        let mut avoid_dx = 0.0f32;
        let mut avoid_dy = 0.0f32;

        for &(ox, oy) in others {
            let dx = self.x - ox;
            let dy = self.y - oy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > 0.0 && dist < min_dist {
                let force = (min_dist - dist) / min_dist;
                let strength = if dist < min_dist * 0.3 { 12.0 } else { 6.0 };
                avoid_dx += (dx / dist) * force * strength;
                avoid_dy += (dy / dist) * force * strength;
            }
        }

        if avoid_dx != 0.0 || avoid_dy != 0.0 {
            self.target_x += avoid_dx;
            self.target_y += avoid_dy;
            // Also nudge current position slightly to prevent stuck pets
            self.x += avoid_dx * 0.1;
            self.y += avoid_dy * 0.1;
        }
    }

    pub fn set_target(&mut self, tx: f32, ty: f32) {
        self.target_x = tx;
        self.target_y = ty;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_returns_to_idle_after_reaching_target() {
        // Regression: a pet that walked to its target and got no further cursor
        // input used to stay frozen in the Walk pose forever.
        let mut pet = Pet::new(500.0, 1050.0);
        pet.state = PetState::Walk;
        pet.speed = 2.0;
        pet.target_x = 520.0;
        pet.target_y = 1050.0;

        // Advance until it should have covered the 20px gap.
        for _ in 0..200 {
            pet.update_movement(16);
            if pet.state != PetState::Walk {
                break;
            }
        }

        assert_eq!(pet.state, PetState::Idle, "walk should end at Idle");
        assert!(
            (pet.x - 520.0).abs() < 1.0,
            "pet should have reached target"
        );
    }

    #[test]
    fn locomotion_at_target_does_not_get_stuck() {
        // A locomotion state whose target already equals its position must not
        // linger — it should flip to Idle on the next movement tick.
        for state in [PetState::Walk, PetState::Run, PetState::Wander] {
            let mut pet = Pet::new(300.0, 1050.0);
            pet.state = state;
            pet.target_x = pet.x;
            pet.target_y = pet.y;
            pet.update_movement(16);
            assert_eq!(pet.state, PetState::Idle, "{state:?} should reset to Idle");
        }
    }

    #[test]
    fn test_animation_frame_advancement() {
        let frames = vec![
            AnimationFrame {
                x: 0,
                y: 0,
                width: 32,
                height: 32,
            },
            AnimationFrame {
                x: 32,
                y: 0,
                width: 32,
                height: 32,
            },
        ];
        let spec = AnimationSpec {
            state: PetState::Idle,
            sprite_sheet: SpriteSheet {
                image_path: "idle.png".to_string(),
                frame_width: 32,
                frame_height: 32,
                frames,
            },
            fps: 10,
            loop_anim: true,
        };

        let mut pet = Pet::new(0.0, 0.0);
        assert_eq!(pet.frame_index, 0);

        pet.update_animation(50, &spec);
        assert_eq!(pet.frame_index, 0);

        pet.update_animation(60, &spec);
        assert_eq!(pet.frame_index, 1);
    }

    #[test]
    fn test_ai_sleep_after_idle() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.archetype = PetArchetype::ground_dweller();
        pet.cursor_attraction_radius = pet.archetype.attraction_radius;
        pet.cursor_known = false;
        pet.wander_timer_ms = 20_000; // prevent wander from triggering
        pet.look_around_cooldown_ms = 20_000; // prevent look-around
        pet.update_ai(11_000, 0.0, 0.0, false, &[]);
        assert_eq!(pet.state, PetState::Sleep);
    }

    #[test]
    fn test_ai_wakes_on_cursor_move() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.state = PetState::Sleep;
        pet.idle_time_ms = 15_000;
        pet.update_ai(100, 200.0, 200.0, true, &[]);
        assert!(pet.state == PetState::Walk || pet.state == PetState::Run);
    }

    #[test]
    fn test_ai_runs_on_fast_cursor() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.cursor_x = 0.0;
        pet.cursor_y = 0.0;
        pet.update_ai(16, 500.0, 500.0, true, &[]);
        assert_eq!(pet.state, PetState::Run);
    }

    #[test]
    fn test_falling_physics() {
        let mut pet = Pet::new_falling(100.0, 1920.0, 1080.0);
        assert!(!pet.grounded);
        assert_eq!(pet.state, PetState::Fall);
        let vy_before = pet.vy;

        pet.update_movement(16);
        assert!(pet.vy > vy_before, "gravity should increase vy");
    }

    #[test]
    fn test_landing() {
        let mut pet = Pet::new_falling(100.0, 1920.0, 1080.0);
        pet.y = pet.floor_y - 1.0;
        pet.vy = 3.0;
        for _ in 0..10 {
            pet.update_movement(16);
            if pet.grounded {
                break;
            }
        }
        assert!(pet.grounded);
        assert_eq!(pet.state, PetState::Land);
    }

    #[test]
    fn test_archetype_not_null() {
        for _ in 0..20 {
            let arch = PetArchetype::random();
            let total = arch.idle_weight
                + arch.wander_weight
                + arch.float_weight
                + arch.tumble_weight
                + arch.climb_weight
                + arch.stomp_weight;
            assert!(total > 0.0, "archetype {} has zero weight", arch.name);
        }
    }

    #[test]
    fn test_float_movement() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.state = PetState::Float;
        pet.float_base_y = 200.0;
        pet.y = 200.0;
        pet.target_x = 300.0;
        pet.speed = 1.0;
        let y_before = pet.y;
        pet.update_movement(16);
        assert!((pet.y - y_before).abs() < 3.0, "float should bob gently");
    }

    #[test]
    fn test_tumble_movement() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.state = PetState::Tumble;
        pet.target_x = 300.0;
        pet.speed = 3.0;
        pet.floor_y = 100.0;
        pet.y = 100.0;
        pet.update_movement(16);
        assert!(pet.x > 100.0, "tumble should move right");
        assert_eq!(pet.y, pet.floor_y, "tumble stays on floor");
    }

    #[test]
    fn test_squish_death_cycle() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.squish();
        assert_eq!(pet.state, PetState::Squish);
        for _ in 0..30 {
            pet.update_movement(16);
            if pet.state == PetState::Splat || pet.state == PetState::Angel {
                break;
            }
        }
        assert!(
            pet.state == PetState::Splat || pet.state == PetState::Angel,
            "squish should lead to splat or angel, got {:?}",
            pet.state
        );
    }

    #[test]
    fn test_stomp_movement() {
        let mut pet = Pet::new(100.0, 100.0);
        pet.state = PetState::Stomp;
        pet.target_x = 500.0;
        pet.target_y = 10.0;
        pet.speed = 3.0;
        pet.floor_y = 100.0;
        pet.y = 100.0;
        pet.update_movement(16);
        assert!(pet.x > 100.0, "stomp should move right");
        assert!(pet.y < pet.floor_y, "stomp should be above floor");
    }

    #[test]
    fn test_pick_idle_behavior() {
        let arch = PetArchetype::tumbler();
        let mut tumble_count = 0;
        for _ in 0..100 {
            if arch.pick_idle_behavior(false) == PetState::Tumble {
                tumble_count += 1;
            }
        }
        assert!(
            tumble_count > 30,
            "tumbler archetype should pick Tumble often"
        );
    }
}
