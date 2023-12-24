use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Complex {
    real: f64,
    imag: f64,
}

#[wasm_bindgen]
impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

impl Complex {
    pub fn abs(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn pow(&self, e: usize) -> Self {
        let mut res = Complex::new(1.0, 0.0);
        for _ in 0..e {
            res = res * self.clone();
        }
        res
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.4},{:.4})", self.real, self.imag)?;
        Ok(())
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.real += other.real;
        self.imag += other.imag;
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator = other.real * other.real + other.imag * other.imag;

        Complex {
            real: (self.real * other.real + self.imag * other.imag) / denominator,
            imag: (self.imag * other.real - self.real * other.imag) / denominator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = Complex::new(3.0, 4.0); // 3 + 4i
        let c2 = Complex::new(1.0, 2.0); // 1 + 2i
        assert_eq!(c1 + c2, Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let c1 = Complex::new(3.0, 4.0); // 3 + 4i
        let c2 = Complex::new(1.0, 2.0); // 1 + 2i
        assert_eq!(c1 - c2, Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_mul() {
        let c1 = Complex::new(3.0, 4.0); // 3 + 4i
        let c2 = Complex::new(1.0, 2.0); // 1 + 2i
        assert_eq!(c1 * c2, Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_abs() {
        let c1 = Complex::new(3.0, 4.0); // 3 + 4i
        let magnitude = c1.abs();
        assert_eq!(magnitude, 5.0);
    }
}
