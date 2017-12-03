use camera::Camera;
use physics::PhysicsWorld;

/// The state of the player in the game world
#[derive(Clone)]
pub struct PlayerState {
    /// 2D camera, relative to the game world
    pub camera: Camera,
    pub physics_world: PhysicsWorld,
    pub highscore: u32,
}
