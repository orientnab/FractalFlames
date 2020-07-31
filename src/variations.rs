use super::point::*;

// Sinusoidal
pub fn v1(p: Point) -> Point {
    let Point(x, y) = p;
    Point(
        js_sys::Math::sin(x as f64) as f32,
        js_sys::Math::sin(y as f64) as f32,
    )
}

// Spherical
pub fn v2(p: Point) -> Point {
    let Point(x, y) = p;
    let r2 = x * x + y * y;
    let damp = 0.0;
    1.0 / (r2 + damp) * Point(x, y)
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
    let damp = 0.0;
    1.0 / (r + damp) * Point((x - y) * (x + y), 2.0 * x * y)
}

// Polar
pub fn v5(p: Point) -> Point {
    let Point(x, y) = p;
    let pi = js_sys::Math::acos(-1.0);
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan2(x as f64, y as f64);
    Point((theta / pi) as f32, (r - 1.0) as f32)
}

// Handkerchief
pub fn v6(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan2(x as f64, y as f64);
    let sintr = js_sys::Math::sin(theta + r);
    let costr = js_sys::Math::cos(theta - r);
    Point((r * sintr) as f32, (r * costr) as f32)
}

// Heart
pub fn v7(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan2(x as f64, y as f64);
    let sintr = js_sys::Math::sin(theta * r);
    let costr = js_sys::Math::cos(theta * r);
    Point((r * sintr) as f32, (-r * costr) as f32)
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

// Spiral
pub fn v9(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let damp = 0.0;
    let theta = js_sys::Math::atan2(y as f64, x as f64);
    let sint = js_sys::Math::sin(theta);
    let cost = js_sys::Math::cos(theta);
    let sinr = js_sys::Math::sin(r);
    let cosr = js_sys::Math::cos(r);
    Point(((cost + sinr) / (r + damp)) as f32, ((sint + cosr) / (r + damp)) as f32)
}

// Hyperbolic
pub fn v10(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let damp = 0.0;
    let theta = js_sys::Math::atan2(y as f64, x as f64);
    let sint = js_sys::Math::sin(theta);
    let cost = js_sys::Math::cos(theta);
    Point((sint / (r + damp)) as f32, (r * cost) as f32)
}

// Diamond
pub fn v11(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan2(y as f64, x as f64);
    let sint = js_sys::Math::sin(theta);
    let cost = js_sys::Math::cos(theta);
    let sinr = js_sys::Math::sin(r);
    let cosr = js_sys::Math::cos(r);
    Point((sint * cosr) as f32, (cost * sinr) as f32)
}

// Ex
pub fn v12(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let theta = js_sys::Math::atan2(y as f64, x as f64);
    let sintr = js_sys::Math::sin(theta);
    let costr = js_sys::Math::cos(theta);
    Point(
        (r * (sintr * sintr * sintr + costr * costr * costr)) as f32,
        (r * (sintr * sintr * sintr - costr * costr * costr)) as f32,
    )
}

// Julia
pub fn v13(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    let sqrtr = js_sys::Math::sqrt(r);
    let theta = js_sys::Math::atan2(y as f64, x as f64);
    let omega = 0.0; // randomly 0 or pi
    let sinto = js_sys::Math::sin(theta / 2.0 + omega);
    let costo = js_sys::Math::cos(theta / 2.0 + omega);
    sqrtr as f32 * Point(costo as f32, sinto as f32)
}

// Fisheye
pub fn v16(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    (2.0 / (r as f32 + 1.0)) as f32 * Point(y, x)
}

// // Cosine
// pub fn v20(p: Point) -> Point {
//     let Point(x, y) = p;
//     let pi = js_sys::Math::acos(-1.0);
//     let sinx = js_sys::Math::sin(pi * x as f64);
//     let cosx = js_sys::Math::cos(pi * x as f64);
//     let sinhy = js_sys::Math::sinh(y as f64);
//     let coshy = js_sys::Math::cosh(y as f64);
//     Point((cosx * coshy) as f32, (-sinx * sinhy) as f32)
// }

// Eyefish
pub fn v27(p: Point) -> Point {
    let Point(x, y) = p;
    let r = js_sys::Math::sqrt((x * x + y * y) as f64);
    (2.0 / (r as f32 + 1.0)) as f32 * Point(x, y)
}

// Bubble
pub fn v28(p: Point) -> Point {
    let Point(x, y) = p;
    let r2 = x * x + y * y;
    (4.0 / (r2 + 4.0)) * Point(x, y)
}

// Cylinder
pub fn v29(p: Point) -> Point {
    let Point(x, y) = p;
    let sinx = js_sys::Math::sin(x as f64);
    Point(sinx as f32, y)
}

// Tangent
pub fn v42(p: Point) -> Point {
    let Point(x, y) = p;
    let sinx = js_sys::Math::sin(x as f64);
    let cosy = js_sys::Math::cos(y as f64);
    let tany = js_sys::Math::tan(y as f64);
    Point((sinx / cosy) as f32, tany as f32)
}
