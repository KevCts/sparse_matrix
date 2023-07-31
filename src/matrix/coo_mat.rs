use std::collections::HashMap;

use super::csr_mat::CsrMat;

#[derive(Debug, PartialEq, Clone)]
pub struct CooMat {
    pub rows    : usize,
    pub columns  : usize,
    pub values  : HashMap<(usize, usize), f64> 
}

impl CooMat {
    pub fn new(r : usize, c : usize) -> CooMat{
        CooMat { rows: r, columns: c, values: HashMap::new() }
    }

    pub fn add( &mut self, row : usize, col : usize, value : f64 ){
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
            self.drop(col, i);
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

    pub fn to_csr(self) -> CsrMat{
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
