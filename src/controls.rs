use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub struct PanOrbitCamera {
    pub focus: Vec3,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::zero(),
        }
    }
}


/// Hold readers for events
#[derive(Default)]
pub struct InputState {
    pub reader_motion: EventReader<MouseMotion>,
    pub reader_scroll: EventReader<MouseWheel>,
    pub mouse_position: EventReader<CursorMoved>,
    pub cursor_pos: Vec2,
}

/// Pan the camera with LHold or scrollwheel, orbit with rclick.
pub fn pan_orbit_camera(
    _time: Res<Time>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
   /*  mut reader_motion: EventReader<MouseMotion>,
    mut reader_scroll: EventReader<MouseWheel>,
    mouse_position: EventReader<CursorMoved>,*/
    ev_motion: Res<Events<MouseMotion>>,
    mousebtn: Res<Input<MouseButton>>,
    ev_scroll: Res<Events<MouseWheel>>,
    //ev_cursor: Res<Events<CursorMoved>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut rotation_move = Vec2::zero();
    let mut scroll = 0.0;

    let window = windows.get_primary().unwrap();
 
  
    //No funciona con wasm.
    if mousebtn.pressed(MouseButton::Right) {
         rotation_move = state.reader_motion
                .iter(&ev_motion)
                .fold(rotation_move, |acc, x| acc + x.delta);
        } 
    
    scroll = state.reader_scroll
        .iter(&ev_scroll)
        .fold(scroll, |acc, x| acc + x.y);

   /*  for ev in state.mouse_position.iter(&ev_cursor) {
        state.cursor_pos = ev.position;
    }*/


    let window_w = window.width();
    let window_h = window.height();

    // Either pan+scroll or arcball. We don't do both at once.
    for (camera, mut trans) in query.iter_mut() {

        if rotation_move.length_squared() > 0.0 {
            // Link virtual sphere rotation relative to window to make it feel nicer
            let delta_x = rotation_move.x / window_w * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window_h * std::f32::consts::PI;

            let delta_yaw = Quat::from_rotation_y(delta_x);
            let delta_pitch = Quat::from_rotation_x(delta_y);

            trans.translation =
                delta_yaw * delta_pitch * (trans.translation - camera.focus) + camera.focus;

            let look = Mat4::face_toward(trans.translation, camera.focus, Vec3::new(0.0, 1.0, 0.0));
            trans.rotation = look.to_scale_rotation_translation().1;
        }

        if scroll != 0.0 && (trans.scale.x + 0.05 * -scroll) > 0.0 {
            trans.scale.y += 0.05 * -scroll;
            trans.scale.x += 0.05 * -scroll;
        }
        //println!("{:?}", trans);

    }

}

// Spawn a camera like this:

pub fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn((PanOrbitCamera::default(),))
        .with_bundle(/*PerspectiveCameraBundle*/  Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(-336.65918, 333.0896, -447.4693), 
                rotation: Quat::from_xyzw(0.084021315, 0.9145102, 0.2514309, -0.30560428), 
                scale: Vec3::new(0.7487499, 0.7487499, 1.0)
            },
            ..Default::default()
        })  /* .with_children(|parent| { 
            parent.spawn( LightBundle {
                light : Light {
                    depth : 0.0..360.0,
                    fov : 360.0,
                    ..Default::default()
                },
                transform: Transform::from_rotation(Quat::from_rotation_y(f32::to_radians(90.0))),
                ..Default::default()
            });
        })*/;
}
