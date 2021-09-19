#[macro_use]
extern crate kiss3d;
extern crate nalgebra as na;
extern crate stdweb;
mod dot_reader;

use std::fs::File;
use kiss3d::conrod::color::{Color, Colorable};
use kiss3d::conrod::position::{Positionable, Sizeable};
use kiss3d::conrod::widget::{button::Style, Button, Text, Widget};
use kiss3d::conrod::Labelable;
use wasm_bindgen::prelude::*;
use stdweb::{js};

use kiss3d::conrod;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use na::{UnitQuaternion, Vector3};

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



// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}



#[wasm_bindgen(start)]
pub fn main_js(){
    
    let mut window = Window::new("Kiss3d: wasm example");
    window.set_background_color(0.5, 0.5, 0.5);
    let mut c = window.add_sphere(0.5);
    c.set_color(1.0, 5.0, 0.0);

    window.set_light(Light::StickToCamera);



    let state = AppState { test: 50 };

    window.render_loop(state);
   
}
