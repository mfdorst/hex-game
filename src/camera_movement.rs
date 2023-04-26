use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel},
        Input,
    },
    math::Vec3,
    prelude::*,
    render::camera::Camera,
};

const MAX_SCALE: f32 = 3.0;
const MIN_SCALE: f32 = 0.5;
const SCROLL_LINE_ZOOM_FACTOR: f32 = 0.25;
const SCROLL_PIXEL_ZOOM_FACTOR: f32 = 1.0;
const KEYBOARD_PAN_SPEED: f32 = 2.0;

pub struct CameraMovement;

impl Plugin for CameraMovement {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseDragState>()
            .add_system(keyboard_camera_movement)
            .add_system(mouse_drag_camera_movement)
            .add_system(mouse_wheel_camera_zoom);
    }
}

#[derive(Resource, Default)]
struct MouseDragState {
    is_dragging: bool,
    last_pos: Vec2,
}

fn mouse_drag_camera_movement(
    mut state: ResMut<MouseDragState>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera_xform_query: Query<(&mut Transform, &OrthographicProjection), With<Camera2d>>,
) {
    for event in &mut mouse_button_events {
        if event.button == MouseButton::Left {
            state.is_dragging = !state.is_dragging;
        }
    }

    if let Some(event) = cursor_moved_events.iter().last() {
        let delta = event.position - state.last_pos;
        state.last_pos = event.position;
        if !state.is_dragging {
            return;
        }
        for (mut xform, projection) in &mut camera_xform_query {
            let scale = projection.scale;
            xform.translation -= Vec3::new(scale * delta.x, scale * delta.y, 0.0);
        }
    }
}

fn mouse_wheel_camera_zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_xform_query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    if let Some(mut ortho) = camera_xform_query.iter_mut().last() {
        for event in &mut mouse_wheel_events {
            match event.unit {
                // Depending on the device, the scroll units could be different.
                MouseScrollUnit::Line => {
                    // Fixed step scrolling (Windows)
                    ortho.scale += event.y * SCROLL_LINE_ZOOM_FACTOR;
                }
                MouseScrollUnit::Pixel => {
                    // Smooth scrolling (MacOS)
                    ortho.scale += event.y * SCROLL_PIXEL_ZOOM_FACTOR;
                }
            }
            if ortho.scale < MIN_SCALE {
                ortho.scale = MIN_SCALE;
            }
            if ortho.scale > MAX_SCALE {
                ortho.scale = MAX_SCALE;
            }
        }
    }
}

fn keyboard_camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_xform_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, ortho) in &mut camera_xform_query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(KEYBOARD_PAN_SPEED, 0.0, 0.0) * ortho.scale;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(KEYBOARD_PAN_SPEED, 0.0, 0.0) * ortho.scale;
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, KEYBOARD_PAN_SPEED, 0.0) * ortho.scale;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, KEYBOARD_PAN_SPEED, 0.0) * ortho.scale;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
