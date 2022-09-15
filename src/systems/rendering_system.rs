use crate::components::*;
use crate::constants::TILE_WIDTH;

use ggez::{
    graphics::{self, DrawParam, Image},
    Context,
};
use glam::Vec2;
use specs::{Join, ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // clear the screen
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.context, renderable.path.clone()).expect("expected render");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        graphics::present(self.context).expect("expected to present");
    }
}
