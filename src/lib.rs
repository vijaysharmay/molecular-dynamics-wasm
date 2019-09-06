#![allow(dead_code)]
#![allow(unused_variables)]

extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

extern crate console_error_panic_hook;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[derive(Serialize, Deserialize, Clone)]
pub struct Position{ x: f64, y: f64 }

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    id: String,
    size: f64,
    position: Position
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Link{
    source: String,
    target: String,
    value: f64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph{
    nodes: Vec<Node>,
    links: Vec<Link>
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render(graph: &JsValue) {
    let graph: Graph = graph.into_serde().unwrap();
    draw(&graph);
}

#[wasm_bindgen]
pub fn get_clicked_node(position: &JsValue, graph: &JsValue) -> String {
    let graph: Graph = graph.into_serde().unwrap();
    let position: Position = position.into_serde().unwrap();
    let filtered_content = &graph.nodes.iter().filter(|&node| {
        position.x > (node.position.x - node.size) &&
        position.x < (node.position.x + node.size) &&
        position.y > (node.position.y - node.size) &&
        position.y < (node.position.y + node.size)
    }).into_iter().collect::<Vec<&Node>>();
    let result: String = match filtered_content.first() {
        Some(node) => String::from(&node.id),
        None => String::from("Node not found")
    };
    return result;
}

#[wasm_bindgen]
pub fn mouse_move(position: &JsValue, graph: &JsValue, id: String) -> JsValue{
    let graph: Graph = graph.into_serde().unwrap();
    let position: Position = position.into_serde().unwrap();

    let touched= &graph.nodes.iter().find(|&x| x.id == id).unwrap();

    let updated_nodes: Vec<Node> = graph.nodes.iter().map(|node: &Node| {
        let vx: f64 = touched.position.x - position.x;
        let vy: f64 = touched.position.y - position.y;
        let position: Position = if node.id == id{
            Position { x: position.x, y: position.y }
        }else{
            Position { x: node.position.x - vx, y: node.position.y - vy }
        };
        return Node { id: node.id.clone(), position, size: node.size };
    }).into_iter().collect::<Vec<Node>>();

    let updated_links: Vec<Link> = graph.links;

    let updated_graph = Graph { nodes: updated_nodes, links: updated_links };

    draw(&updated_graph);

    return JsValue::from_serde(&updated_graph).unwrap();
}

fn get_canvas_context() -> (CanvasRenderingContext2d, f64, f64){
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let width: f64 = canvas.width() as f64;
    let height: f64 = canvas.height() as f64;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    return (context, width, height);
}

fn draw(graph: &Graph) {
    let (context, width, height) = get_canvas_context();
    context.clear_rect(0.0, 0.0, width, height);

    for Node { id, size, position } in &graph.nodes{
        context.begin_path();
        context.arc(
            position.x,
            position.y,
            size.clone(),
            0.0,
            f64::consts::PI * 2.0
        ).unwrap();
        context.stroke();
        context.fill();
    }

    for Link { source, target, value } in &graph.links {
        let source_node = &graph.nodes.iter().find(|&x| x.id == source.clone()).unwrap();
        let target_node = &graph.nodes.iter().find(|&x| x.id == target.clone()).unwrap();
        context.begin_path();
        context.move_to(source_node.position.x, source_node.position.y);
        context.line_to(target_node.position.x, target_node.position.y);
        context.stroke();
    }
}



