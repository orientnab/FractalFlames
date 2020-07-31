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

const PIC_WIDTH: u32 = 512;
const PIC_HEIGHT: u32 = 512;
const ITER: usize = 5_000_000;

#[wasm_bindgen]
pub struct Picture {
    width: u32,
    height: u32,
    cell_counter: Vec<u32>,
    cell_alpha: Vec<f32>,
    cell_color: Vec<(f32, f32, f32)>,
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

pub fn weigths(num_variations: usize) -> Vec<f32> {
    let mut weights: Vec<f32> = vec![1.0 / num_variations as f32; num_variations];
    for i in 0..num_variations {
        weights[i] = 2.0 / num_variations as f32 * js_sys::Math::random() as f32;
        // weights[i] = js_sys::Math::random() as f32;
    }
    // weights[0] = js_sys::Math::random() as f32;
    // for i in 1..num_variations {
    //     weights[i] = weights[i - 1] + js_sys::Math::random() as f32;
    // }
    // for i in 0..num_variations {
    //     weights[i] /= weights[num_variations - 1];
    // }
    weights
}

pub fn apply_variation(p: Point, weights: &Vec<f32>, vars: &Vec<fn(Point) -> Point>) -> Point {
    // let mut res = p.clone();
    let mut res = Point::new();
    for i in 0..vars.len() {
        res += weights[i] * vars[i](p);
    }
    return res;
}

#[wasm_bindgen]
impl Picture {
    pub fn new() -> Picture {
        utils::set_panic_hook();
        let width = PIC_WIDTH;
        let height = PIC_HEIGHT;
        let cell_counter = (0..width * height).map(|_| 1).collect();
        let cell_alpha = (0..width * height).map(|_| 0.0).collect();
        let cell_color = (0..width * height).map(|_| (1.0, 1.0, 1.0)).collect();

        Picture {
            width,
            height,
            cell_counter,
            cell_alpha,
            cell_color,
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

    pub fn cell_color(&self) -> *const (f32, f32, f32) {
        self.cell_color.as_slice().as_ptr()
    }

    pub fn paint(&mut self) {
        let vars: Vec<fn(Point) -> Point> = vec![
            // v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v16, v27, v28, v29, v42,
            v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v16, v27, v28, v29, v42,
        ];
        let coeffs_pre1 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_pre2 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_pre3 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_pre4 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_post1 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_post2 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_post3 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        let coeffs_post4 = (
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 0.2 - 0.1,
        );
        // let coeffs_pre = (1.3, 0.3, 0.1, -0.8, 0.4, 0.4);
        // let coeffs_post = (1.3, 0.3, 0.1, -0.8, 0.4, 0.4);
        let weights1 = weigths(vars.len());
        let weights2 = weigths(vars.len());
        let weights3 = weigths(vars.len());
        let weights4 = weigths(vars.len());
        let col1 = (
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
        );
        let col2 = (
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
        );
        let col3 = (
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
        );
        let col4 = (
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
        );
        let colors = [col1, col2, col3, col4];
        let x = js_sys::Math::random() as f32 * 2.0 - 1.0;
        let y = js_sys::Math::random() as f32 * 2.0 - 1.0;
        let mut coord = Point(x, y);
        let mut col;
        for _ in 0..20 {
            let val = js_sys::Math::random();
            if val < 0.25 {
                coord = apply_variation(coord.affine(coeffs_pre1), &weights1, &vars)
                    .affine(coeffs_post1);
            } else if val < 0.5 {
                coord = apply_variation(coord.affine(coeffs_pre2), &weights2, &vars)
                    .affine(coeffs_post2);
            } else if val < 0.75 {
                coord = apply_variation(coord.affine(coeffs_pre3), &weights3, &vars)
                    .affine(coeffs_post3);
            } else {
                coord = apply_variation(coord.affine(coeffs_pre4), &weights4, &vars)
                    .affine(coeffs_post4);
            };
        }
        for _ in 0..ITER {
            let val = js_sys::Math::random();
            if val < 0.25 {
                coord = apply_variation(coord.affine(coeffs_pre1), &weights1, &vars)
                    .affine(coeffs_post1);
                col = colors[0];
            } else if val < 0.5 {
                coord = apply_variation(coord.affine(coeffs_pre2), &weights2, &vars)
                    .affine(coeffs_post2);
                col = colors[1];
            } else if val < 0.75 {
                coord = apply_variation(coord.affine(coeffs_pre3), &weights3, &vars)
                    .affine(coeffs_post3);
                col = colors[2];
            } else {
                coord = apply_variation(coord.affine(coeffs_pre4), &weights4, &vars)
                    .affine(coeffs_post4);
                col = colors[3];
            };
            let idx_opt = self.get_index_from_coord(&coord);
            match idx_opt {
                Some(idx) => {
                    self.cell_counter[idx] += 1;
                    self.cell_color[idx].0 = self.cell_color[idx].0 + col.0;
                    self.cell_color[idx].1 = self.cell_color[idx].1 + col.1;
                    self.cell_color[idx].2 = self.cell_color[idx].2 + col.2;
                }
                None => (),
            }
            // log!("{:?}", coord);
        }
        let max_counter = self.cell_counter.iter().cloned().fold(0, u32::max);
        let log_max_counter = js_sys::Math::log(max_counter as f64);
        // for i in 0..(self.width as usize * self.height as usize) {
        for i in 0..(self.width * self.height) as usize {
            // log!("{}: {}", i, self.cell_counter[i]);
            self.cell_alpha[i] =
                (js_sys::Math::log(self.cell_counter[i] as f64) / log_max_counter) as f32;
            self.cell_color[i].0 =
                (js_sys::Math::log(self.cell_color[i].0 as f64) / log_max_counter) as f32;
            self.cell_color[i].1 =
                (js_sys::Math::log(self.cell_color[i].1 as f64) / log_max_counter) as f32;
            self.cell_color[i].2 =
                (js_sys::Math::log(self.cell_color[i].2 as f64) / log_max_counter) as f32;
            // Gamma correction
            self.cell_color[i].0 = (js_sys::Math::pow(self.cell_color[i].0 as f64, 1.0 / 2.2)) as f32;
            self.cell_color[i].1 = (js_sys::Math::pow(self.cell_color[i].1 as f64, 1.0 / 2.2)) as f32;
            self.cell_color[i].2 = (js_sys::Math::pow(self.cell_color[i].2 as f64, 1.0 / 2.2)) as f32;

        }
    }
}
