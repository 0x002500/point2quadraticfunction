use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn calculateA(point1: Point, point2: Point, point3: Point) -> f64 {
    (point1.y * (point2.x - point3.x)
        + point2.y * (point3.x - point1.x)
        + point3.y * (point1.x - point2.x))
        / (point1.x * point1.x * (point2.x - point3.x)
            + point2.x * point2.x * (point3.x - point1.x)
            + point3.x * point3.x * (point1.x - point2.x))
}

fn calculateB(point1: Point, point2: Point, point3: Point) -> f64 {
todo!()
}

#[wasm_bindgen]
pub fn point2quadraticfunction(point1: Point, point2: Point, point3: Point) {}
