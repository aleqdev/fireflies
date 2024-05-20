use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*, sprite::Mesh2d, utils::hashbrown::HashMap};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct Firefly {
    timer: f32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    }, BloomSettings::default()));
    commands.insert_resource(ClearColor(Color::BLACK));

    for x in -7..=7 {
        for y in -7..=7 {
            commands.spawn((
                ColorMesh2dBundle {
                    material: colors.add(Color::GRAY),
                    mesh: meshes.add(Mesh::from(Circle::new(6.0))).into(),
                    transform: Transform::from_xyz(
                        x as f32 * 90.0 + rand::thread_rng().gen_range(-15.0..=15.0),
                        y as f32 * 90.0 + rand::thread_rng().gen_range(-15.0..=15.0),
                        0.0,
                    ),
                    ..default()
                },
                Firefly {
                    timer: rand::thread_rng().gen_range(0.0..=3.0)
                },
            ));
        }
    }
}

fn update(
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut fireflies: Query<(Entity, &mut Firefly, &mut Handle<ColorMaterial>, &Transform)>,
    time: Res<Time>,
) {
    let positions: Vec<_> = fireflies.iter().map(|x| (x.0, x.3.translation)).collect();
    let mut changes = HashMap::<u64, u32>::new();

    for (entity, mut firefly, mut color, transform) in fireflies.iter_mut() {
        firefly.timer -= time.delta_seconds();

        if firefly.timer >= 2.9 {
            *color = colors.add(ColorMaterial::from(Color::YELLOW_GREEN * 2.0));
        } else {
            *color = colors.add(ColorMaterial::from(Color::GRAY));
        }

        if firefly.timer <= 0.0 {
            firefly.timer = 3.0;

            for (e, position) in &positions {
                if *e == entity {continue}

                if transform.translation.distance_squared(*position) <= 90.0 * 90.0 * 4.0 {
                    *changes.entry(e.to_bits()).or_default() += 1;
                }
            }
        }
    }

    for (entity, change) in changes.into_iter() {
        let timer = &mut fireflies.get_mut(Entity::from_bits(entity)).unwrap().1.timer;

        if *timer < 1.0 {
            *timer -= change as f32 / 8.0;
        }
        if *timer > 2.0 {
            *timer += change as f32 / 8.0;
        }

        *timer = (*timer).clamp(0.0, 3.0);
    }
}
