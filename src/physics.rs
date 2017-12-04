//! Physics system
//!
//! Calculates the physics for the game world, returns a description of the world
//! in world coordinates

use player_state::{PlayerSpritePosition, PlayerState};
use input::GameInputEvent;

/// If the player presses a key, he should arrive at his goal (with linear interpolation)
/// in `SPEED_FACTOR` seconds
pub const SPEED_FACTOR: f32 = 0.25;

/// Maximum speed the player can go, in any direction (plus an additional SPEED_FACTOR)
pub const MAX_SPEED: f32 = 0.5;

#[derive(Default, Debug, Clone)]
pub struct PhysicsWorld {
    /// Player velocity, in X and Y
    pub player_velocity: PlayerVelocity,
    pub crates: Vec<CratePosition>,
    pub player_position: PlayerSpritePosition,
    pub gravity: f32,
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
