use specs::{Builder, RunNow, World, WorldExt};
use wasm_bindgen::prelude::*;

pub mod components;
pub mod io;
pub mod systems;

use components::Position;
use systems::Render;

#[wasm_bindgen(start)]
pub fn run() {
    // Create a new World, register component types (which also add internal storage to the World)
    let mut world = World::new();
    components::register(&mut world);

    // Create an entity (Filling in world data)
    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();

    // Run a system (May replace run_now with Dispatcher logic)
    let mut render = Render;
    render.run_now(&world);

    // If entities are created or deleted while a system is running,
    // .maintain() will record the changes in its internal data structure
    world.maintain();
}
