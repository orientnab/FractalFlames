use super::point::*;

#[derive(Debug, Copy, Clone)]
struct Blob {
    high: f32,
    low: f32,
    waves: f32,
}

#[derive(Debug, Copy, Clone)]
struct Pdj {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

#[derive(Debug, Copy, Clone)]
struct Fan {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
struct Curl {
    c1: f32,
    c2: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Params {
    blob: Blob,
    pdj: Pdj,
    fan: Fan,
    curl: Curl,
}

#[derive(Debug, Copy, Clone)]
pub struct PreProc {
    coeffs: (f32, f32, f32, f32, f32, f32),
    params: Params,
    pi: f32,
    x: f32,
    y: f32,
    r: f32,
    r_inv: f32,
    r2: f32,
    r2_inv: f32,
    theta: f32,
    sinx: f32,
    siny: f32,
    cosy: f32,
    tany: f32,
    sint: f32,
    cost: f32,
    sinr: f32,
    cosr: f32,
    sinr2: f32,
    cosr2: f32,
    sintr_sum: f32,
    costr_sum: f32,
    sintr_prod: f32,
    costr_prod: f32,
    sinpr: f32,
    cospr: f32,
    sinpy: f32,
    cospy: f32,
}

impl Params {
    pub fn new(num_functions: usize) -> Vec<Params> {
        (0..num_functions)
            .map(|_| {
                let low = js_sys::Math::random() as f32;
                let high = low + js_sys::Math::random() as f32;
                let waves = js_sys::Math::floor(js_sys::Math::random() * 8.0) as f32;
                let pdj_a = 3.0 * js_sys::Math::random() as f32;
                let pdj_b = 3.0 * js_sys::Math::random() as f32;
                let pdj_c = 3.0 * js_sys::Math::random() as f32;
                let pdj_d = 3.0 * js_sys::Math::random() as f32;
                let fanx = js_sys::Math::random() as f32;
                let fany = js_sys::Math::random() as f32;
                let curlc1 = js_sys::Math::random() as f32;
                let curlc2 = js_sys::Math::random() as f32;
                Params {
                    blob: Blob {
                        high: high,
                        low: low,
                        waves: waves,
                    },
                    pdj: Pdj {
                        a: pdj_a,
                        b: pdj_b,
                        c: pdj_c,
                        d: pdj_d,
                    },
                    fan: Fan { x: fanx, y: fany },
                    curl: Curl {
                        c1: curlc1,
                        c2: curlc2,
                    },
                }
            })
            .collect::<Vec<Params>>()
    }
}

pub fn pre_proc(p: Point, coeffs: (f32, f32, f32, f32, f32, f32), params: &Params) -> PreProc {
    let Point(x, y) = p;
    let pi = (-1.0_f32).acos();
    let r2 = x * x + y * y;
    let r = r2.sqrt();
    let theta = x.atan2(y);
    PreProc {
        coeffs: coeffs,
        params: *params,
        pi: pi,
        x: x,
        y: y,
        r: r,
        r_inv: 1.0 / r,
        r2: r2,
        r2_inv: 1.0 / r2,
        theta: theta,
        sinx: x.sin(),
        siny: y.sin(),
        cosy: y.cos(),
        tany: y.tan(),
        sint: theta.sin(),
        cost: theta.cos(),
        sinr: r.sin(),
        cosr: r.sin(),
        sinr2: r2.sin(),
        cosr2: r2.cos(),
        sintr_sum: (theta + r).sin(),
        costr_sum: (theta - r).cos(),
        sintr_prod: (theta * r).sin(),
        costr_prod: (theta * r).sin(),
        sinpr: (pi * r).sin(),
        cospr: (pi * r).cos(),
        sinpy: (pi * y).sin(),
        cospy: (pi * y).cos(),
    }
}
// Sinusoidal
pub fn v1(p: &PreProc) -> Point {
    Point(p.sinx, p.siny)
}

// Spherical
pub fn v2(p: &PreProc) -> Point {
    p.r2_inv * Point(p.x, p.y)
}

// Swirl
pub fn v3(p: &PreProc) -> Point {
    Point(p.x * p.sinr2 - p.y * p.cosr2, p.x * p.cosr2 + p.y * p.sinr2)
}

// Horseshoe
pub fn v4(p: &PreProc) -> Point {
    p.r_inv * Point((p.x - p.y) * (p.x + p.y), 2.0 * p.x * p.y)
}

// Polar
pub fn v5(p: &PreProc) -> Point {
    Point(p.theta / p.pi, p.r - 1.0)
}

// Handkerchief
pub fn v6(p: &PreProc) -> Point {
    Point(p.r * p.sintr_sum, p.r * p.costr_sum)
}

// Heart
pub fn v7(p: &PreProc) -> Point {
    Point(p.r * p.sintr_prod, -p.r * p.costr_prod)
}

// Disc
pub fn v8(p: &PreProc) -> Point {
    (p.theta / p.pi) * Point(p.sinpr, p.cospr)
}

// Spiral
pub fn v9(p: &PreProc) -> Point {
    p.r_inv * Point(p.cost + p.sinr, p.sint - p.cosr)
}

// Hyperbolic
pub fn v10(p: &PreProc) -> Point {
    Point(p.sint * p.r_inv, p.r * p.cost)
}

// Diamond
pub fn v11(p: &PreProc) -> Point {
    Point(p.sint * p.cosr, p.cost * p.sinr)
}

// Ex
pub fn v12(p: &PreProc) -> Point {
    let p03 = p.sintr_sum * p.sintr_sum * p.sintr_sum;
    let p13 = p.costr_sum * p.costr_sum * p.costr_sum;
    p.r * Point(p03 + p13, p03 - p13)
}

// Julia
pub fn v13(p: &PreProc) -> Point {
    let sqrtr = p.r.sqrt();
    // let omega = 0.0; // randomly 0 or pi
    let omega = (js_sys::Math::random() - 0.5).signum() as f32; // randomly 0 or pi
    let sinto = (p.theta / 2.0 + omega).sin();
    let costo = (p.theta / 2.0 + omega).cos();
    sqrtr * Point(costo, sinto)
}

// Bent
pub fn v14(p: &PreProc) -> Point {
    let x = if p.x >= 0.0 { p.x } else { 2.0 * p.x };
    let y = if p.y >= 0.0 { p.y } else { p.x / 2.0 };
    Point(x, y)
}

// Waves
pub fn v15(p: &PreProc) -> Point {
    let sinyc2 = (p.y / (p.coeffs.2 * p.coeffs.2)).sin();
    let sinxf2 = (p.x / (p.coeffs.5 * p.coeffs.5)).sin();
    Point(p.x + p.coeffs.1 * sinyc2, p.y + p.coeffs.4 * sinxf2)
}

// Fisheye
pub fn v16(p: &PreProc) -> Point {
    (2.0 / (p.r + 1.0)) * Point(p.y, p.x)
}

// Popcorn
pub fn v17(p: &PreProc) -> Point {
    let sintan3y = ((3.0 * p.y).tan()).sin();
    let sintan3x = ((3.0 * p.x).tan()).sin();
    Point(p.x + p.coeffs.2 * sintan3y, p.y + p.coeffs.5 * sintan3x)
}

// // Exponential
// pub fn v18(p: &PreProc) -> Point {
//     let expx1 = js_sys::Math::exp(p.x as f64 - 1.0) as f32;
//      expx1 * Point(p.cospy, p.sinpy)
// }

// Power
pub fn v19(p: &PreProc) -> Point {
    let rsint = p.r.powf(p.sint);
    rsint * Point(p.cost, p.sint)
}

// // Cosine
// pub fn v20(p: &PreProc) -> Point {
//     let sinx = (p.pi * p.x).sin();
//     let cosx = (p.pi * p.x).cos();
//     let sinhy = p.y.sinh();
//     let coshy = p.y.cosh();
//     Point(cosx * coshy, -sinx * sinhy)
// }

// Rings
pub fn v21(p: &PreProc) -> Point {
    let c2 = p.coeffs.2 * p.coeffs.2;
    let modulo = (p.r + c2).rem_euclid(2.0 * c2);
    let factor = modulo - c2 + p.r * (1.0 - c2);
    factor * Point(p.cost, p.sint)
}

// Fan
pub fn v22(p: &PreProc) -> Point {
    let t = p.pi * p.coeffs.2 * p.coeffs.2;
    let modulo = (p.theta + p.coeffs.5).rem_euclid(t);
    match modulo {
        m if m > t / 2.0 => {
            let sin_minus = (p.theta - t / 2.0).sin();
            let cos_minus = (p.theta - t / 2.0).cos();
            p.r * Point(cos_minus, sin_minus)
        }
        _ => {
            let sin_plus = (p.theta + t / 2.0).sin();
            let cos_plus = (p.theta + t / 2.0).cos();
            p.r * Point(cos_plus, sin_plus)
        }
    }
}

// Blob
pub fn v23(p: &PreProc) -> Point {
    let p1 = p.params.blob.low;
    let p2 = (p.params.blob.high - p.params.blob.low) / 2.0;
    let p3 = (p.params.blob.waves * p.theta).sin();
    let factor = p.r * (p1 + p2 * (p3 + 1.0));
    factor * Point(p.cost, p.sint)
}

// PDJ
pub fn v24(p: &PreProc) -> Point {
    Point(
        (p.params.pdj.a * p.y).sin() - (p.params.pdj.b * p.x).cos(),
        (p.params.pdj.c * p.x).sin() - (p.params.pdj.d * p.y).cos(),
    )
}

// Fan2
pub fn v25(p: &PreProc) -> Point {
    let p1 = 0.5 * p.pi * p.params.fan.x * p.params.fan.x;
    let p2 = p.params.fan.y;
    let t = p.theta + p2 - 2.0 * p1 * (p.theta * p2 / p1).trunc();
    if t > p1 {
        p.r * Point((p.theta - p1).sin(), (p.theta - p1).cos())
    } else {
        p.r * Point((p.theta + p1).sin(), (p.theta + p1).cos())
    }
}

// // Rings2
// pub fn v26(p: &PreProc) -> Point {
//     Point(0.0, 0.0)
// }

// Eyefish
pub fn v27(p: &PreProc) -> Point {
    (2.0 / (p.r + 1.0)) * Point(p.x, p.y)
}

// Bubble
pub fn v28(p: &PreProc) -> Point {
    (4.0 / (p.r2 + 4.0)) * Point(p.x, p.y)
}

// Cylinder
pub fn v29(p: &PreProc) -> Point {
    Point(p.sinx, p.y)
}

// Curl
pub fn v39(p: &PreProc) -> Point {
    let p1 = p.params.curl.c1;
    let p2 = p.params.curl.c2;
    let t1 = 1.0 + p1 * p.x + p2 * (p.x * p.x - p.y * p.y);
    let t2 = p1 * p.y + 2.0 * p2 * p.x * p.y;
    1.0 / (t1 * t1 + t2 * t2) * Point(p.x * t1 + p.y * t2, p.y * t1 - p.x * t2)
}

// Tangent
pub fn v42(p: &PreProc) -> Point {
    Point(p.sinx / p.cosy, p.tany)
}

// // Sinusoidal
// pub fn v1(p: Point) -> Point {
//     let Point(x, y) = p;
//     Point(
//         js_sys::Math::sin(x as f64) as f32,
//         js_sys::Math::sin(y as f64) as f32,
//     )
// }

// // Spherical
// pub fn v2(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r2 = x * x + y * y;
//     let damp = 0.0;
//     1.0 / (r2 + damp) * Point(x, y)
// }

// // Swirl
// pub fn v3(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r2 = x * x + y * y;
//     let sinr2 = js_sys::Math::sin(r2 as f64) as f32;
//     let cosr2 = js_sys::Math::cos(r2 as f64) as f32;
//     Point(x * sinr2 - y * cosr2, x * cosr2 + y * sinr2)
// }

// // Horseshoe
// pub fn v4(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64) as f32;
//     let damp = 0.0;
//     1.0 / (r + damp) * Point((x - y) * (x + y), 2.0 * x * y)
// }

// // Polar
// pub fn v5(p: Point) -> Point {
//     let Point(x, y) = p;
//     let pi = js_sys::Math::acos(-1.0);
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let theta = js_sys::Math::atan2(x as f64, y as f64);
//     Point((theta / pi) as f32, (r - 1.0) as f32)
// }

// // Handkerchief
// pub fn v6(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let theta = js_sys::Math::atan2(x as f64, y as f64);
//     let sintr = js_sys::Math::sin(theta + r);
//     let costr = js_sys::Math::cos(theta - r);
//     Point((r * sintr) as f32, (r * costr) as f32)
// }

// // Heart
// pub fn v7(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let theta = js_sys::Math::atan2(x as f64, y as f64);
//     let sintr = js_sys::Math::sin(theta * r);
//     let costr = js_sys::Math::cos(theta * r);
//     Point((r * sintr) as f32, (-r * costr) as f32)
// }

// // Disc
// pub fn v8(p: Point) -> Point {
//     let Point(x, y) = p;
//     let pi = js_sys::Math::acos(-1.0);
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let theta = js_sys::Math::atan2(y as f64, x as f64);
//     let sinpr = js_sys::Math::sin(theta * r);
//     let cospr = js_sys::Math::cos(theta * r);
//     Point((theta / pi * sinpr) as f32, (theta / pi * cospr) as f32)
// }

// // Spiral
// pub fn v9(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let damp = 0.0;
//     let theta = js_sys::Math::atan2(y as f64, x as f64);
//     let sint = js_sys::Math::sin(theta);
//     let cost = js_sys::Math::cos(theta);
//     let sinr = js_sys::Math::sin(r);
//     let cosr = js_sys::Math::cos(r);
//     Point(
//         ((cost + sinr) / (r + damp)) as f32,
//         ((sint + cosr) / (r + damp)) as f32,
//     )
// }

// // Hyperbolic
// pub fn v10(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let damp = 0.0;
//     let theta = js_sys::Math::atan2(y as f64, x as f64);
//     let sint = js_sys::Math::sin(theta);
//     let cost = js_sys::Math::cos(theta);
//     Point((sint / (r + damp)) as f32, (r * cost) as f32)
// }

// // Diamond
// pub fn v11(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let theta = js_sys::Math::atan2(y as f64, x as f64);
//     let sint = js_sys::Math::sin(theta);
//     let cost = js_sys::Math::cos(theta);
//     let sinr = js_sys::Math::sin(r);
//     let cosr = js_sys::Math::cos(r);
//     Point((sint * cosr) as f32, (cost * sinr) as f32)
// }

// // Ex
// pub fn v12(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let theta = js_sys::Math::atan2(y as f64, x as f64);
//     let sintr = js_sys::Math::sin(theta);
//     let costr = js_sys::Math::cos(theta);
//     Point(
//         (r * (sintr * sintr * sintr + costr * costr * costr)) as f32,
//         (r * (sintr * sintr * sintr - costr * costr * costr)) as f32,
//     )
// }

// // Julia
// pub fn v13(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     let sqrtr = js_sys::Math::sqrt(r);
//     let theta = js_sys::Math::atan2(y as f64, x as f64);
//     let omega = 0.0; // randomly 0 or pi
//     let sinto = js_sys::Math::sin(theta / 2.0 + omega);
//     let costo = js_sys::Math::cos(theta / 2.0 + omega);
//     sqrtr as f32 * Point(costo as f32, sinto as f32)
// }

// // Fisheye
// pub fn v16(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     (2.0 / (r as f32 + 1.0)) as f32 * Point(y, x)
// }

// // // Cosine
// // pub fn v20(p: Point) -> Point {
// //     let Point(x, y) = p;
// //     let pi = js_sys::Math::acos(-1.0);
// //     let sinx = js_sys::Math::sin(pi * x as f64);
// //     let cosx = js_sys::Math::cos(pi * x as f64);
// //     let sinhy = js_sys::Math::sinh(y as f64);
// //     let coshy = js_sys::Math::cosh(y as f64);
// //     Point((cosx * coshy) as f32, (-sinx * sinhy) as f32)
// // }

// // Eyefish
// pub fn v27(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r = js_sys::Math::sqrt((x * x + y * y) as f64);
//     (2.0 / (r as f32 + 1.0)) as f32 * Point(x, y)
// }

// // Bubble
// pub fn v28(p: Point) -> Point {
//     let Point(x, y) = p;
//     let r2 = x * x + y * y;
//     (4.0 / (r2 + 4.0)) * Point(x, y)
// }

// // Cylinder
// pub fn v29(p: Point) -> Point {
//     let Point(x, y) = p;
//     let sinx = js_sys::Math::sin(x as f64);
//     Point(sinx as f32, y)
// }

// // Tangent
// pub fn v42(p: Point) -> Point {
//     let Point(x, y) = p;
//     let sinx = js_sys::Math::sin(x as f64);
//     let cosy = js_sys::Math::cos(y as f64);
//     let tany = js_sys::Math::tan(y as f64);
//     Point((sinx / cosy) as f32, tany as f32)
// }
