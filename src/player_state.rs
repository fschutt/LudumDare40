use camera::Camera;
use physics::PhysicsWorld;

/// The state of the player in the game world
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// 2D camera, relative to the game world
    pub camera: Camera,
    pub physics_world: PhysicsWorld,
    pub highscore: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            camera: Camera { x: 0.0, y: 0.0 },
            physics_world: PhysicsWorld { },
            highscore: { 0.0 }
        }
    }
}
