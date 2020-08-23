//! A library for creating abstract flames inspired by the paper by Draves (2003)

mod point;
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
const ITER: usize = 100_000;
const NUM_FUNCTIONS: usize = 6;
const GAMMA: f32 = 2.2;

/// Representation of the picture
#[wasm_bindgen]
pub struct Picture {
    width: u32,
    height: u32,
    /// Total number of times a particular cell is selected by the algorithm
    cell_counter: Vec<u32>,
    /// Representation of the color of the cell in B/W scale.
    cell_alpha: Vec<f32>,
    /// Representation of the color of the cell in RGB scale.
    cell_color: Vec<(f32, f32, f32)>,
}

impl Picture {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Given the coordinates of a point in [-1, 1]x[-1, 1],
    /// returns its corresponding index in Picture
    fn get_index_from_coord(&self, coord: &Point) -> Option<usize> {
        let Point(x, y) = coord;
        if x.abs() >= 1.0 || y.abs() >= 1.0 {
            None
        } else {
            let a = ((x + 1.0) / 2.0 * self.width as f32).floor() as u32;
            let b = ((y + 1.0) / 2.0 * self.height as f32).floor() as u32;
            Some(self.get_index(
                std::cmp::min(a, self.width - 1),
                std::cmp::min(b, self.height - 1),
            ))
        }
    }
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

    /// Returns width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a pointer to the array that contains the number of times individual cells have been
    /// hit
    pub fn cell_counter(&self) -> *const u32 {
        self.cell_counter.as_slice().as_ptr()
    }

    /// Returns a pointer to the array that contains the B/W color of individual cells
    pub fn cell_alpha(&self) -> *const f32 {
        self.cell_alpha.as_slice().as_ptr()
    }

    /// Returns a pointer to the array that contains the RGB color of individual cells
    pub fn cell_color(&self) -> *const (f32, f32, f32) {
        self.cell_color.as_slice().as_ptr()
    }

    /// Creates a Fractal Flame
    pub fn paint(&mut self) {
        // Variations to be used.
        // Variations are defined in `src/variations.rs`
        let vars: Vec<fn(&PreProc) -> Point> = vec![
            v1, v2, v3, v4, v5, v6, v8, v9, v10, v11, v12, v13, v14, v15, v16, v19, v23, v24, v27,
            v28, v29, v39, v42,
        ];

        // Randomly choose coefficients for the affine pre/post transformations, parmeters for
        // parametric variations (such as pdj or popcorn), variations' weights, colors and
        // probability threshold of each function.
        let coeffs_pre = create_coeffs(NUM_FUNCTIONS);
        let coeffs_post = create_coeffs(NUM_FUNCTIONS);
        let params = Params::new(NUM_FUNCTIONS);
        let weights = weigths(NUM_FUNCTIONS, vars.len());
        let colors = create_colors(NUM_FUNCTIONS);
        let threshold = prob_dist(NUM_FUNCTIONS);

        let coeffs_pre_final = create_coeffs(1);
        let coeffs_post_final = create_coeffs(1);
        let params_final = Params::new(1);
        let weights_final = weigths(1, vars.len());

        // Sets an initial random point in the canvas and skip the first 20 iterations
        // After 20 iterations any random point has fallen into the actual
        // shape of the attractor and we can start recording data for the picture
        let mut coord = Point(
            js_sys::Math::random() as f32 * 2.0 - 1.0,
            js_sys::Math::random() as f32 * 2.0 - 1.0,
        );
        let mut col = colors[0];
        for _ in 0..20 {
            let val = js_sys::Math::random() as f32;
            for idx_threshold in 0..NUM_FUNCTIONS {
                if val < threshold[idx_threshold] {
                    let pre_proc = pre_proc(
                        coord.affine(coeffs_pre[idx_threshold]),
                        coeffs_pre[idx_threshold],
                        &params[idx_threshold],
                    );
                    coord = Point::apply_variation(&pre_proc, &weights[idx_threshold], &vars)
                        .affine(coeffs_post[idx_threshold]);
                    break;
                }
            }
        }
        // Closelly following the algorithm in Draves (2003)
        //
        // 1. Pics a random function. Each function has a probability threshold
        // 2. Applies an affine transformation
        // 3. Applies a function (a weighted set of variations)
        // 4. Applies an affine transformation
        // 5. Applies a final common function
        // 6. The resulting point is stored as a counter and a color
        for _ in 0..ITER {
            let val = js_sys::Math::random() as f32;
            for idx_threshold in 0..NUM_FUNCTIONS {
                if val < threshold[idx_threshold] {
                    let pre_proc = pre_proc(
                        coord.affine(coeffs_pre[idx_threshold]),
                        coeffs_pre[idx_threshold],
                        &params[idx_threshold],
                    );
                    coord = Point::apply_variation(&pre_proc, &weights[idx_threshold], &vars)
                        .affine(coeffs_post[idx_threshold]);
                    col = colors[idx_threshold];
                    break;
                }
            }

            let pre_proc = pre_proc(
                coord.affine(coeffs_pre_final[0]),
                coeffs_pre_final[0],
                &params_final[0],
            );
            coord = Point::apply_variation(&pre_proc, &weights_final[0], &vars)
                .affine(coeffs_post_final[0]);

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
        }

        // Rescale counter and color for all cells
        let max_counter = self.cell_counter.iter().cloned().fold(0, u32::max);
        let log_max_counter = (max_counter as f32).ln();
        for i in 0..(self.width * self.height) as usize {
            self.cell_alpha[i] = (self.cell_counter[i] as f32).ln() / log_max_counter;
            self.cell_color[i].0 = self.cell_color[i].0.ln() / log_max_counter;
            self.cell_color[i].1 = self.cell_color[i].1.ln() / log_max_counter;
            self.cell_color[i].2 = self.cell_color[i].2.ln() / log_max_counter;
            // Gamma correction
            self.cell_color[i].0 = self.cell_color[i].0.powf(1.0 / GAMMA);
            self.cell_color[i].1 = self.cell_color[i].1.powf(1.0 / GAMMA);
            self.cell_color[i].2 = self.cell_color[i].2.powf(1.0 / GAMMA);
        }

        // Auxiliary functions

        fn create_coeffs(num_functions: usize) -> Vec<(f32, f32, f32, f32, f32, f32)> {
            (0..num_functions)
                .map(|_| {
                    (
                        js_sys::Math::random() as f32 * 2.0 - 1.0,
                        js_sys::Math::random() as f32 * 2.0 - 1.0,
                        js_sys::Math::random() as f32 * 0.2 - 0.1,
                        js_sys::Math::random() as f32 * 2.0 - 1.0,
                        js_sys::Math::random() as f32 * 2.0 - 1.0,
                        js_sys::Math::random() as f32 * 0.2 - 0.1,
                    )
                })
                .collect::<Vec<(f32, f32, f32, f32, f32, f32)>>()
        }

        fn create_colors(num_functions: usize) -> Vec<(f32, f32, f32)> {
            (0..num_functions)
                .map(|_| {
                    (
                        // js_sys::Math::random() as f32,
                        1.0,
                        js_sys::Math::random() as f32,
                        js_sys::Math::random() as f32,
                    )
                })
                .collect::<Vec<(f32, f32, f32)>>()
        }

        fn prob_dist(num_functions: usize) -> Vec<f32> {
            let mut weights: Vec<f32> = vec![0.0; num_functions];
            weights[0] = js_sys::Math::random() as f32;
            for i in 1..num_functions {
                weights[i] = weights[i - 1] + js_sys::Math::random() as f32;
            }
            for i in 0..num_functions {
                weights[i] /= weights[num_functions - 1];
            }
            weights
        }

        fn weigths(num_functions: usize, num_variations: usize) -> Vec<Vec<f32>> {
            (0..num_functions)
                .map(|_| {
                    let mut weights: Vec<f32> = vec![1.0 / num_variations as f32; num_variations];
                    for i in 0..num_variations {
                        // weights[i] = 2.0 / num_variations as f32 * js_sys::Math::random() as f32;
                        weights[i] = js_sys::Math::random() as f32;
                    }
                    let weights_sum: f32 = weights.iter().cloned().sum();
                    for i in 0..num_variations {
                        weights[i] /= weights_sum;
                    }
                    weights
                })
                .collect::<Vec<Vec<f32>>>()
        }
    }
}
