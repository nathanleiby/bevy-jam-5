#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::f64::EPSILON;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig};

pub fn bodies_plugin(app: &mut App) {
    app.insert_resource(Timestep(0))
        .register_type::<Body>()
        .add_systems(Startup, setup_shapes)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, change_orbits)
        .add_systems(Update, change_timestep)
        .add_systems(Update, move_shapes);
}

// TODO: why isn't deref mut working to avoid need for .0 ?
// TODO: Update this to be a float, but make timestep lock to 1.0 and do +1, -1 (for debugging? for gameplay?)
#[derive(Resource, Deref, DerefMut)]
struct Timestep(usize);

const MIN_TIMESTEP: usize = 0;
const MAX_TIMESTEP: usize = 200; // Not sure about this, but some kinda bounds to avoid too many cycles for player to consider? Think opus magnum

fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: build_puzzle() .. can just compute distances, bodies don't really matter given Mass is N/A except central mass
    let bodies = bodies_from_periods(vec![0., f64::sqrt(3.), 2., 3., 4., 5., 6.]);

    // draw orbits
    let orbit_color = Color::srgb(0.9, 0.9, 0.9);
    let distances: Vec<f64> = bodies
        .iter()
        .map(|b| b.distance_from_central_body)
        .collect();

    for d in distances {
        if d == 0. {
            continue; // don't draw orbit (point!) for central body
        }
        let inner_radius = d as f32 * DISTANCE_UI_SCALE - 1.;
        let outer_radius = inner_radius + 2.;

        let shape = Mesh2dHandle(meshes.add(Annulus::new(inner_radius, outer_radius)));

        commands.spawn((MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(orbit_color),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },));
    }

    for body in bodies {
        let radius = (body.mass * 5.) as f32;
        let shape = Mesh2dHandle(meshes.add(Circle { radius }));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(body.color),
                transform: Transform::from_xyz(
                    body.distance_from_central_body as f32 * DISTANCE_UI_SCALE,
                    0.0,
                    0.0,
                ),
                ..default()
            },
            Name::new(format!("body_{}", body.distance_from_central_body)), // TODO: better naming
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

fn change_orbits(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Body, &mut Transform)>) {
    if !input.just_pressed(KeyCode::KeyO) {
        return;
    }

    // Get all distances from the bodies
    let mut distances = vec![];
    for (body, _) in &query {
        distances.push(body.distance_from_central_body);
    }

    // shuffle distances
    let mut rng = thread_rng();
    distances.shuffle(&mut rng);

    // update bodies
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
    for (body, mut transform) in &mut query {
        if body.distance_from_central_body < EPSILON {
            transform.translation = Vec3::ZERO;
            // approx 0
            // ignore the central body of the system
            continue;
        }

        let standard_grav_param = 1.; // TODO: G(m1+m2), or G(M) if one body is much larger
                                      // For better physics:
                                      // - [ ] tweak sun size to affect standard_grav_param
                                      // - [ ] assign orbits for nice polyrhythms

        let orbital_period =
            2. * PI * f64::sqrt(body.distance_from_central_body.powi(3) / standard_grav_param);
        println!("Body = {:?} .. Orbital Period = {:?}", body, orbital_period);

        let mut timestep_prime = (timestep.0 as f64);
        while timestep_prime > orbital_period {
            timestep_prime -= orbital_period;
        }
        let cycle_position = timestep_prime / orbital_period;

        let angle_radians: f64 = 2. * PI * cycle_position;
        let x = body.distance_from_central_body * angle_radians.cos();
        let y = body.distance_from_central_body * angle_radians.sin();
        transform.translation = Vec3::new(
            x as f32 * DISTANCE_UI_SCALE,
            y as f32 * DISTANCE_UI_SCALE,
            0.,
        );
    }
}

#[derive(Component, Debug, Reflect)]
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

const DISTANCE_UI_SCALE: f32 = 300.;

fn bodies_from_periods(periods: Vec<f64>) -> Vec<Body> {
    // given a list of orbital periods, compute the radii
    let mut out = vec![];
    for (idx, period) in periods.iter().enumerate() {
        let hue = (1. - idx as f32 / periods.len() as f32) * 360.;
        let color = Color::hsl(hue, 0.95, 0.7);

        // TODO: simplified formula
        // TODO: confirk works for central body? shoiuld be 0
        let distance = f64::powf((period / (2. * PI)).powi(2), 1. / 3.);
        // let orbital_period_secs = (4. * pi_2 * r_3) / (gravity * m_central);

        let mass = 1.;

        let b = Body::new(distance, mass, color);
        out.push(b);
    }

    out
}
