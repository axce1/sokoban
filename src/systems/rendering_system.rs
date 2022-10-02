use std::time::Duration;
use crate::components::*;
use crate::resources::*;
use crate::constants::TILE_WIDTH;

use ggez::{Context, graphics::{self, DrawParam, Image, Color}};
use specs::{Join, ReadStorage, System, Read};
use glam::Vec2;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
            .expect("expected drawing queued text");
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
        let path_index = match renderable.kind() {
            RenderableKind::Static => {
                0
            }
            RenderableKind::Animated => {
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };
        let image_path = renderable.path(path_index);
        Image::new(self.context, image_path).expect("expected image")
    }
}


impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        Read<'a, Gameplay>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            positions,
            renderables,
            gameplay,
            time
        ) = data;

        // clear the screen
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let image = self.get_image(renderable, time.delta);

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        graphics::present(self.context).expect("expected to present");
    }
}