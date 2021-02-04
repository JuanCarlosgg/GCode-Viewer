use std::{collections::{HashMap}};

use bevy::{diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*};

use bevy_egui::{EguiContext, egui, EguiSettings};

#[cfg(not(target_arch = "wasm32"))] 
use nfd::Response;

use crate::gcode_plugin;

#[cfg(target_arch = "wasm32")]
use web_sys::{Document, Element, HtmlElement, Window, FileReader, InputEvent,DataTransfer};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, closure::Closure};

#[cfg(target_arch = "wasm32")]
use std::sync::Once;

#[cfg(target_arch = "wasm32")]
static START: Once = Once::new();

#[cfg(target_arch = "wasm32")]
use js_sys::Reflect;

#[derive(PartialEq, Clone, Copy)]
enum LayerOption {
    All,
    Range,
    Top
}

impl Default for LayerOption {
    fn default() -> LayerOption {
        LayerOption::All
    }
}

#[derive(Default)]
pub struct UiState {
    moves : bool,
    layers_option : LayerOption,
    min_layer : u32,
    max_layer : u32,
    top_layer : u32,

}


pub fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

pub fn ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut gcode_context: ResMut<gcode_plugin::GCodeContent>,
    mut ui_state: ResMut<UiState>,
    mut query: Query<(Entity, &mut Visible)>,
    diagnostics: Res<Diagnostics>,
) {
    let ctx = &mut egui_context.ctx;
    
    let mut load = false;
    //let mut show = false;
    #[cfg(target_arch = "wasm32")]
    let mut process = false;

    let last_min_layer = ui_state.min_layer;
    let last_max_layer = ui_state.max_layer;
    let last_top_layer = ui_state.top_layer;
    let last_layer_options = ui_state.layers_option;
    let last_show_moves = gcode_context.show_moves;

    egui::Window::new("Configuration").show(ctx, |ui| {

    ui.separator();

    egui::Frame::menu(ui.style()).show(ui, |ui|{   

        ui.heading("Show Layers");
        ui.separator();

        ui.radio_value(&mut ui_state.layers_option, LayerOption::All, "Show All");
        ui.separator();
        ui.radio_value(&mut ui_state.layers_option, LayerOption::Range, "Show Range");
        ui.separator();
        ui.radio_value(&mut ui_state.layers_option, LayerOption::Top, "Show From 0 to Top");


        match &mut ui_state.layers_option {
            LayerOption::All => (),
            LayerOption::Range => {
                ui.separator();
                ui.add(egui::Slider::u32(&mut ui_state.min_layer, 0..=gcode_context.level as u32).text("Min Layer"));
                ui.add(egui::Slider::u32(&mut ui_state.max_layer, 0..=gcode_context.level as u32).text("Max Layer"));
                
            },
            LayerOption::Top => {

                ui.separator();                
                ui.add(egui::Slider::u32(&mut ui_state.top_layer, 0..=gcode_context.level as u32).text("Top Layer"));
            },
        }
    });


    egui::Frame::menu(ui.style()).show(ui, |ui|{   

        ui.heading("Show Layers");
        ui.separator();                

        ui.add(egui::Checkbox::new(&mut gcode_context.show_moves, "Travel Moves"));

        //Color movimiento
        //Color extrusión.

    });

    egui::Frame::menu(ui.style()).show(ui, |ui|{   

        ui.heading("Load file");
        ui.separator();                

        ui.horizontal(|ui|{

            load = ui.button("Load File").on_hover_text("Load a new GCode file").clicked;
        
            #[cfg(target_arch = "wasm32")]{
            process = ui.button("Process File").on_hover_text("Process the new new file").clicked;
            }

            //show = ui.button("Show GCode").on_hover_text("Show the content of the GCode").clicked;
            //if show { ui_state.clicked = !ui_state.clicked; }

        });

    });
       

    match ui_state.layers_option {
        LayerOption::All => { 
            if last_layer_options != LayerOption::All || last_show_moves != ui_state.moves {
                let ent = gcode_context.entities.iter().map(|(idx, e)| {
                    let show = if idx.1 == 1 { true } else if idx.1 == 0 && gcode_context.show_moves { true } else { false };
                    (*e, show) 
                }).collect::<HashMap<Entity, bool>>();

                for (entity, mut draw) in  query.iter_mut() { 
                     if let Some(res) = ent.get(&entity) {
                        if *res { 
                        draw.is_visible = true;
                        } else {
                            draw.is_visible = false;
                        }
                    }
                }; 
            }
        },
        LayerOption::Range => {
            if last_layer_options != LayerOption::Range || last_min_layer != ui_state.min_layer || last_max_layer != ui_state.max_layer || last_show_moves != ui_state.moves {
                //println!("Mostramos todo entre {} y {} . size: {}", ui_state.min_layer, ui_state.max_layer,  gcode_context.entities.len());
                let ent = gcode_context.entities.iter().map(|(idx, e)| {
                    let in_range = idx.0 >=  ui_state.min_layer && idx.0 <= ui_state.max_layer;
                    let show = if idx.1 == 1 { true } else if idx.1 == 0 && gcode_context.show_moves { true } else { false };
                    (*e, in_range && show) 
                }).collect::<HashMap<Entity, bool>>();
                for (entity, mut draw) in  query.iter_mut() { 
                    if let Some(res) = ent.get(&entity) {
                        if *res { 
                        draw.is_visible = true;
                        } else {
                            draw.is_visible = false;
                        }
                    }
                }; 
            }
        },
        LayerOption::Top => {
            if last_layer_options !=  LayerOption::Top ||  ui_state.top_layer != last_top_layer || last_show_moves != ui_state.moves {
                //println!("Mostramos todo hasta {} size: {}", ui_state.top_layer,  gcode_context.entities.len());
                
                let ent = gcode_context.entities.iter().map(|(idx, e)| { 
                    let in_range = idx.0 <=  ui_state.top_layer;
                    let show = if idx.1 == 1 { true } else if idx.1 == 0 && gcode_context.show_moves { true } else { false };
                    (*e,  in_range && show ) 
                }).collect::<HashMap<Entity, bool>>();
                                
                for (entity, ref mut draw) in  query.iter_mut() { 
                    if let Some(res) = ent.get(&entity) {
                        if *res {  draw.is_visible = true;  } else { draw.is_visible = false; }
                    }
                };                 
            }
        },
        
    }

    if load {

        #[cfg(not(target_arch = "wasm32"))] {
            let result = nfd::open_file_dialog(Some("gco, gcode"), None).unwrap_or_else(|e| {
                panic!(e);
            });
        
            match result {
                Response::Okay(file_path) => {
                    println!("File path = {:?}", file_path);
                    let content_ =
                        std::fs::read_to_string(file_path).unwrap();

                    gcode_context.need_reload = true;
                    gcode_context.text = content_;
                },
                Response::OkayMultiple(_files) => println!("Select only one file"),
                Response::Cancel => println!("User canceled"),
            }
        }

        #[cfg(target_arch = "wasm32")] {
        
 
            let window = web_sys::window().expect("global window does not exists");
            let document = window.document().expect("expecting a document on window");
            let element = document.get_element_by_id("input_file").expect("Se esperaba que element existiese").dyn_into::<web_sys::HtmlElement>().expect("There are no input buttom");
                     
            START.call_once(|| {
                
            let closure = Closure::wrap(Box::new(  | event : web_sys::InputEvent| {
                let window = web_sys::window().expect("global window does not exists");
                let document = window.document().expect("expecting a document on window");
            
                let element = document.get_element_by_id("input_file").expect("Se esperaba que element existiese").dyn_into::<web_sys::HtmlElement>().expect("There are no input buttom");

                let filesinput = js_sys::Reflect::get(&element, &"files".into()).expect("Debería haber files").dyn_into::<web_sys::FileList>().expect("No es un FIleList");
                let f = filesinput.item(0).expect("Se esperaba que hubiese al menos uno");
                let file_reader = web_sys::FileReader::new().expect("Error al crear el filereader");
                file_reader.read_as_text(&f).expect("Error al leer a texto el filereader");

                let onload = Closure::wrap(Box::new(  | event: web_sys::Event | {

                  let file_reader: FileReader = event.target().expect("No hay filereader en el onload").dyn_into().expect("No  se puede convertir a Filereader");
                  let content = file_reader.result().expect("No hay result");
                  
                  
                  let window = web_sys::window().expect("global window does not exists");
                  let document = window.document().expect("expecting a document on window");
                  let element = document
                  .get_element_by_id("file_content")
                  .expect("String does not exist");
                  
                  
                  element.set_attribute("value", &content.as_string().expect("No se puede convertir a String"));
                  

                }) as  Box< dyn FnMut(_)>);
                
                file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();

            }) as Box<dyn FnMut(_)>);

                let _ = element.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref());
                closure.forget();

            });


                  
            element.click();
            
            //document.remove_child(&element);
        }
    }
    });


    #[cfg(target_arch = "wasm32")] if process {

        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let element = document
        .get_element_by_id("file_content")
        .expect("String does not exist");
        let file: String = element
        .get_attribute("value")
        .expect("Content is not a string");

        gcode_context.need_reload = true;
        gcode_context.text = file.clone();
        //ui_state.text = file;
        element.set_attribute("value", "");
    }

    /*if ui_state.clicked {

        egui::Window::new("GCode").show(ctx, |ui| {
            egui::ScrollArea::from_max_height(200.0)
            .id_source("Gcode_content")
            .show(ui, |ui| {
                ui.label(&ui_state.text[..]);
            });
        });
    }*/
    
    egui::Window::new("Info").show(ctx, |ui| {
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                ui.label(format!("FPS: {}",fps_avg));
            }
        }
        ui.separator();        

        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                ui.label(format!("Frame Time: {}",frame_time_avg));
            }
        }

    });
}