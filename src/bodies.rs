#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{asset::AssetMetaCheck, color::palettes::css::PURPLE};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

const X_EXTENT: f32 = 900.;

pub fn bodies_plugin(app: &mut App) {
    app.add_systems(Startup, setup_shapes);
}

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

    for distance_from_central_body in [200., 500.] {
        // spawn sat
        let satellite = Satellite::new(distance_from_central_body);
        let shape = Mesh2dHandle(meshes.add(Circle { radius: 50. }));
        let hue = 180.; // 0 - 360
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

    // // commands.spawn(Camera2dBundle::default());

    // let shapes = [
    //     Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
    //     Mesh2dHandle(meshes.add(Circle { radius: 25.0 })),
    //     Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
    //     // Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
    //     // Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
    //     Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
    //     Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
    //     Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
    //     Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
    //     Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
    //     Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
    //     Mesh2dHandle(meshes.add(Triangle2d::new(
    //         Vec2::Y * 50.0,
    //         Vec2::new(-50.0, -50.0),
    //         Vec2::new(50.0, -50.0),
    //     ))),
    // ];
    // let num_shapes = shapes.len();

    // for (i, shape) in shapes.into_iter().enumerate() {
    //     // Distribute colors evenly across the rainbow.
    //     let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

    //     commands.spawn(MaterialMesh2dBundle {
    //         mesh: shape,
    //         material: materials.add(color),
    //         transform: Transform::from_xyz(
    //             // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
    //             -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //             0.0,
    //             0.0,
    //         ),
    //         ..default()
    //     });
    // }

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

// fn move_shapes() {}

// /// the last frame.
// fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
//     for (mut logo, mut transform) in &mut sprite_position {
//         match *logo {
//             Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
//             Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
//         }

//         if transform.translation.y > 200. {
//             *logo = Direction::Down;
//         } else if transform.translation.y < -200. {
//             *logo = Direction::Up;
//         }
//     }
// }

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
