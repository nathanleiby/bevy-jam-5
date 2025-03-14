#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_kira_audio::prelude::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::f64::EPSILON;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig};

pub fn bodies_plugin(app: &mut App) {
    app.init_resource::<Timestep>()
        // register types for bevy UI debugger
        .register_type::<Body>()
        .register_type::<IsPlaying>()
        .register_type::<Timestep>()
        .add_plugins(ResourceInspectorPlugin::<Timestep>::default())
        .add_systems(Startup, setup_play_status)
        .add_systems(Startup, setup_shapes)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, change_orbits)
        .add_systems(Update, handle_timestep_input)
        .add_systems(Update, update_timestep)
        .add_systems(Update, move_shapes);
}

#[derive(Component)]
struct Puzzle {

    // TDO: help
    distances:
    solution_timestep: f64,
    solution_distances: Vec<f64>,

    vec![f64::sqrt(3), 2., 3.]



}

// TODO: why isn't deref mut working to avoid need for .0 ?
// TODO: Update this to be a float, but make timestep lock to 1.0 and do +1, -1 (for debugging? for gameplay?)
#[derive(Resource, Deref, DerefMut, Reflect)]
struct Timestep(f64);

// custom implementation for unusual values
impl Default for Timestep {
    fn default() -> Self {
        Timestep(0.)
    }
}

const MIN_TIMESTEP: f64 = 0.;
const MAX_TIMESTEP: f64 = 200.; // Not sure about this, but some kinda bounds to avoid too many cycles for player to consider? Think opus magnum

fn setup_play_status(mut commands: Commands) {
    commands.spawn((IsPlaying(false), Name::new("IsPlaying")));
}

fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: build_puzzle() .. can just compute distances, bodies don't really matter given Mass is N/A except central mass
    // let bodies = bodies_from_periods(vec![0., 1., f64::sqrt(3.), 2., 3., 4., 5., 6., 7., 8.]);
    // let bodies = bodies_from_periods(vec![f64::sqrt(3.), 2., f64::sqrt(6.), 3., f64::sqrt(11.)]);
    let bodies = bodies_from_periods(vec![2., 3.]);
    // let bodies = bodies_from_periods(vec![0.25, 2., 3.]);

    // draw orbits
    let orbit_color = Color::srgba(0.9, 0.9, 0.9, 0.5);
    let distances: Vec<f64> = bodies
        .iter()
        .map(|b| b.distance_from_central_body)
        .collect();
    println!("distances: {:?}", distances);

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

    // draw line of "syzygy"
    let middle_line_color = Color::srgba(0.9, 0.9, 0.9, 0.2);
    let middle_line = Mesh2dHandle(meshes.add(Rectangle::new(1280.0, 2.0)));
    commands.spawn((MaterialMesh2dBundle {
        mesh: middle_line,
        material: materials.add(middle_line_color),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    },));

    // draw bodies
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
    mut query: Query<(&Body, &mut Transform)>,
    timestep: Res<Timestep>,
    mut query2: Query<&mut IsPlaying>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut is_playing = query2.get_single_mut().unwrap();
    if !is_playing.0 {
        return;
    }

    let mut just_passed_go = vec![];
    for (body, mut transform) in &mut query {
        if body.distance_from_central_body < EPSILON {
            transform.translation = Vec3::ZERO;
            // approx 0
            // ignore the central body of the system
            continue;
        }

        // let standard_grav_param = 2.; // TODO: G(m1+m2), or G(M) if one body is much larger
        let standard_grav_param = 1.; // TODO: G(m1+m2), or G(M) if one body is much larger
                                      //                               // For better physics:
                                      //                               // - [ ] tweak sun size to affect standard_grav_param
                                      //                               // - [ ] assign orbits for nice polyrhythms

        // gravity of center and planet matter to each other!

        let orbital_period =
            2. * PI * f64::sqrt(body.distance_from_central_body.powi(3) / standard_grav_param);

        let mut timestep_prime = timestep.0;
        while timestep_prime > orbital_period {
            timestep_prime -= orbital_period;
        }
        let cycle_position = timestep_prime / orbital_period;
        // let cycle_position = timestep.0 / orbital_period;

        let mut timestep_prime_prev = timestep.0 - TIMESTEP_SPEED;
        while timestep_prime_prev > orbital_period {
            timestep_prime_prev -= orbital_period;
        }
        // let cycle_position = timestep_prime_prev / orbital_period;

        // check if any bodies have just passed 0-point
        if timestep_prime < timestep_prime_prev {
            // play SFX
            audio.play(asset_server.load("plop.ogg"));
            just_passed_go.push(body);
        }

        let angle_radians: f64 = 2. * PI * cycle_position;
        let x = body.distance_from_central_body * angle_radians.cos();
        let y = body.distance_from_central_body * angle_radians.sin();
        transform.translation = Vec3::new(
            x as f32 * DISTANCE_UI_SCALE,
            y as f32 * DISTANCE_UI_SCALE,
            0.,
        );
    }

    let did_syzygy = check_for_syzygy(bodies, timestep.0);
    if did_syzygy {
        is_playing.0 = false;
        println!("syzygy!");
    }
}

fn approx_equal(a: f64, b: f64) -> bool {
    f64::abs(a - b) < EPSILON
}

fn check_for_syzygy(bodies: Vec<&Body>, ts: f64) -> bool {
    let expected = vec![2.0, 3.0];

    for e in expected {
        let mut found = false;
        for b in &bodies {
            // if b.distance_from_central_body == e {
            if approx_equal(b.distance_from_central_body, e) {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    return true;

    // expected
    //     .iter()
    //     .map(|e| {
    //         bodies
    //             .iter()
    //             .any(|b| approx_equal(b.distance_from_central_body, *e))
    //     })
    //     .all(|x| x)
}

#[cfg(test)]
mod tests {
    use bevy::color::palettes::css::PURPLE;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_syzygy() {
        let b1 = Body::new(2., 1., PURPLE.into());
        let b2 = Body::new(3., 1., PURPLE.into());
        let bodies = vec![&b1, &b2];
        assert_eq!(check_for_syzygy(bodies), true);
    }

    #[test]
    fn test_syzygy2() {
        let b1 = Body::new(2., 1., PURPLE.into());
        let bodies = vec![&b1];
        assert_eq!(check_for_syzygy(bodies), false);
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

#[derive(Component, Reflect)]
struct IsPlaying(bool);

fn handle_timestep_input(
    input: Res<ButtonInput<KeyCode>>,
    mut timestep: ResMut<Timestep>,
    mut query: Query<&mut IsPlaying>,
) {
    for mut play_status in &mut query {
        if input.just_pressed(KeyCode::Space) {
            play_status.0 = !play_status.0;
        }
    }

    // press "r" to reset timestamp
    if input.just_pressed(KeyCode::KeyR) {
        if timestep.0 < MAX_TIMESTEP {
            timestep.0 = 0.;
        }
    }

    // press right arrow or left arrow to adjust timestamp
    if input.just_pressed(KeyCode::ArrowRight) {
        if timestep.0 < MAX_TIMESTEP {
            timestep.0 = timestep.0.floor() + 1.;
        }
    }

    if input.just_pressed(KeyCode::ArrowLeft) {
        if timestep.0 > MIN_TIMESTEP {
            timestep.0 = timestep.0.ceil() - 1.;
        }
    }
}

// TODO: make this a user set (play, 2x, Fast (5x), pause) so they can slowmo the simulation and think about it? (nit: doesn't fit well with music. Instead maybe a BPM per system)
const TIMESTEP_SPEED: f64 = 0.005;

fn update_timestep(mut timestep: ResMut<Timestep>, query: Query<&IsPlaying>) {
    for play_status in &query {
        if play_status.0 {
            timestep.0 += TIMESTEP_SPEED;
        }
    }
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
