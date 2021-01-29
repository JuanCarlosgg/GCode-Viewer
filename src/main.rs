//#![feature(destructuring_assignment)]
//#![feature(iter_advance_by)]

use bevy::prelude::*;

mod controls;
mod gcode_plugin;
//mod line;
mod poly;
mod cylinder;

use controls::*;

fn main() {
    let mut app = App::build();

    app .add_resource(Msaa { samples: 4 }) 
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .init_resource::<InputState>()
        .add_system(pan_orbit_camera.system())
        .add_plugin(gcode_plugin::GCodePlugin); //

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
// ver esto https://crates.io/crates/bevy_mod_picking
/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

     
    commands.spawn( LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 500.0, 4.0)),
        ..Default::default()
    });
    spawn_camera(commands);
}
