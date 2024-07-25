#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::f64::EPSILON;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig};

pub fn bodies_plugin(app: &mut App) {
    app.insert_resource(Timestep(0))
        .add_systems(Startup, setup_shapes)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, change_orbits)
        .add_systems(Update, change_timestep)
        .add_systems(Update, move_shapes);
}

// TODO: why isn't deref mut working to avoid need for .0 ?
#[derive(Resource, Deref, DerefMut)]
struct Timestep(usize);

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

    let bodies = [
        Body::new(DISTANCES[0], M_EARTH, Color::srgb(1., 0., 0.)),
        Body::new(DISTANCES[1], M_EARTH * 1.25, Color::srgb(1., 1., 0.)),
        Body::new(DISTANCES[2], M_EARTH * 1.5, Color::srgb(0., 1., 1.)),
        Body::new(DISTANCES[3], M_EARTH * 1.75, Color::srgb(1., 0., 1.)),
        Body::new(DISTANCES[4], M_EARTH * 3., Color::srgb(0., 0.5, 1.)),
        Body::new(DISTANCES[5], M_EARTH * 5., Color::srgb(0.5, 0., 0.5)),
    ];
    //  let (d0, m0) = (0., M_EARTH);
    //     let (d1, m1) = (100., 2. * M_EARTH);
    //     let (d2, m2) = (200., 3. * M_EARTH);

    // draw orbits
    let orbit_color = Color::srgb(0.9, 0.9, 0.9);
    for d in DISTANCES {
        if d == 0. {
            continue; // don't draw orbit (point!) for central body
        }
        let shape = Mesh2dHandle(meshes.add(Annulus::new(d as f32 - 1.0, d as f32 + 1.0)));

        commands.spawn((MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(orbit_color),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },));
    }

    // for (d, m) in [(d1, m1), (d2, m2)] {
    for body in bodies {
        // spawn sat
        let radius = (body.mass / M_EARTH * 5.) as f32;
        let shape = Mesh2dHandle(meshes.add(Circle { radius }));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(body.color),
                transform: Transform::from_xyz(body.distance_from_central_body as f32, 0.0, 0.0),
                ..default()
            },
            body,
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

static M_EARTH: f64 = 5.98e24;

const DISTANCES: [f64; 6] = [0., 75., 120., 175., 250., 350.];

fn change_orbits(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Body, &mut Transform)>) {
    if !input.just_pressed(KeyCode::KeyO) {
        return;
    }

    let mut distances = DISTANCES.clone().to_vec();
    // let slice: &mut [u32] = &mut distances;
    let mut rng = thread_rng();
    distances.shuffle(&mut rng);

    let mut idx = 0;
    for (mut body, _transform) in &mut query {
        body.distance_from_central_body = distances[idx];
        idx += 1;
    }
}

fn move_shapes(
    // time: Res<Time>,
    mut query: Query<(&mut Body, &mut Transform)>,
    timestep: Res<Timestep>,
) {
    // Cycle duration

    let pi_2 = PI.powi(2);
    let m_central = M_EARTH;
    let gravity: f64 = 6.678e11;
    let distance_scale = 1e11; // multiplier for distance like 100, 250
    let timestep_scale = 1e3;

    for (body, mut transform) in &mut query {
        if body.distance_from_central_body < EPSILON {
            transform.translation = Vec3::ZERO;
            // approx 0
            // ignore the central body of the system
            continue;
        }
        // TODO: compute for various orbital radii, based on time elapsed

        let r_3 = (body.distance_from_central_body * distance_scale).powi(3);
        let orbital_period_secs = (4. * pi_2 * r_3) / (gravity * m_central);
        println!("Satellite = {:?}", body);
        println!("Orbital period = {:?}", orbital_period_secs);

        // let cycle_position = (timestep.0 % TIMESTEP_PER_CANONICAL_CYCLE) as f64
        //     / TIMESTEP_PER_CANONICAL_CYCLE as f64;
        let mut timestep_prime = (timestep.0 as f64) * timestep_scale;
        while timestep_prime > orbital_period_secs {
            timestep_prime -= orbital_period_secs;
        }
        let cycle_position = timestep_prime / orbital_period_secs;

        let angle_radians: f64 = 2. * PI * cycle_position;
        let x = body.distance_from_central_body * angle_radians.cos();
        let y = body.distance_from_central_body * angle_radians.sin();
        transform.translation = Vec3::new(x as f32, y as f32, 0.);
    }
}

#[derive(Component, Debug)]
struct Body {
    distance_from_central_body: f64,
    mass: f64,
    color: Color,
}

impl Body {
    fn new(distance_from_central_body: f64, mass: f64, color: Color) -> Self {
        Self {
            distance_from_central_body,
            mass,
            color,
        }
    }
}

// TODO: refactor out the idea of ring radius into the System (puzzle) itself. The bodies don't need to store it
// #[derive(Component, Debug)]
// struct System {
//     positions: Vec<Vec3>,
// }

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
    // press "r" to reset timestamp
    if input.just_pressed(KeyCode::KeyR) {
        if timestep.0 < MAX_TIMESTEP {
            timestep.0 = 0;
        }
    }

    // press right arrow or left arrow to adjust timestamp
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

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        wireframe_config.global = !wireframe_config.global;
    }
}
