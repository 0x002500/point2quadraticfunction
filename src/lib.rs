use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn calculateA(point1: Point, point2: Point, point3: Point) -> f64 {
    let x1: f64 = point1.x;
    let x2: f64 = point2.x;
    let x3: f64 = point3.x;
    let y1: f64 = point1.y;
    let y2: f64 = point2.y;
    let y3: f64 = point3.y;
    (y1 * (x2 - x3) + y2 * (x3 - x1) + y3 * (x1 - x2))
        / (x1 * x1 * (x2 - x3) + x2 * x2 * (x3 - x1) + x3 * x3 * (x1 - x2))
}

#[wasm_bindgen]
pub fn point2quadraticfunction(point1: Point, point2: Point, point3: Point) {}
