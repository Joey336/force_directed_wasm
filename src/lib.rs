extern crate kiss3d;
extern crate nalgebra as na;
extern crate stdweb;
mod dot_reader;

use wasm_bindgen::prelude::*;
use kiss3d::light::Light;
use kiss3d::window::{State, Window};

extern crate web_sys;



struct AppState {
    test: i32
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        //        for event in window.conrod_ui().widget_input(self.ids.button).events() {
        //            console!(log, format!("Found event: {:?}", event))
        //        }

        //let mut ui = window.conrod_ui_mut().set_widgets();
        //gui(&mut ui, &self.ids, &mut self.app)
    }
}





#[wasm_bindgen()]
pub fn main_js(message: &str){


    let mut window = Window::new("Kiss3d: wasm example");
    window.set_background_color(0.5, 0.5, 0.5);
    if message != "0.0"{
        let (node_data, edge_data) = dot_reader::read_dot(message);
        let mut d = window.add_sphere(50.);
        log!("{:?} \n\n\n{:?}",node_data, edge_data);
    }

    let mut c = window.add_sphere(0.5);
    c.set_color(1.0, 5.0, 0.0);

    window.set_light(Light::StickToCamera);



    let state = AppState { test: 50 };

    window.render_loop(state);
   
}

