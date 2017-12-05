//! Physics system
//!
//! Calculates the physics for the game world, returns a description of the world
//! in world coordinates

use player_state::{PlayerSpritePosition, PlayerState};
use input::GameInputEvent;
use std::time::Instant;

/// If the player presses a key, he should arrive at his goal (with linear interpolation)
/// in `SPEED_FACTOR` seconds
pub const SPEED_FACTOR: f32 = 0.25;

/// Maximum speed the player can go, in any direction (plus an additional SPEED_FACTOR)
pub const MAX_SPEED: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct PhysicsWorld {
    /// Player velocity, in X and Y
    pub player_velocity: PlayerVelocity,
    pub crates: Vec<CratePosition>,
    pub player_position: PlayerSpritePosition,
    pub last_crate_spawned: Instant,
    pub gravity: f32,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            last_crate_spawned: Instant::now(),
            crates: Vec::new(),
            player_position: PlayerSpritePosition::default(),
            player_velocity: PlayerVelocity::default(),
            gravity: 9.8,
        }
    }
}

impl PhysicsWorld {
    // spawns a new crate
    pub fn spawn_crate(&mut self, x_pos: f32, y_pos: f32) {
        self.crates.push(CratePosition {
            x: x_pos,
            y: y_pos,
            width: 32.0,
            height: 32.0,
        });
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerVelocity {
    pub x: f32,
    pub y: f32,
}

impl Default for PlayerVelocity {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CratePosition {
    /// X from the bottom of the screen
    pub x: f32,
    /// Y from the left of the screen
    pub y: f32,
    /// width of the crate sprite
    pub width: f32,
    /// height of the crate
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayerResult {
    PlayerOk,
    PlayerQuitGame,
    PlayerHasLost,
}

/// The physics world, for submission to the renderer
///
/// i.e. where, for this one frame,
#[derive(Debug, Clone)]
pub struct PhysicsFinalizedData {
    pub player_position: PlayerSpritePosition,
    pub crates: Vec<CratePosition>,
    /// Has the player quit or lost the game?
    pub result: PlayerResult,
    pub highscore: f32,
}
