use std::ops::Mul;

use crate::vector::Vector;

#[derive(Debug, PartialEq)]
pub struct CsrMat {
    pub rows            : usize,
    pub columns         : usize,
    pub values          : Vec<f64>,
    pub columns_index   : Vec<usize>,
    pub rows_index      : Vec<usize> 
}

impl Mul<Vector> for CsrMat {
    type Output = Result<Vector, &'static str>;

    fn mul(self, rhs: Vector) -> Self::Output {
        if self.columns != rhs.values.len() {
            Err("Invalid shapes")
        } else {
            let mut result = vec![];
            for i in 0..self.rows_index.len()-1{
                let mut sum = 0.;
                for j in self.rows_index[i]..self.rows_index[i+1] {
                    sum += self.values[j] * rhs.values[self.columns_index[j]];
                }
                result.push(sum)
            }
            Ok(Vector { values : result})
        }
    }
}
