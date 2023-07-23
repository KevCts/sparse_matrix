use std::collections::HashMap;

use super::csr_mat::CsrMat;

#[derive(Debug, PartialEq)]
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
        self.values.insert((row, col), value);
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
        if keys.len() != 0 {
            result.rows_index.push(non_null_so_far);
        }
        result
    }
}
