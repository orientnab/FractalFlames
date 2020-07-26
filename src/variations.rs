use super::point::*;

// // Sinusoidal
// pub fn v1(p: Point) -> Point {
//     let Point(x, y) = p;
//     Point(
//         js_sys::Math::sin(x as f64) as f32,
//         js_sys::Math::sin(y as f64) as f32,
//     )
// }

// Spherical
pub fn v2(p: Point) -> Point {
    let Point(x, y) = p;
    let r2 = x * x + y * y;
    Point(1.0 / r2 * x, 1.0 / r2 * y)
}

// Swirl
pub fn v3(p: Point) -> Point {
    let Point(x, y) = p;
    let r2 = x * x + y * y;
    let sinr2 = js_sys::Math::sin(r2 as f64) as f32;
    let cosr2 = js_sys::Math::cos(r2 as f64) as f32;
    Point(x * sinr2 - y * cosr2, x * cosr2 + y * sinr2)
}

// Horseshoe
pub fn v4(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64) as f32;
    Point(1.0 / r * (x - y) * (x + y), 1.0 / r * 2.0 * x * y)
}

// Handkerchief
pub fn v6(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan(x as f64 / y as f64);
    let sintr = js_sys::Math::sin(theta + r);
    let costr = js_sys::Math::cos(theta - r);
    Point((r * sintr) as f32, (r * costr) as f32)
}

// Disc
pub fn v8(p: Point) -> Point {
    let Point(x, y) = p;
    let pi = js_sys::Math::acos(-1.0);
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan2(y as f64, x as f64);
    let sinpr = js_sys::Math::sin(theta * r);
    let cospr = js_sys::Math::cos(theta * r);
    Point((theta / pi * sinpr) as f32, (theta / pi * cospr) as f32)
}

