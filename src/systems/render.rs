use specs::{ReadStorage, System, Join};

use crate::components::Position;
use crate::io;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        render(position.join(), io::log);
    }
}

/// A testable function
fn render<'a>(
    position_iter: impl IntoIterator<Item=&'a Position>,
    #[cfg(not(test))] log: impl Fn(&str) -> (),
    #[cfg(test)] mut log: impl FnMut(&str) -> (),
) {
    for position in position_iter {
        log(&format!("position {} {}", position.x, position.y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_test() {
        let positions = vec![
            Position { x: 0.0, y: 0.0 },
            Position { x: 1.0, y: 1.0 },
        ];
        let mut strings = Vec::new();

        render(&positions, |s| strings.push(s.to_owned()));

        let iter = positions.iter().zip(strings.iter());

        for (p, s) in iter {
            assert_eq!(&format!("position {} {}", p.x, p.y), s);
        }
    }
}
