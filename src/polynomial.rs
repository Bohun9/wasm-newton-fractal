use super::complex::Complex;
use std::fmt;
use std::ops::Mul;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Poly {
    a: Vec<Complex>,
}

#[wasm_bindgen]
impl Poly {
    pub fn from_roots(roots: Vec<Complex>) -> Self {
        let mut p = Self::from_coef(vec![Complex::new(1.0, 0.0)]);
        for &root in roots.iter() {
            p = p * Self::from_coef(vec![-root, Complex::new(1.0, 0.0)]);
        }
        p
    }

    pub fn to_str(&self) -> String {
        self.to_string()
    }
}

impl Poly {
    pub fn from_coef(a: Vec<Complex>) -> Self {
        Poly { a }
    }

    pub fn evaluate(&self, x: Complex) -> Complex {
        let mut y = Complex::new(0.0, 0.0);
        for (i, &a) in self.a.iter().enumerate() {
            y += a * x.pow(i);
        }
        y
    }

    pub fn derivative(&self) -> Self {
        let mut b = self.a.iter().enumerate();
        b.next();
        let b = b.map(|(i, &c)| c * Complex::new(i as f64, 0.0)).collect();
        Poly::from_coef(b)
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, c) in self.a.iter().enumerate().rev() {
            write!(f, "{}x^{}", c, i)?;
            if i > 0 {
                write!(f, " + ")?;
            }
        }

        Ok(())
    }
}

impl Mul for Poly {
    type Output = Poly;

    fn mul(self, o: Self) -> Self {
        let mut b = vec![Complex::new(0.0, 0.0); self.a.len() + o.a.len() - 1];
        for (i, &c1) in self.a.iter().enumerate() {
            for (j, &c2) in o.a.iter().enumerate() {
                b[i + j] += c1 * c2;
            }
        }
        Poly::from_coef(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_coef() {
        let p = Poly::from_coef(vec![
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
        ]);
        assert_eq!(p.evaluate(Complex::new(2.0, 0.0)), Complex::new(4.0, 0.0));
    }

    #[test]
    fn test_from_roots() {
        let p = Poly::from_roots(vec![Complex::new(-1.0, 0.0), Complex::new(1.0, 0.0)]);
        assert_eq!(p.evaluate(Complex::new(2.0, 0.0)), Complex::new(3.0, 0.0));
    }
}
