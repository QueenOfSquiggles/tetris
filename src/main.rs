use bevy::{ecs::world::error, math::U16Vec2, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Grid>()
        .add_systems(Startup, initialize)
        .observe(on_update_grid)
        .run();
}

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const TILE_SIZE: f32 = 32.0;

#[derive(Resource, Hash, Clone, Default)]
struct Grid(pub [[GridObject; GRID_HEIGHT]; GRID_WIDTH]);

#[derive(Hash, Clone, Default)]
enum GridObject {
    #[default]
    None,
    Tetrino,
}

#[derive(Component)]
struct GridRender;

#[derive(Event, Default)]
struct RedrawGrid(Option<Vec<U16Vec2>>);

fn initialize(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle::new_with_far(1.0));
    cmd.spawn((GridRender, SpatialBundle::default()))
        .with_children(|b| {
            let bg_sprite = assets.load("textures/Back tiles/BackTile_06.png");
            for x in 0..GRID_WIDTH {
                for y in 0..GRID_HEIGHT {
                    b.spawn(SpriteBundle {
                        texture: bg_sprite.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32) * TILE_SIZE,
                            (y as f32) * TILE_SIZE,
                            -0.1,
                        ),
                        ..default()
                    });
                }
            }
        });
    cmd.trigger(RedrawGrid::default());
}

fn on_update_grid(
    _: Trigger<RedrawGrid>,
    grid_query: Query<(Entity, &Children), With<GridRender>>,
    grid: Res<Grid>,
) {
    let Ok(e) = grid_query.get_single() else {
        return;
    };
}
