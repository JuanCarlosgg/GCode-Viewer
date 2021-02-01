use std::collections::{HashMap};

use gcode::Mnemonic;

use bevy::prelude::*;

use crate::{poly};


pub struct GCodeContent {
    pub text: String, 
    pub level: u32,
    updated: bool,
    pub need_reload : bool,
    pos: (f32, f32, f32),
    pub entities: HashMap<(u32, u32, u32), Entity>,
    pub show_moves : bool,
}

impl Default for GCodeContent {
    fn default() -> Self {

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


#[derive(Clone, Copy)]
pub enum PolyElement {
    Point(f32, f32, f32),
    Change,
}

fn spawn_points_custom_mesh(
    commands: &mut Commands,
    gcode: &mut ResMut<GCodeContent>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    if !gcode.updated && gcode.text != "" {
        //let mut vec_lines: Vec<PbrBundle> = vec![];

        
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

        // Dibujar un solo polígono por cada nivel.
        let mut points_extrusion : Vec<PolyElement> = vec![];
        let mut points_move : Vec<PolyElement> = vec![];
        let mut last_points_extrusion : Vec<(f32, f32, f32)> = vec![];
        let mut last_points_move : Vec<(f32, f32, f32)> = vec![];

        for block in segments.iter() {
            let block_str: &str = &*block.join("\n");
            let mut iter = gcode::parse(block_str).peekable();
            
            let mut draw = false;
            //let last_major_number : Option<u32> = None;

            match iter.peek().unwrap().major_number() { 
                1 => { 
                    points_extrusion.push(PolyElement::Change);
                    last_points_extrusion.push((gcode.pos.0, gcode.pos.1, gcode.pos.2)) 
                },
                0 => { 
                    points_move.push(PolyElement::Change);
                    last_points_move.push((gcode.pos.0, gcode.pos.1, gcode.pos.2)) 
                },
                _=>()
            }
            

            for (_idx, instruction) in iter.enumerate() {
                //Parece que la función parse mete instrucciones vacías.
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
                    El resto dan igual de momento.
                    */
                    (Mnemonic::General, 0, _) => {
                        last_x = instruction.value_for('X').unwrap_or(last_x);
                        last_z = instruction.value_for('Y').unwrap_or(last_z);
                        let y_opt = instruction.value_for('Z');
                        last_y = y_opt.unwrap_or(last_y);

                        points_move.push(PolyElement::Point(last_x, last_y, last_z));
                        //last_major_number = Some(0);

                        if y_opt.is_some() {
                            gcode.level += 1;
                            ctr = 0;
                            draw = true;
                        }
                    }
                    (Mnemonic::General, 1, _) => {
                        last_x = instruction.value_for('X').unwrap_or(last_x);
                        last_z = instruction.value_for('Y').unwrap_or(last_z);
                        points_extrusion.push(PolyElement::Point(last_x, last_y, last_z));
                        //last_major_number = Some(1);
                    }
                    _ => continue,
                }

                
            }

            gcode.pos = (last_x, last_y, last_z);
            ctr +=1;

            if draw {
                //let last_point = Vec3::new(gcode.pos.0, gcode.pos.1, gcode.pos.2);

                let (size_move, color_move) = (0.12, Color::rgb(1.0, 1.0, 0.0));
                let (size_extr, color_extr) =  (0.22, Color::rgb(1.0, 0.0, 0.0));
                
                let level = gcode.level;

                //let vis = major_number == 1 || gcode.show_moves;
                /*vec_lines.push(*/ 
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(poly::Poly::new(size_extr, size_extr, &points_extrusion, &last_points_extrusion))),
                    material: materials.add(color_extr.into()),
                    transform: Transform::from_translation(Vec3::new(-100.0, 0.0, -100.0)), //rotate(&(-new_point + last_point)),
                    visible : Visible {
                        is_visible: true,
                        is_transparent: false,
                    },
                    ..Default::default()
                });
                let c = commands.current_entity().unwrap();
                gcode.entities.insert((level, 1, ctr), c);

                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(poly::Poly::new(size_move, size_move, &points_move, &last_points_move))),
                    material: materials.add(color_move.into()),
                    transform: Transform::from_translation(Vec3::new(-100.0, 0.0, -100.0)), //rotate(&(-new_point + last_point)),
                    visible : Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                });
                let c = commands.current_entity().unwrap();
                gcode.entities.insert((level, 0, ctr), c);


                points_move.clear();
                last_points_move.clear();
                points_extrusion.clear();
                last_points_extrusion.clear();
                
            }
        }

        

        //println!("Total : {}", vec_lines.len());
        //commands.spawn_batch(vec_lines);

        
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
        //app.insert_resource(state)
        app.add_resource(state)
            .add_system(spawn_points_system.system());
    }
}
