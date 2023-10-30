use bevy::{math::vec3, prelude::*, transform, window::PrimaryWindow};

#[derive(Resource)]
pub struct CameraView {
    vec3: Vec3,
}
#[derive(Resource)]
pub struct CameraScale {
    vec3: Vec3,
}
// .init_resource::<CameraView>()
impl Default for CameraView {
    fn default() -> Self {
        CameraView {
            vec3: Vec3 {
                x: 640.0,
                y: 360.0,
                z: 0.0,
            },
        }
    }
}
impl Default for CameraScale {
    fn default() -> Self {
        CameraScale {
            vec3: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_view: ResMut<CameraView>,
) {
    let window = window_query.get_single().unwrap();
    // To scale the Camera view
    camera_view.vec3.y = camera_view.vec3.x * (window.height() / window.width());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(camera_view.vec3.x, camera_view.vec3.y, 0.0),
        ..default()
    });
}

/// # How to move a camera
/// The Query gets the camera
/// The query is an interator, because there could be multible cameras and the forloop extracts the camera out of the iterator
/// Then we grow the translation by 1 every frame and thus move the camera to the right
pub fn move_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    for mut transform in &mut camera_query {
        transform.translation.x += 1.0;
        transform.translation.y = 0.0;
    }
}

/// # How to zoom a camera
/// The Query gets the camera
/// The ResMut, gets a global Scale Ressource, so multible things can change the scale
/// we increase the ResMut
/// Extract the camera out of the iterator query
/// Place the ScaleRes into the cameras scale
pub fn zoom_out_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut camera_scale: ResMut<CameraScale>,
) {
    camera_scale.vec3.x += 1.0;
    camera_scale.vec3.y += 1.0;
    for mut transform in &mut camera_query {
        *transform = Transform::from_scale(camera_scale.vec3);
        println!("{:?}", transform);
    }
}
