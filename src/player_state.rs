use camera::Camera;
use physics::{PhysicsWorld, PhysicsFinalizedData, PlayerResult, MAX_SPEED};
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
    pub player_has_box: bool,
    pub last_tick_update: Instant,
}

impl PlayerState {
    /// Calculates all the positions, etc.
    pub fn finalize(&mut self, events: Vec<GameInputEvent>) -> PhysicsFinalizedData {

        const TIME_NANOS_PER_FRAME: f32 = 1_000_000_000.0 / 60.0;

        let now = Instant::now();
        let tick = now - self.last_tick_update;
        self.last_tick_update = now;
        let tick_nanos = (tick.as_secs() * 1_000_000_000) + tick.subsec_nanos() as u64;
        let time_diff = tick_nanos as f32 / TIME_NANOS_PER_FRAME;

        if let Some(event) = events.get(0) {
            match *event {
                GameInputEvent::PlayerJump => {
                    self.physics_world.player_velocity.y *= 1.1;
                    self.physics_world.player_velocity.y += 0.5;
                },
                GameInputEvent::PlayerGoDown => {
                    self.physics_world.player_velocity.y *= 0.9;
                    self.physics_world.player_velocity.y -= 0.5;
                },
                GameInputEvent::PlayerGoRight => {
                    self.physics_world.player_velocity.x *= 1.1;
                    self.physics_world.player_velocity.x -= 0.5;
                },
                GameInputEvent::PlayerGoLeft => {
                    self.physics_world.player_velocity.x *= 0.9;
                    self.physics_world.player_velocity.x -= 0.5;
                },
                GameInputEvent::PlayerTakeBox => {
                    if !self.player_has_box {
                        self.player_has_box = true;
                    }
                },
            }
        }

        // TODO: express as vector!
        let mut x_diff = ::physics::SPEED_FACTOR * self.physics_world.player_velocity.x * time_diff;
        let mut y_diff = ::physics::SPEED_FACTOR * self.physics_world.player_velocity.y * time_diff;

        self.physics_world.player_velocity.y -= 9_800_000_000.0 / time_diff.powi(2);  // 9.8 m/s2

        // keep character in bounds of the screen
        if (self.physics_world.player_position.y + y_diff) < (self.camera.y + self.floor_height)  {
            y_diff = self.camera.y + self.floor_height;
        }

        self.physics_world.player_position.x += x_diff;
        self.physics_world.player_position.y += y_diff;

        let new_highscore = self.physics_world.crates.iter().map(|c|
            /* TODO: add rotation for crates */
            (c.y + c.height) as u32
        ).max();

        // TODO: check if a crate has fallen down, if so, end the game
        let game_result = PlayerResult::PlayerOk;
        self.highscore = new_highscore.map(|x| x as f32).unwrap_or(0.0);

        let new_crates = self.physics_world.crates.clone();

        if self.player_has_box {
            // add the crate the player is carrying to the crates

            /*
                if the event was to pick up the crate, let the crate fall
            */
        };

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
            player_has_box: false,
            floor_height: 25.0,
            last_tick_update: Instant::now(),
            // TODO: this is bad design
            camera: Camera { x: 0.0, y: 0.0, screen_width: 800.0, screen_height: 600.0 },
            physics_world: PhysicsWorld::default(),
            highscore: { 0.0 },
        }
    }
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

