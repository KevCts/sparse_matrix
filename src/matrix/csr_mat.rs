#[derive(Debug, PartialEq)]
pub struct CsrMat {
    pub values : Vec<f64>,
    pub columns_index: Vec<i64>,
    pub rows_index : Vec<i64> 
}
