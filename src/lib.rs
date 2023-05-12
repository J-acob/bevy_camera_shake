use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::camera,
    transform,
};
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};
use rand::Rng;

/// A camera shaker component
#[derive(Component, Default)]
pub struct CameraShaker2d {
    pub shake: f32,
    pub max_angle: f32,
    pub max_offset: f32,
    pub enabled: bool,
}

impl CameraShaker2d {
    pub fn new(shake: f32, max_angle: f32, max_offset: f32) -> CameraShaker2d {
        CameraShaker2d {
            shake,
            max_angle,
            max_offset,
            enabled: false,
            ..Default::default()
        }
    }
}

/// A component representing how much "trauma" an entity has
#[derive(Component, Default)]
pub struct Trauma {
    /// The current value of the trauma, clamped to [0, 1];
    pub value: f32,
    pub decay: f32,
}

impl Trauma {
    pub fn new(value: f32, decay: f32) -> Trauma {
        Trauma { value, decay }
    }
}

/*
impl CameraShaker2d {
    pub fn shake(&mut self, camera_transform: &mut Transform) {
        if self.enabled && self.trauma > 0. {
            // Generate a random seed to use.
            let seed = rand::thread_rng().gen_range(u32::MIN..u32::MAX);

            let angle = self.max_angle as f64
                * self.settings.shake
                * perlin_2d(
                    [seed as f64, self.settings.time as f64],
                    &PermutationTable::new(seed),
                );

            let offset_x = self.max_offset as f64
                * self.settings.shake
                * perlin_2d(
                    [seed as f64, self.settings.time as f64],
                    &PermutationTable::new(seed + 1),
                );

            let offset_y = self.max_offset as f64
                * self.settings.shake
                * perlin_2d(
                    [seed as f64, self.settings.time as f64],
                    &PermutationTable::new(seed + 2),
                );

            camera_transform.rotate_z(angle as f32);
            camera_transform.translation =
                camera_transform.translation + vec2(offset_x as f32, offset_y as f32).extend(0.);
        }
    }

    /// Updates the trauma value
    pub fn apply_trauma(&mut self, camera_transform: &mut Transform) {
        // Reduce trauma
        self.trauma -= self.settings.time * self.settings.trauma_decay;
        self.trauma = self.trauma.clamp(0., 1.);

        if self.trauma <= 0. {
            self.enabled = false;
            /*
            camera_transform.translation =
                camera_transform.translation.lerp(vec3(0., 0., 0.), 0.25);
            */
            camera_transform.rotation = camera_transform.rotation.lerp(Quat::IDENTITY, 0.25);
        }
    }
}

fn update_trauma(mut query: Query<(&mut CameraShaker2d, &mut Transform)>) {
    for (mut camera_shaker, mut camera_transform) in query.iter_mut() {
        camera_shaker.apply_trauma(&mut camera_transform);
    }
}
*/

/// Shakes the cameras!
fn shake_cameras(
    pure_camera: Query<(&Transform, &Camera2d, Without<CameraShaker2d>)>,
    mut shaky_camera: Query<(&mut Transform, &Trauma, &mut CameraShaker2d)>,
    time: Res<Time>,
) {
    let (pure_transform, _, _) = pure_camera.single();

    // Consider putting this into an impl and then doing something else to allow more abstraction of how to apply cameras
    for (mut shake_transform, trauma, mut camera_shaker) in shaky_camera.iter_mut() {
        if trauma.value > 0. {
            camera_shaker.enabled = true;
            // Generate a seed
            let seed = rand::thread_rng().gen_range(u32::MIN..u32::MAX);

            // Calculate shaky values
            let angle = camera_shaker.max_angle as f64
                * camera_shaker.shake as f64
                * perlin_2d(
                    [seed as f64, time.delta_seconds_f64() as f64],
                    &PermutationTable::new(seed),
                );

            let offset_x = camera_shaker.max_offset as f64
                * camera_shaker.shake as f64
                * perlin_2d(
                    [seed as f64, time.delta_seconds_f64()],
                    &PermutationTable::new(seed + 1),
                );

            let offset_y = camera_shaker.max_offset as f64
                * camera_shaker.shake as f64
                * perlin_2d(
                    [seed as f64, time.delta_seconds_f64()],
                    &PermutationTable::new(seed + 2),
                );

            // Update the shake transform
            shake_transform.translation =
                pure_transform.translation + vec3(offset_x as f32, offset_y as f32, 0.);
            shake_transform.rotation =
                pure_transform.rotation + Quat::from_axis_angle(Vec3::Z, angle as f32);
        } else {
            // If we have no more trauma at all...
            if trauma.value == 0. {
                camera_shaker.enabled = false;
                shake_transform.translation = pure_transform.translation;
                shake_transform.rotation = pure_transform.rotation;
            } else {
                if camera_shaker.enabled == true {
                    // Lerp back towards the pure camera
                    shake_transform.translation = shake_transform
                        .translation
                        .lerp(pure_transform.translation, 0.25);
                    shake_transform.rotation =
                        shake_transform.rotation.lerp(pure_transform.rotation, 0.25);
                }
            }
        }
    }
}

/// Decreases trauma over time
fn trauma_decreases(mut query: Query<&mut Trauma>, time: Res<Time>) {
    for mut trauma in query.iter_mut() {
        if trauma.value > 0. {
            trauma.value = (trauma.value - trauma.decay * time.delta_seconds()).clamp(0., 1.);
        }
    }
}

/// Plugin for enabling camera shaking
pub struct CameraShakerPlugin;

impl Plugin for CameraShakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(trauma_decreases.before(shake_cameras))
            .add_system(shake_cameras);
    }
}

pub enum CameraShakePreset {
    Earthquake,
    Punch,
    Custom,
}

pub mod prelude {
    pub use crate::{CameraShaker2d, CameraShakerPlugin};
}

/// Camera shaker bundle
#[derive(Bundle)]
pub struct CameraShaker2dBundle {
    camera: Camera2dBundle,
    trauma: Trauma,
    camera_shaker2d: CameraShaker2d,
}
