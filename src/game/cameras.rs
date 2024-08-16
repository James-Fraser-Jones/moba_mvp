pub mod orbit_camera;
pub mod overlay_camera;

use bevy::color::palettes::css;
use bevy::prelude::*;

pub struct CamerasPlugin;
impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            orbit_camera::OrbitCameraPlugin,
            overlay_camera::OverlayCameraPlugin,
        ));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::Srgba(css::FOREST_GREEN);
}

fn update() {}

//=================================================================================
//UTILITY FUNCTIONS BELOW MUST BE CALLED AFTER TRANSFORM PROPAGATION IN PostUpdate:
//https://bevy-cheatbook.github.io/fundamentals/transforms.html#transform-propagation
//else, global transform for given camera will be 1 frame behind

//logical pixels, top-left (0,0), to Vec2 representing intersection point with horizontal plane of height, in world space
pub fn pixel_to_horizontal_plane(
    pixel: Vec2,
    height: f32,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let pixel_ray = camera.viewport_to_world(camera_transform, pixel).unwrap();
    let intersection_distance =
        pixel_ray.intersect_plane(Vec3::Z * height, InfinitePlane3d::new(Vec3::Z))?;
    let intersection_point = pixel_ray.get_point(intersection_distance);
    Some(intersection_point.truncate())
}

//inverse
pub fn position_to_pixel(
    position: Vec3,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    camera.world_to_viewport(camera_transform, position)
}
//=================================================================================
