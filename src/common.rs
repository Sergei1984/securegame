use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn get_world_coord_from_screen(
    screen_pos: Vec2,
    window_width: f32,
    window_height: f32,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    // check if the cursor is inside the window and get its position
    // get the size of the window
    let window_size = Vec2::new(window_width as f32, window_height as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    return world_pos;
}

pub fn get_cursor_pos(
    // need to get window dimensions
    wnds: &Res<Windows>,
    // query to get camera transform
    q_camera: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Vec2 {
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        return get_world_coord_from_screen(
            screen_pos,
            wnd.width(),
            wnd.height(),
            camera,
            camera_transform,
        );
    }

    return Vec2::new(0.0, 0.0);
}

pub fn cuboid_from(start: &Vec2, end: &Vec2, thikness: f32) -> (Vec2, f32, Collider) {
    let axis = *start - *end;
    let width = axis.length() + 3.0;
    let midpoint = Vec2::new((end.x + start.x) / 2.0, (end.y + start.y) / 2.0);
    let angle = -axis.angle_between(Vec2::new(1.0, 0.0));

    let collider = Collider::cuboid(width / 2.0, thikness);
    return (midpoint, angle, collider);
}
