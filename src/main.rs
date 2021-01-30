

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
mod controls;
mod gcode_plugin;
mod poly;
mod ui;

use controls::*;




fn main() {
    let mut app = App::build();

    app //.add_resource(Msaa { samples: 4 }) 
        .add_startup_system(setup.system())
        .add_startup_system(startup_window.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EguiPlugin)
        .add_system(ui::ui_system.system())
        .init_resource::<ui::UiState>()
        .init_resource::<InputState>()
        .add_system(pan_orbit_camera.system())
        .add_plugin(gcode_plugin::GCodePlugin)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
        

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
// ver esto https://crates.io/crates/bevy_mod_picking

//Resize windoe
fn startup_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_maximized(true);
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 250.0 })),
        material: materials.add(Color::rgba(0.2, 0.2, 0.2, 0.7).into()),
        transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 252.0 })),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.3).into()),
        ..Default::default()
    });
     
    commands.spawn( LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1000.0, 0.0)),
        ..Default::default()
    });

    spawn_camera(commands);
}
