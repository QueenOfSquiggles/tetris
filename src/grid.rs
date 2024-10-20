use bevy::{math::U16Vec2, prelude::*};
use rand::seq::SliceRandom;

use crate::Random;

pub fn init(app: &mut App) {
    app.init_resource::<Grid>()
        .init_resource::<BlockTextures>()
        .add_systems(Startup, initialize)
        .add_systems(PostStartup, spawn_tetrino)
        .add_systems(Update, fall_tetrino)
        .observe(on_update_grid);
}

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const TILE_SIZE: f32 = 32.0;
const LAYER_BG: f32 = 0.0;
const LAYER_TETRINO: f32 = 0.2;
const LAYER_FG: f32 = 0.4;

#[derive(Resource, Hash, Clone, Default)]
struct Grid(pub [[GridObject; GRID_HEIGHT]; GRID_WIDTH]);

#[derive(Hash, Clone, Default)]
enum GridObject {
    #[default]
    None,
    Tetrino(BlockColor),
}

#[derive(Hash, Clone, Default, Copy, PartialEq, Eq)]
enum BlockColor {
    #[default]
    Blue,
    Green,
    Orange,
    Pink,
    Red,
    Yellow,
}

#[derive(Component)]
struct GridRender;

#[derive(Event, Default)]
struct RedrawGrid(Option<Vec<U16Vec2>>);

#[derive(Resource, Default)]
struct BlockTextures {
    bg: Handle<Image>,
    blue: Handle<Image>,
    green: Handle<Image>,
    orange: Handle<Image>,
    pink: Handle<Image>,
    red: Handle<Image>,
    yellow: Handle<Image>,
}

impl BlockTextures {
    pub fn texture_for_obj(&self, obj: &GridObject) -> Handle<Image> {
        match obj {
            GridObject::None => self.bg.clone(),
            GridObject::Tetrino(block_color) => self.texture_for_color(block_color),
        }
    }

    pub fn texture_for_color(&self, col: &BlockColor) -> Handle<Image> {
        match *col {
            BlockColor::Blue => self.blue.clone(),
            BlockColor::Green => self.green.clone(),
            BlockColor::Orange => self.orange.clone(),
            BlockColor::Pink => self.pink.clone(),
            BlockColor::Red => self.red.clone(),
            BlockColor::Yellow => self.yellow.clone(),
        }
    }
}

#[derive(Component)]
struct Tetrino(BlockColor);

fn initialize(mut cmd: Commands, assets: Res<AssetServer>, mut blocks: ResMut<BlockTextures>) {
    blocks.bg = assets.load("textures/Back tiles/BackTile_06.png");
    blocks.blue = assets.load("textures/Tiles blue/tileBlue_13.png");
    blocks.green = assets.load("textures/Tiles green/tileGreen_13.png");
    blocks.orange = assets.load("textures/Tiles orange/tileOrange_13.png");
    blocks.pink = assets.load("textures/Tiles pink/tilePink_13.png");
    blocks.red = assets.load("textures/Tiles red/tileRed_13.png");
    blocks.yellow = assets.load("textures/Tiles yellow/tileYellow_13.png");

    cmd.spawn(Camera2dBundle::new_with_far(1.0));
    cmd.spawn((
        GridRender,
        SpatialBundle {
            transform: Transform::from_xyz(
                -TILE_SIZE * (GRID_WIDTH as f32) * 0.5,
                -TILE_SIZE * (GRID_HEIGHT as f32) * 0.5,
                LAYER_BG,
            ),
            ..default()
        },
    ))
    .with_children(|b| {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                b.spawn(SpriteBundle {
                    texture: blocks.bg.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32) * TILE_SIZE,
                        (y as f32) * TILE_SIZE,
                        0.0,
                    ),
                    ..default()
                });
            }
        }
    });
    cmd.trigger(RedrawGrid::default());
}

fn spawn_tetrino(mut cmd: Commands, rng: ResMut<Random>, tex: Res<BlockTextures>) {
    let color = [
        BlockColor::Blue,
        BlockColor::Green,
        BlockColor::Orange,
        BlockColor::Pink,
        BlockColor::Red,
        BlockColor::Yellow,
    ]
    .choose(&mut rng.into_inner())
    .cloned()
    .unwrap_or_default();

    cmd.spawn((
        Tetrino(color.clone()),
        SpatialBundle {
            transform: Transform::from_xyz(
                0.0,
                TILE_SIZE * (GRID_HEIGHT as f32) * 0.5,
                LAYER_TETRINO,
            ),
            ..default()
        },
    ))
    .with_children(|b| {
        b.spawn(SpriteBundle {
            texture: tex.texture_for_color(&color),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        });
    });
}

fn fall_tetrino(mut query: Query<&mut Transform, With<Tetrino>>, time: Res<Time>) {
    for mut trans in query.iter_mut() {
        trans.translation.y -= TILE_SIZE * 0.2 * time.delta_seconds();
    }
}

fn on_update_grid(
    trigger: Trigger<RedrawGrid>,
    grid_query: Query<(Entity, &Children), With<GridRender>>,
    grid: Res<Grid>,
    textures: Res<BlockTextures>,
    mut cmd: Commands,
) {
    let Ok(e) = grid_query.get_single() else {
        return;
    };
    for (index, child) in e.1.iter().enumerate() {
        let grid_pos = (index % GRID_WIDTH, index / GRID_WIDTH);
        if let Some(targets) = &trigger.event().0 {
            if !targets.is_empty()
                && !targets.contains(&U16Vec2 {
                    x: grid_pos.0 as u16,
                    y: grid_pos.1 as u16,
                })
            {
                // provided there is a specified set of modified tiles and this current tile is not one of them, skip
                continue;
            }
        }

        let GridObject::Tetrino(color) = &grid.0[grid_pos.0][grid_pos.1] else {
            cmd.entity(*child).insert(textures.bg.clone());
            continue;
        };
        cmd.entity(*child).insert(textures.texture_for_color(color));
    }
}
