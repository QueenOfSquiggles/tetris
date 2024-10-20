use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputMove(TetrinoMove);

pub enum TetrinoMove {
    Left,
    Right,
    Clockwise,
    CounterClockWise,
}

pub fn init(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<Actions>::default());
}

#[derive(Hash, Clone, Copy, Reflect, PartialEq, Eq, Debug)]
enum Actions {
    Movement,
    Rotation,
}

impl Actionlike for Actions {
    fn input_control_kind(&self) -> InputControlKind {
        match *self {
            Actions::Movement => InputControlKind::Axis,
            Actions::Rotation => InputControlKind::Axis,
        }
    }
}

// fn emit_input_events()
