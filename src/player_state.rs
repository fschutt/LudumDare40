use camera::Camera;
use physics::{PhysicsWorld, PhysicsFinalizedData, PlayerResult, MAX_SPEED, CratePosition};
use input::GameInputEvent;
use std::time::{Duration, Instant};

/// The state of the player in the game world
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// 2D camera, relative to the game world
    pub camera: Camera,
    pub floor_height: f32,
    pub physics_world: PhysicsWorld,
    pub highscore: f32,
    pub player_wants_box: bool,
    pub player_carrying_crate: Option<CratePosition>,
    pub last_tick_update: Instant,
}

impl PlayerState {
    /// Calculates all the positions, etc.
    pub fn finalize(&mut self, events: Vec<GameInputEvent>) -> PhysicsFinalizedData {

        const TIME_NANOS_PER_FRAME: f32 = 1_000_000_000.0 / 60.0;
        const GRAVITY_PER_FRAME: f32 = 9_800_000_000.0 / TIME_NANOS_PER_FRAME;

        let now = Instant::now();
        let tick = now - self.last_tick_update;
        self.last_tick_update = now;
        let tick_nanos = (tick.as_secs() * 1_000_000_000) + tick.subsec_nanos() as u64;
        let time_diff = tick_nanos as f32 / TIME_NANOS_PER_FRAME;

        let mut x_diff = 0.0;
        let mut y_diff = 0.0;

        for event in events {
            match event {
                GameInputEvent::PlayerJump => {
                    y_diff += 20.0;
                },
                GameInputEvent::PlayerGoDown => {
                    y_diff -= 20.0;
                },
                GameInputEvent::PlayerGoRight => {
                    x_diff += 20.0;
                },
                GameInputEvent::PlayerGoLeft => {
                    x_diff -= 20.0;
                },
                GameInputEvent::PlayerTakeBox => {
                    if !self.player_wants_box {
                        self.player_wants_box = true;
                    }
                },
            }
        }

        // keep character in bounds of the screen
        if (self.physics_world.player_position.y + self.physics_world.player_position.y) <
           (self.camera.y + self.floor_height)
        {
            y_diff = self.camera.y + self.floor_height;
        }

        // -- spawn crates

        if now - self.physics_world.last_crate_spawned > Duration::from_secs(3) {
            self.physics_world.spawn_crate(0.0, 0.0);
            self.physics_world.last_crate_spawned = now;
        }

        let mut new_crates: Vec<CratePosition> = self.physics_world.crates.iter().cloned().filter_map(|mut crate_box| {
            if crate_box.x + crate_box.width > (self.camera.x + self.camera.screen_width) ||
                crate_box.y + crate_box.height > (self.camera.y + self.camera.screen_height)
            {
                // remove crates that have flown outside the screen
                None
            } else {
                crate_box.x += 1.0;
                crate_box.y += 1.0;
                Some(crate_box)
            }
        }).collect();

        self.physics_world.player_position.x += x_diff;
        self.physics_world.player_position.y += y_diff;

        // TODO: check if a crate has fallen down, if so, end the game
        let game_result = PlayerResult::PlayerOk;

        if self.player_wants_box {
            if let Some(idx) = player_intersect_crate(self.physics_world.player_position,
                                                      &self.physics_world.crates)
            {
                // add the crate the player is carrying to the crates
                if let Some(box_crate) = self.player_carrying_crate {
                    // if the event was to pick up the crate, let the crate fall
                    self.physics_world.crates.push(box_crate.clone());
                    self.player_carrying_crate = None;
                } else {
                    // remove the crates from the default crate stack
                    let carrying_crate = self.physics_world.crates.remove(idx);
                    self.player_carrying_crate = Some(carrying_crate);
                }
            }
            self.player_wants_box = false;
        };

        self.physics_world.crates = new_crates.clone();

        let new_highscore = self.physics_world.crates.iter().map(|c|
            // TODO: add rotation for crates
            (c.y + c.height) as u32
        ).max();

        self.highscore = new_highscore.map(|x| x as f32).unwrap_or(0.0);

        // push the crate on the head of the player if he is carrying a crate
        if self.player_carrying_crate.is_some() {
            new_crates.push(CratePosition {
                x: self.physics_world.player_position.x + ((self.physics_world.player_position.width / 2.0) - 25.0),
                y: self.physics_world.player_position.y + self.physics_world.player_position.height,
                width: 50.0,
                height: 50.0,
            });
        }

        PhysicsFinalizedData {
            crates: new_crates,
            result: game_result,
            player_position: self.physics_world.player_position,
            highscore: self.highscore,
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            player_wants_box: false,
            player_carrying_crate: None,
            floor_height: 25.0,
            last_tick_update: Instant::now(),
            // TODO: this is bad design
            camera: Camera { x: 0.0, y: 0.0, screen_width: 800.0, screen_height: 600.0 },
            physics_world: PhysicsWorld::default(),
            highscore: { 0.0 },
        }
    }
}

// shitty AABB test
fn player_intersect_crate(player_position: PlayerSpritePosition, crates: &Vec<CratePosition>) -> Option<usize> {

    let halfwidth_player = player_position.width / 2.0;
    let halfheight_player = player_position.height / 2.0;
    let center_player_x = player_position.x + halfwidth_player;
    let center_player_y = player_position.y + halfheight_player;

    for (idx, box_crate) in crates.iter().enumerate() {
        let halfwidth_crate = box_crate.width / 2.0;
        let halfheight_crate = box_crate.height / 2.0;
        let center_crate_x = box_crate.x + halfwidth_crate;
        let center_crate_x = box_crate.y + halfheight_crate;

        if
        ((center_player_x - center_crate_x).abs() < (halfwidth_crate + halfwidth_player)) &&
        ((center_player_y - center_player_y).abs() < (halfheight_crate + halfheight_player)) {
            return Some(idx);
        }
    }

    None
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerSpritePosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for PlayerSpritePosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        }
    }
}
