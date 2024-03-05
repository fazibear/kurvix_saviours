use bevy::prelude::*;

pub struct BackgroundPlugin;
use rand::Rng;

#[derive(Component, Debug, Default)]
pub struct Background {
    pub display_time: f32,
    pub displayed_time: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("background.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(912., 16.),
        1,
        4,
        None,
        None,
    ));
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        commands.spawn((
            Background {
                display_time: 0.1,
                displayed_time: 0.0,
            },
            SpriteSheetBundle {
                texture: texture.clone(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(0.6))
                    .with_translation(Vec3::new(0.0, 0.0, -0.9)),
                atlas: TextureAtlas {
                    layout: layout.clone(),
                    index: rng.gen_range(0..4),
                },
                ..default()
            },
        ));
    }
}

fn update(
    mut query: Query<(&mut Background, &mut TextureAtlas, &mut Transform), With<Background>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (mut background, mut texture_atlas, mut transform) in query.iter_mut() {
        if time.elapsed_seconds() - background.displayed_time > background.display_time {
            background.displayed_time = time.elapsed_seconds();
            transform.translation.x = rng.gen_range(-500.0..500.0);
            transform.translation.y = rng.gen_range(-400.0..400.0);
            texture_atlas.index = rng.gen_range(0..4);
        }
    }
}

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}
