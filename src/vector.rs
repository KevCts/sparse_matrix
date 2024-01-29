use std::ops:: {Add, Sub, Mul};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Vector {
    pub values : Vec<f64>
}

impl Vector {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    pub fn norm(&self) -> f64 {
        let mut sum = 0.;
        for x in self.values.iter() {
            sum += x*x;
        }
        sum.sqrt()
    }

    pub fn null(n : usize) -> Vector {
        Vector { values : vec![0.;n] }
    }
}

impl Add for &Vector {
    type Output = Result<Vector, &'static str>;

    fn add(self, other: Self) -> Self::Output {
        if self.values.len() == other.values.len() {
            let mut result = vec![];
            for i in 0..self.values.len() {
                result.push(self.values[i] + other.values[i])
            }
            Ok(Vector { values: result })
        } else {
            Err("Wrong shapes")
        }
    }
}

impl Sub for &Vector {
    type Output = Result<Vector, &'static str>;

    fn sub(self, other: Self) -> Self::Output {
        if self.values.len() == other.values.len() {
            let mut result = vec![];
            for i in 0..self.values.len() {
                result.push(self.values[i] - other.values[i])
            }
            Ok(Vector { values: result })
        } else {
            Err("Wrong shapes")
        }
    }
}

impl Mul for &Vector {
    type Output = Result<f64, &'static str>;

    fn mul(self, other: Self) -> Self::Output {
        if self.values.len() == other.values.len() {
            let mut result = 0.;
            for i in 0..self.values.len() {
                result += self.values[i] * other.values[i];
            }
            Ok(result)
        } else {
            Err("Wrong shapes")
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Self::Output {
        let mut result = self.clone();
        for i in 0..self.len() {
            result.values[i] *= other;
        }
        result
    }
}
