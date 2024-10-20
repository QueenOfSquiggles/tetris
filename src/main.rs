use bevy::prelude::*;
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::{GlobalEntropy, WyRand},
};

type Random = GlobalEntropy<WyRand>;

mod grid;
mod inputs;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, EntropyPlugin::<WyRand>::default()));
    grid::init(&mut app);
    inputs::init(&mut app);
    app.run();
}
