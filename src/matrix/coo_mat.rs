use std::{collections::HashMap, ops::{Add, AddAssign, Mul}};

use super::csr_mat::CsrMat;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CooMat {
    pub rows    : usize,
    pub columns  : usize,
    pub values  : HashMap<(usize, usize), f64> 
}

impl Add for &CooMat {
    type Output = CooMat;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        for ((r, c), value) in &other.values {
            result.add_value(*r, *c, *value);
        }
        result
    }
}

impl Mul for &CooMat {
    type Output = CooMat;

    fn mul(self, other: Self) -> Self::Output {
        let r1 = self.rows;
        let c1r1 = self.columns;
        let c2 = other.columns;
        let mut result = CooMat::new(r1, c2);

        for i in 0..r1 {
            for j in 0..c2 {
                let mut acc = 0.;
                for k in 0..c1r1 {
                    acc +=
                    match &self.values.get(&(i,k)) {
                        Some(&a) => 
                        match &other.values.get(&(k,j)) {
                            Some(&b) => a * b,
                            _ => 0.,
                        }
                        _ => 0.,
                    };
                }
                result.add_value(i,j,acc);
            }
        }

        result
    }
}

impl AddAssign for CooMat {
    fn add_assign(&mut self, other: Self) {
        for ((r, c), value) in &other.values {
            self.add_value(*r, *c, *value);
        }
    }
}

impl CooMat {
    pub fn new(r : usize, c : usize) -> CooMat{
        CooMat { rows: r, columns: c, values: HashMap::new() }
    }

    pub fn add_value( &mut self, row : usize, col : usize, value : f64 ){
        if value != 0. {
            self.values.entry((row, col)).and_modify(|x| *x += value).or_insert(value);
        }
    }

    pub fn drop(&mut self, row : usize, col : usize){
        if self.values.contains_key(&(row, col)){
            self.values.remove(&(row, col));
        }
    }

    pub fn drop_row(&mut self, row : usize){
        for i in 0..self.columns {
            self.drop(row, i);
        }
        for i in row..self.rows{
            for j in 0..self.columns{
                if self.values.contains_key(&(i, j)){
                    self.values.insert((i-1, j), self.values[&(i, j)]);
                    self.values.remove(&(i, j));
                }
            }
        }
    }

    pub fn drop_col(&mut self, col: usize){
        for i in 0..self.rows {
            self.drop(i, col);
        }
        for j in col..self.columns{
            for i in 0..self.rows{
                if self.values.contains_key(&(i, j)){
                    self.values.insert((i, j-1), self.values[&(i, j)]);
                    self.values.remove(&(i, j));
                }
            }
        }
    }

    pub fn to_csr(&self) -> CsrMat{
        let mut keys : Vec<&(usize,usize)> = self.values.keys().collect();
        keys.sort_unstable();
        let mut result = CsrMat { rows : self.rows, columns : self.columns, values : vec![], columns_index : vec![], rows_index : vec![0] };
        let mut current_row = 0;
        let mut non_null_so_far = 0;
        for (r, c) in &keys {
            while current_row < *r {
                result.rows_index.push(non_null_so_far);
                current_row += 1;
            }
            result.columns_index.push(*c);
            result.values.push(*self.values.get(&(*r,*c)).unwrap());
            non_null_so_far += 1;
        }
        if !keys.is_empty() {
            result.rows_index.push(non_null_so_far);
        }
        result
    }
}
