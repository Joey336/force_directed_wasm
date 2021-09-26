extern crate kiss3d;
extern crate nalgebra as na;
extern crate stdweb;
extern crate rand;
mod dot_reader;
mod node;
mod spring;
mod common_funcs;
use rand::prelude::*;
use kiss3d::nalgebra::{Point3};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use kiss3d::light::Light;
use kiss3d::window::{State, Window};

extern crate web_sys;



struct AppState {
    spring_vector: Vec<spring::Spring>,
    nodes_hashmap: std::collections::HashMap<std::string::String, node::Node>,
    edges_data: std::collections::HashMap<std::string::String, (std::string::String, std::string::String, f32)>
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
         //iterate through all springs to apply acceleration of nodes
         for spring in self.spring_vector.iter(){
            spring.move_nodes(&mut self.nodes_hashmap);
            }

        //update position on canvas of all nodes (after all spring forces applied, each node will have its final acceleration for this frame)
        for (_key, value) in &mut self.nodes_hashmap{
            value.update_position();
        }

        //draw visible edges between nodes after updated position on canvas
        draw_edges(&self.nodes_hashmap, &self.edges_data, window)
    }
}





#[wasm_bindgen()]
pub fn main_js(message: &str){

    let mut window = Window::new("Kiss3d: wasm example");
    window.set_background_color(0.5, 0.5, 0.5);
    window.set_light(Light::StickToCamera);
    
    //Do not call read_dot when webpage is first loaded(no files input)
    if message != "0.0"{
        let (nodes_data, edges_data) = dot_reader::read_dot(message);
        let nodes_hashmap = create_nodes(nodes_data, &mut window);
        let springs_vector = create_springs(&edges_data);
        let state = AppState{
            spring_vector: springs_vector,
            nodes_hashmap: nodes_hashmap,
            edges_data: edges_data
        };
        window.set_framerate_limit(Some(10));
        window.render_loop(state);
    }
   
}

//draws edges between appropriate nodes
fn draw_edges(nodes_hashmap: &HashMap<String, node::Node>, edges_data: &HashMap<String, (String, String, f32)>, window: &mut Window){
    for (_key, data) in edges_data{
        let node1 = data.0.clone();
        let node2 = data.1.clone();

        let node_ob1 = nodes_hashmap.get(&node1).unwrap();
        let node_ob2 = nodes_hashmap.get(&node2).unwrap();

        let point1 = node::Node::get_point(&node_ob1);
        let point2 = node::Node::get_point(&node_ob2);

        window.draw_line(&point1, &point2, &Point3::new(0., 1.0,1.0));
        window.set_line_width(1.);
    }
}


//creates spring for each edge in graph
fn create_springs(edges_data: &HashMap<String, (String, String, f32)>) -> Vec<spring::Spring>{
    //Responsible for spread of graph (ideal multiplier different depending on data file selected)
    let spring_multiplier = 3.;

    let mut return_vec: Vec<spring::Spring> = Vec::new();

    for (_key, data) in edges_data{
        //resting length scaled by spring_multiplier
        let new_spring = spring::Spring::new(data.2 * spring_multiplier, data.0.clone(), data.1.clone());
        return_vec.push(new_spring);
    }

    return_vec
}

//given node_data from dot file, returns hashmap of Node objects
fn create_nodes(node_data: HashMap<String, dot_reader::NodeData>, window: &mut Window) -> HashMap<String, node::Node>{
    //initialize x, y, z at random positions
    let mut return_hashmap = HashMap::new();    
    
    
    for (_key, value) in &node_data{
        //randomly generate initial x,y,z position of node
        let mut rng = thread_rng();
        let x = rng.gen_range(0., 100.0);
        let y = rng.gen_range(0., 100.0);
        let z = rng.gen_range(0., 100.0);
        
        let mut sphere = window.add_sphere(1.);

        //RBG scaled to float between 0.0 - 1.0 (kiss3d RGB parameters are 0.0 - 1.0)
        sphere.set_color((1./255.)*value.color.red, (1./255.)*value.color.green, (1./255.)*value.color.blue);

        let new_node = node::Node::new(na::Vector3::new(x, y, z),value.cardinality as f32, sphere);
        return_hashmap.insert(value.name.clone(), new_node);
       
    }

    return_hashmap
}