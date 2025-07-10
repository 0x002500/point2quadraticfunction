use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub fn calculate_a(point1: Point, point2: Point, point3: Point) -> f64 {
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
pub fn calculate_b(point1: Point, point2: Point, point3: Point) -> f64 {
    let x1: f64 = point1.x;
    let x2: f64 = point2.x;
    let y1: f64 = point1.y;
    let y2: f64 = point2.y;
    let a: f64 = calculate_a(point1, point2, point3);
    (y1 - y2) / (x1 - x2) - a * (x1 + x2)
}

#[wasm_bindgen]
pub fn calculate_c(point1: Point, point2: Point, point3: Point) -> f64 {
    let x1: f64 = point1.x;
    let y1: f64 = point1.y;
    let a: f64 = calculate_a(point1, point2, point3);
    let b: f64 = calculate_b(point1, point2, point3);
    y1 - (x1 * x1) * a - x1 * b
}

#[wasm_bindgen]
pub fn point2quadraticfunction(point1: Point, point2: Point, point3: Point) -> String {
    format!(
        "f(x) = {} * x ^ 2 + {} * x + {}",
        calculate_a(point1, point2, point3),
        calculate_b(point1, point2, point3),
        calculate_c(point1, point2, point3)
    )
}

#[wasm_bindgen]
pub fn x_axis_intersection(point1: Point, point2: Point, point3: Point) {
todo!()
}
