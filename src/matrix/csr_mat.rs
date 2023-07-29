use std::ops::Mul;

use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct CsrMat {
    pub rows            : usize,
    pub columns         : usize,
    pub values          : Vec<f64>,
    pub columns_index   : Vec<usize>,
    pub rows_index      : Vec<usize> 
}

impl CsrMat {
    pub fn minres(self, b : Vector, relative_eps : f64) -> Result<Vector, &'static str> {
        let mut r = b.clone();
        let eps = relative_eps * b.norm();
        let mut x = Vector::null(b.len());
        let mut alpha;
        let mut ar;
        let mut arnorm;
        while r.norm() > eps {
            ar = (&self * &r)?;
            arnorm = ar.norm();
            println!("{r:?}");
            println!("{ar:?}");
            alpha = (&r * &ar)? / arnorm / arnorm;
            x = (&x + &(&r * alpha))?;
            r = (&b - &(&self * &x)?)?;
        }
        Ok(x)
    }
}

impl Mul<&Vector> for &CsrMat {
    type Output = Result<Vector, &'static str>;

    fn mul(self, rhs: &Vector) -> Self::Output {
        if self.columns != rhs.values.len() {
            Err("Invalid shapes")
        } else {
            let mut result = vec![0.; self.columns];
            for i in 0..self.rows_index.len()-1{
                let mut sum = 0.;
                for j in self.rows_index[i]..self.rows_index[i+1] {
                    sum += self.values[j] * rhs.values[self.columns_index[j]];
                }
                result[i] = sum;
            }
            Ok(Vector { values : result})
        }
    }
}
