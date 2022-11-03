use std::fmt::{Display, Formatter};
use std::time::Duration;
use ggez::event::KeyCode;
use crate::events::Event;
use specs::World;
use crate::audio::AudioStore;


#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

pub enum GameplayState {
    Won,
    Playing
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}
