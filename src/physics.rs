//! Physics system
//!
//! Calculates the physics for the game world, returns a description of the world
//! in world coordinates

#[derive(Debug, Clone)]
pub struct PhysicsWorld {

}

/// The physics world, for submission to the renderer
pub struct PhysicsFinalizedData {

}


impl PhysicsWorld {
    /// Calculates all the positions
    pub fn finalize(&self) -> PhysicsFinalizedData {
        PhysicsFinalizedData { /* todo*/}
    }
}
