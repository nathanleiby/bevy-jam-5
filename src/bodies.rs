#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::f64::consts::PI;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{asset::AssetMetaCheck, color::palettes::css::PURPLE};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub fn bodies_plugin(app: &mut App) {
    app.insert_resource(Timestep(0));
    app.add_systems(Startup, setup_shapes);
    app.add_systems(Update, move_shapes);
    app.add_systems(Update, change_timestep);
}

// TODO: why isn't deref mut working to avoid need for .0 ?
#[derive(Resource, Deref, DerefMut)]
struct Timestep(usize);

const TIMESTEP_PER_CANONICAL_CYCLE: usize = 16;
const MIN_TIMESTEP: usize = 0;
const MAX_TIMESTEP: usize = 200; // Not sure about this, but some kinda bounds to avoid too many cycles for player to consider? Think opus magnum

fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: Setup a central body

    // spawn_satellite_entity(&commands, &meshes, &materials, Satellite { radius: 50. });
    // spawn_satellite_entity(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Satellite { radius: 250. },
    // );
    let d1 = 100.;
    let d2 = 200.;

    for distance_from_central_body in [d1, d2] {
        // spawn sat
        let satellite = Satellite::new(distance_from_central_body);
        let shape = Mesh2dHandle(meshes.add(Circle { radius: 5. }));

        // let hue = 180.; // 0 - 360
        let hue = if distance_from_central_body == d1 {
            100.
        } else if distance_from_central_body == d2 {
            200.
        } else {
            0.
        };

        let color = Color::hsl(hue, 0.95, 0.7);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(
                    satellite.distance_from_central_body as f32,
                    0.0,
                    0.0,
                ),
                ..default()
            },
            satellite,
        ));
    }

    commands.spawn(
        TextBundle::from_section("Press `s` to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn move_shapes(
    // time: Res<Time>,
    mut query: Query<(&mut Satellite, &mut Transform)>,
    timestep: Res<Timestep>,
) {
    // Cycle duration

    for (satellite, mut transform) in &mut query {
        // TODO: compute for various orbital radii, based on time elapsed
        let cycle_position = (timestep.0 % TIMESTEP_PER_CANONICAL_CYCLE) as f64
            / TIMESTEP_PER_CANONICAL_CYCLE as f64;

        let angle_radians: f64 = 2. * PI * cycle_position;
        let x = satellite.distance_from_central_body * angle_radians.cos();
        let y = satellite.distance_from_central_body * angle_radians.sin();
        transform.translation = Vec3::new(x as f32, y as f32, 0.);
    }
}

#[derive(Component)]
struct Satellite {
    distance_from_central_body: f64,
    // central_mass: f64,
    // hue: f32,
}

impl Satellite {
    fn new(radius: f64) -> Self {
        Self {
            distance_from_central_body: radius,
            // // central_mass,
            // hue: 180.,
        }
    }
}

// fn spawn_satellite_entity(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     satellite: Satellite,
// ) {
//     let shape = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
//     let hue = 180.; // 0 - 360
//     let color = Color::hsl(hue, 0.95, 0.7);
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: shape,
//             material: materials.add(color),
//             transform: Transform::from_xyz(satellite.radius as f32, 0.0, 0.0),
//             ..default()
//         },
//         satellite,
//     ));
// }

fn change_timestep(input: Res<ButtonInput<KeyCode>>, mut timestep: ResMut<Timestep>) {
    if input.just_pressed(KeyCode::ArrowRight) {
        if timestep.0 < MAX_TIMESTEP {
            timestep.0 += 1;
        }
    }

    if input.just_pressed(KeyCode::ArrowLeft) {
        if timestep.0 > MIN_TIMESTEP {
            timestep.0 -= 1;
        }
    }

    println!("Timestep = {}", timestep.0);
}
