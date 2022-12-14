use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{Gameplay, GameplayState},
};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, boxes, box_spots, positions) = data;

        // get all boxes indexed by position
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        for (box_spot, position) in (&box_spots, &positions).join() {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if the_box.colour == box_spot.colour {
                    // continue
                } else {
                    return;
                }
            } else {
                gameplay_state.state = GameplayState::Playing;
                return;
            }
        }

        gameplay_state.state = GameplayState::Won;
    }
}
