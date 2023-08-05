use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{window, MouseEvent, console};


#[derive(Clone)]
struct node {
    x: f64,
    y: f64,

}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let mut node_list: Vec<Vec<node>> = vec![];
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();


    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    context.begin_path();


    for i in 1..100 {
        let mut temp = vec![];
        for j in 1..100 {
            context.move_to(12.0 * i as f64, 12.0 * j as f64);
            context
                .arc(12.0 * i as f64, 12.0 * j as f64, 4.0, 0.0, 360.0)
                .unwrap();
            temp.push(node { x: 12.0 * i as f64, y: 12.0 * j as f64 });
        }
        node_list.push(temp);
    }


    context.stroke();
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
        console::log_1(&"mousedown".into());
        let x: usize = ((event.offset_x() as f64 / 12.0).round() - 1.0) as usize;
        let y: usize = ((event.offset_y() as f64 / 12.0).round() - 1.0) as usize;
        // console::log_1(&**js_sys::Number::from(x as f64));
        // console::log_1(&**js_sys::Number::from(y as f64));
        let c_node: node = node_list.get(x).unwrap().get(y).unwrap().clone();
        console::log_1(&**js_sys::Number::from(c_node.x));
        console::log_1(&**js_sys::Number::from(c_node.y));
        context.begin_path();
        context.move_to(c_node.x, c_node.y);
        context
            .arc(c_node.x, c_node.y, 4.0, 0.0, 360.0)
            .unwrap();
        context.fill();
    });
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

