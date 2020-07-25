mod utils;

use js_sys;
use wasm_bindgen::prelude::*;
use web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// const WEIGHTS: [f32; 2] = [0.3, 0.6];
const WEIGHTS: [f32; 4] = [0.2, 0.4, 0.6, 0.8];

#[wasm_bindgen]
pub struct Picture {
    width: u32,
    height: u32,
    cell_counter: Vec<u32>,
    cell_alpha: Vec<f32>,
    // cell_color: Vec<(u8, u8, u8)>,
}

#[wasm_bindgen]
pub struct Point(f32, f32);

impl Picture {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_index_from_coord(&self, coord: &Point) -> Option<usize> {
        let Point(x, y) = coord;
        if x.abs() > 1.0 || y.abs() > 1.0 {
            None
        } else {
            let a = js_sys::Math::floor(((x + 1.0) / 2.0 * self.width as f32) as f64) as u32;
            let b = js_sys::Math::floor(((y + 1.0) / 2.0 * self.height as f32) as f64) as u32;
            Some(self.get_index(a, b))
        }
    }
}

impl Point {
    pub fn affine(&self, coeffs: (f32, f32, f32, f32, f32, f32)) -> Point {
        let Point(x, y) = self;
        let (a, b, c, d, e, f) = coeffs;
        Point(a * x + b * y + c, d * x + e * y + f)
    }
}

pub fn f0(p: Point) -> Point {
    let Point(x, y) = p;
    Point(x / 2.0, y / 2.0)
}

pub fn f1(p: Point) -> Point {
    let Point(x, y) = p;
    Point((x + 1.0) / 2.0, y / 2.0)
}

pub fn f2(p: Point) -> Point {
    let Point(x, y) = p;
    Point(x / 2.0, (y + 1.0) / 2.0)
}

pub fn vrand() -> Point {
    Point(
        js_sys::Math::random() as f32 * 2.0 - 1.0,
        js_sys::Math::random() as f32 * 2.0 - 1.0,
    )
}

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

pub fn apply_variation(p: Point) -> Point {
    let coeffs_pre = (1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let coeffs_post = (1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    // let coeffs = (1.3, 0.3, 0.1, -0.8, 0.4, 0.4);
    let val = js_sys::Math::random() as f32;
    if val < WEIGHTS[0] {
        v2(p.affine(coeffs_pre)).affine(coeffs_post)
    } else if val < WEIGHTS[1] {
        v3(p.affine(coeffs_pre)).affine(coeffs_post)
    } else if val < WEIGHTS[2] {
        v4(p.affine(coeffs_pre)).affine(coeffs_post)
    } else if val < WEIGHTS[3] {
        v6(p.affine(coeffs_pre)).affine(coeffs_post)
    } else {
        v8(p.affine(coeffs_pre)).affine(coeffs_post)
    }
}

#[wasm_bindgen]
impl Picture {
    pub fn new() -> Picture {
        utils::set_panic_hook();
        let width = 128;
        let height = 128;
        let cell_counter = (0..width * height).map(|_| 1).collect();
        let cell_alpha = (0..width * height).map(|_| 0.0).collect();
        // let cell_color = (0..width * height)
        //     .map(|_| {
        //         if js_sys::Math::random() < 0.5 {
        //             (0, 0, 0)
        //         } else {
        //             (0, 0, 0)
        //         }
        //     })
        //     .collect();

        Picture {
            width,
            height,
            cell_counter,
            cell_alpha,
            // cell_color,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cell_counter(&self) -> *const u32 {
        self.cell_counter.as_slice().as_ptr()
    }

    pub fn cell_alpha(&self) -> *const f32 {
        self.cell_alpha.as_slice().as_ptr()
    }

    //     pub fn cell_color(&self) -> *const (u8, u8, u8) {
    //         self.cell_color.as_slice().as_ptr()
    //     }

    pub fn paint(&mut self) {
        let x = js_sys::Math::random() as f32 * 2.0 - 1.0;
        let y = js_sys::Math::random() as f32 * 2.0 - 1.0;
        let mut coord = Point(x, y);
        for _ in 0..20 {
            coord = apply_variation(coord);
        }
        for _ in 0..5000000 {
            coord = apply_variation(coord);
            let idx_opt = self.get_index_from_coord(&coord);
            match idx_opt {
                Some(idx) => self.cell_counter[idx] += 1,
                None => (),
            }
        }
        let max_counter = self.cell_counter.iter().cloned().fold(0, u32::max);
        let log_max_counter = js_sys::Math::log(max_counter as f64);
        // for i in 0..(self.width as usize * self.height as usize) {
        for i in 0..(self.width * self.height) as usize {
            // log!("{}: {}", i, self.cell_counter[i]);
            self.cell_alpha[i] =
                (js_sys::Math::log(self.cell_counter[i] as f64) / log_max_counter) as f32;
        }
    }
}
