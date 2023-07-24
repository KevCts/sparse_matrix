use std::ops:: {Add, Sub, Mul};

#[derive(Debug, PartialEq)]
pub struct Vector {
    pub values : Vec<f64>
}

impl Add for Vector {
    type Output = Result<Self, &'static str>;

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

impl Sub for Vector {
    type Output = Result<Self, &'static str>;

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

impl Mul for Vector {
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
