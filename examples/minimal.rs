use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_camera_shake::{prelude::*, CameraShaker2d, Trauma};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraShakerPlugin)
        .add_startup_system(setup)
        .add_system(shake)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // The shaky camera...!
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            ..Default::default()
        },
        Trauma::new(0., 0.8),
        CameraShaker2d::new(0.5, PI / 6., 50.),
    ));

    // Another camera... the "pure" camera!
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0,
            ..Default::default()
        },
        ..Default::default()
    });

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0., -50., 0.)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::ORANGE)),
        transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
        ..default()
    });
}

fn shake(
    mut query: Query<(&mut Trauma, &CameraShaker2d)>,
    //time: Res<Time>,
    input: Res<Input<MouseButton>>,
) {
    for (mut trauma, camera_shaker) in query.iter_mut() {
        //println!("Trauma: {:?}", trauma.value);
        if input.just_pressed(MouseButton::Left) {
            trauma.value += 0.3;
        }
    }
}
