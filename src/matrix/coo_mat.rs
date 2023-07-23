use std::collections::HashMap;

use super::csr_mat::CsrMat;

#[derive(Debug, PartialEq)]
pub struct CooMat {
    pub rows    : i64,
    pub columns  : i64,
    pub values  : HashMap<(i64, i64), f64> 
}

pub fn new_coo_mat(r : i64, c : i64) -> CooMat {
    CooMat { rows: r, columns: c, values: HashMap::new() }
}

impl CooMat {
    pub fn add( &mut self, row : i64, col : i64, value : f64 ){
        self.values.insert((row, col), value);
    }

    pub fn to_csr(self) -> CsrMat{
        let mut keys : Vec<&(i64,i64)> = self.values.keys().collect();
        keys.sort_unstable();
        let mut result = CsrMat { values : vec![], columns_index : vec![], rows_index : vec![] };
        let mut current_row = -1;
        let mut non_null_so_far = 0;
        for (r, c) in keys {
            if *r != current_row {
                result.rows_index.push(non_null_so_far);
                current_row = *r;
            }
            result.columns_index.push(*c);
            result.values.push(*self.values.get(&(*r,*c)).unwrap());
            non_null_so_far += 1;
        }
        result.rows_index.push(non_null_so_far);
        result
    }
}
