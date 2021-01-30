use std::collections::{HashMap, binary_heap};

use gcode::Mnemonic;

use bevy::prelude::*;

use crate::{poly};

#[cfg(target_arch = "wasm32")]
use web_sys::{Document, Element, HtmlElement, Window};


#[cfg(not(target_arch = "wasm32"))]
pub struct GCodeContent {
    pub text: String, 
    pub level: u32,
    updated: bool,
    pub need_reload : bool,
    pos: (f32, f32, f32),
    pub entities: HashMap<(u32, u32, u32), Entity>,
    pub show_moves : bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for GCodeContent {
    fn default() -> Self {

        //let content_ =
        //    std::fs::read_to_string("files/barco.gco" /*"files/simpletest3.gco"*/).unwrap();

        GCodeContent {
            text : "".to_owned(),
            level : 0,
            updated : false,
            need_reload : false,
            pos : (0.0, 0.0, 0.0),
            entities : HashMap::new(),
            show_moves : false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub struct GCodeContent {
    text: String,
    pos_iter: usize,
    update: bool,
    pos: (f32, f32, f32),
    pub plane: Option<Entity>,
    pub last_point: Option<Entity>,
    pub last_transform: Option<Transform>,
}

#[cfg(target_arch = "wasm32")]
impl Default for GCodeContent {
    fn default() -> Self {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let element = document
            .get_element_by_id("file_content")
            .expect("String does not exist");
        let file: String = element
            .get_attribute("value")
            .expect("Content is not a string");

        //web_sys::console::log_1(&file[..].into());

        document.remove_child(&element);

        GCodeContent {
            text: file,
            pos_iter: 0usize,
            update: true,
            pos: (0.0, 0.0, 0.0),
            plane: None,
            last_point: None,
            last_transform: None,
        }
    }
}


fn spawn_points_custom_mesh(
    commands: &mut Commands,
    gcode: &mut ResMut<GCodeContent>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    if !gcode.updated && gcode.text != "" {
        let mut vec_lines: Vec<PbrBundle> = vec![];

        let mut last_x = gcode.pos.0;
        let mut last_y = gcode.pos.1;
        let mut last_z = gcode.pos.2;

        println!("Start");

        let mut segments: Vec<Vec<String>> = vec![];
        gcode.text.lines().for_each(|line: &str| {
            if line.contains("F") || segments.is_empty() {
                segments.push(vec![line.to_owned()]);
            } else {
                segments.last_mut().unwrap().push(line.to_owned());
            }
        });

        println!("lines: {}", segments.len());

        let mut ctr = 0;
        for block in segments.iter() {
            let block_str: &str = &*block.join("\n");
            let mut iter = gcode::parse(block_str).peekable();
            let major_number = iter.peek().unwrap().major_number();

            let mut points = vec![];

            for (_idx, instruction) in iter.enumerate() {
                if instruction.arguments().is_empty() { continue; }

                match (
                    instruction.mnemonic(),
                    instruction.major_number(),
                    instruction.minor_number(),
                ) {
                    /*
                    //https://reprap.org/wiki/G-code/es#Buffered_G_Commands
                    G0 movimiento sin extrusión. Dibujar otro tipo de línea.
                    G1 movimiento con extrusión.
                    */
                    (Mnemonic::General, 0, _) => {
                        last_x = instruction.value_for('X').unwrap_or(last_x);
                        last_z = instruction.value_for('Y').unwrap_or(last_z);
                        let y_opt = instruction.value_for('Z');
                        last_y = y_opt.unwrap_or(last_y);

                        points.push((last_x, last_y, last_z));
                        if y_opt.is_some() {
                            gcode.level += 1;
                            ctr = 0;
                        }
                    }
                    (Mnemonic::General, 1, _) => {
                        last_x = instruction.value_for('X').unwrap_or(last_x);
                        last_z = instruction.value_for('Y').unwrap_or(last_z);
                        points.push((last_x, last_y, last_z));
                    }
                    _ => (),
                }

                
            }

            if major_number < 2 {
                let last_point = Vec3::new(gcode.pos.0, gcode.pos.1, gcode.pos.2);
                gcode.pos = (last_x, last_y, last_z);

                let (size, color) = match major_number {
                    0 => (0.12, Color::rgb(1.0, 1.0, 0.0)),
                    1 => (0.22, Color::rgb(1.0, 0.0, 0.0)),
                    _ => continue,
                };

                let vis = major_number == 1 || gcode.show_moves;
                /*vec_lines.push(*/ 
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(poly::Poly::new(size, size, points, last_point))),
                    material: materials.add(color.into()),
                    transform: Transform::from_translation(Vec3::new(-100.0, 0.0, -100.0)), //rotate(&(-new_point + last_point)),
                    visible : Visible {
                        is_visible: vis,
                        is_transparent: false,
                    },
                    ..Default::default()
                });
                 /* );*/

                let level = gcode.level;
                let c = commands.current_entity().unwrap();
                gcode.entities.insert((level, major_number, ctr), c);

                ctr +=1;
            }
        }

        

        println!("Total : {}", vec_lines.len());
        commands.spawn_batch(vec_lines);

        
        gcode.updated = true;
        println!("Finish");
    }
}


pub fn spawn_points_system(
    commands: &mut Commands,
    mut state: ResMut<GCodeContent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    /* 
    if !state.update {
       return; 
    }

    state.update = false;
    */

    if state.need_reload {

        println!("Need reload");
         for (_i, e) in state.entities.iter() {
            commands.despawn(*e);
        }

        state.updated = false;
        state.need_reload = false;
        state.pos = (0.0, 0.0, 0.0);
        state.level = 0;
        state.entities.clear();
    }

    //spawn_points_optim(commands, &mut state, &mut meshes, &mut materials);
    spawn_points_custom_mesh(commands, &mut state, &mut meshes, &mut materials);

    /* 
     
    let pr  = PbrBundle {
        mesh: meshes.add(Mesh::from(poly::Poly::new(0.5, 0.5, vec![(0.0, 5.0, 0.0), (0.0, 7.0, 2.0), (5.0, 7.0, 2.0), (6.0, 7.0, 3.0), (7.0, 7.0, 2.0)], Vec3::new(0.0, 0.0,0.0)))),
        material: materials.add(Color::rgb(1.0,0.0,0.0).into()),
        transform: Transform::default(),
        ..Default::default()
    };

    commands.spawn(pr);
    */

}
#[derive(Default)]
pub struct GCodePlugin;

impl Plugin for GCodePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state = GCodeContent {
            ..Default::default()
        };
        app.add_resource(state)
            .add_system(spawn_points_system.system());
    }
}
