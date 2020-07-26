pub mod point;
mod utils;
mod variations;

use point::*;
use variations::*;

use js_sys;
use wasm_bindgen::prelude::*;
// use web_sys;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const PIC_WIDTH: u32 = 128;
const PIC_HEIGHT: u32 = 128;
const ITER: usize = 5_000_000;

#[wasm_bindgen]
pub struct Picture {
    width: u32,
    height: u32,
    cell_counter: Vec<u32>,
    cell_alpha: Vec<f32>,
    // cell_color: Vec<(u8, u8, u8)>,
}

impl Picture {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_index_from_coord(&self, coord: &Point) -> Option<usize> {
        let Point(x, y) = coord;
        if x.abs() >= 1.0 || y.abs() >= 1.0 {
            None
        } else {
            let a = js_sys::Math::floor(((x + 1.0) / 2.0 * self.width as f32) as f64) as u32;
            let b = js_sys::Math::floor(((y + 1.0) / 2.0 * self.height as f32) as f64) as u32;
            Some(self.get_index(
                std::cmp::min(a, self.width - 1),
                std::cmp::min(b, self.height - 1),
            ))
        }
    }
}

pub fn weigths(num_variations: u32) -> Vec<f32> {
    let mut weights: Vec<f32> = vec![0.0; num_variations as usize];
    weights[0] = js_sys::Math::random() as f32;
    for i in 1..num_variations as usize {
        weights[i] = weights[i - 1] + js_sys::Math::random() as f32;
    }
    for i in 0..num_variations as usize {
        weights[i] /= weights[num_variations as usize - 1];
    }
    weights
}

pub fn apply_variation(p: Point, weights: &Vec<f32>, vars: &Vec<fn(Point) -> Point>) -> Point {
    let coeffs_pre = (1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let coeffs_post = (1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    // let coeffs = (1.3, 0.3, 0.1, -0.8, 0.4, 0.4);
    // let mut res = p;

    let mut res = p.clone();
    // res = v2(res.affine(coeffs_pre)).affine(coeffs_post);
    // return res;

    // let val = js_sys::Math::random() as f32;

    for i in 0..vars.len() {
        res += weights[i] * vars[i](p.affine(coeffs_pre)).affine(coeffs_post);
    }

    return res;

    // if val < weights[0] {
    //     v2(p.affine(coeffs_pre)).affine(coeffs_post)
    //     // vars[0](p)
    //     // vars[0](p.affine(coeffs_pre)).affine(coeffs_post)
    // } else if val < weights[1] {
    //     v3(p.affine(coeffs_pre)).affine(coeffs_post)
    // } else if val < weights[2] {
    //     v4(p.affine(coeffs_pre)).affine(coeffs_post)
    // } else if val < weights[3] {
    //     v6(p.affine(coeffs_pre)).affine(coeffs_post)
    // } else {
    //     v8(p.affine(coeffs_pre)).affine(coeffs_post)
    // }
}

#[wasm_bindgen]
impl Picture {
    pub fn new() -> Picture {
        utils::set_panic_hook();
        let width = PIC_WIDTH;
        let height = PIC_HEIGHT;
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
        let weights = weigths(5);
        let vars: Vec<fn(Point) -> Point> = vec![v2, v3, v4, v6, v8];
        let x = js_sys::Math::random() as f32 * 2.0 - 1.0;
        let y = js_sys::Math::random() as f32 * 2.0 - 1.0;
        let mut coord = Point(x, y);
        for _ in 0..20 {
            coord = apply_variation(coord, &weights, &vars);
        }
        for _ in 0..ITER {
            coord = apply_variation(coord, &weights, &vars);
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
