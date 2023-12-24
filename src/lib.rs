#![allow(dead_code)]
#![allow(unused_variables)]

mod complex;
mod polynomial;
mod utils;

use complex::Complex;
use polynomial::Poly;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ColorInfo {
    color_id: u8,
    iterations: u8,
}

#[wasm_bindgen]
pub struct NewtonFractal {
    colors: Vec<ColorInfo>,
}

#[wasm_bindgen]
impl NewtonFractal {
    pub fn new(
        roots: Vec<Complex>,
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        height: usize,
        width: usize,
        iterations: u8,
    ) -> Self {
        let mut colors = vec![
            ColorInfo {
                color_id: 0,
                iterations: 0
            };
            height * width
        ];

        let mut fill_pixel = |x: usize, y: usize, c: ColorInfo| {
            let idx = y * width + x;
            colors[idx] = c;
        };

        for px in 0..width {
            for py in 0..height {
                let x = x1 + (px as f64 / width as f64) * (x2 - x1);
                let y = y1 + (py as f64 / height as f64) * (y2 - y1);
                let c = Self::newton_iteration(x, y, &roots, iterations);
                fill_pixel(px, py, c);
            }
        }

        NewtonFractal { colors }
    }

    fn newton_iteration(x: f64, y: f64, roots: &Vec<Complex>, max_iterations: u8) -> ColorInfo {
        const EPS: f64 = 0.001;
        let mut c = Complex::new(x, y);
        let p = Poly::from_roots(roots.clone());
        let d = p.derivative();

        let mut iter: u8 = 0;
        let mut root_id: u8 = roots.len() as u8;
        while c.abs() > EPS && iter < max_iterations {
            for (i, &root) in roots.iter().enumerate() {
                if (c - root).abs() < EPS {
                    root_id = i as u8;
                    break;
                }
            }
            if root_id != roots.len() as u8 {
                break;
            }
            c = c - p.evaluate(c) / d.evaluate(c);
            iter += 1;
        }

        ColorInfo {
            color_id: root_id,
            iterations: iter,
        }
    }

    pub fn colors(&self) -> *const ColorInfo {
        self.colors.as_ptr()
    }
}
