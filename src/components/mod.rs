mod position;
mod velocity;

pub use position::Position;
pub use velocity::Velocity;

use specs::{World, WorldExt};

pub fn register(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
}
