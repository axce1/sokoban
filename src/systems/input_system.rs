use crate::components::*;
use crate::constants::*;
use crate::resources::InputQueue;
use ggez::event::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use std::collections::HashMap;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            // get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {
                // get all movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, MAP_HEIGHT, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, MAP_WIDTH, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    //find movable
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => match immov.get(&pos) {
                            Some(_id) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }
        }

        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            println!("{:?}", position);
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}