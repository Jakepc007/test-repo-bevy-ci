use bevy::prelude::*;

pub struct Cursor(pub Vec2);

fn update_cursor(
    mut cursor: ResMut<Cursor>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<OrthographicProjection>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = windows.get_primary().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        cursor.0 = world_pos;
    };
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor(Vec2::new(0., 0.)))
            .add_system(update_cursor);
    }
}
