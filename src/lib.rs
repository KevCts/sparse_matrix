pub mod matrix;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::matrix::{coo_mat::{new_coo_mat,CooMat}, csr_mat::CsrMat};

    #[test]
    fn create_coo_mat() {
        let mat = new_coo_mat(3, 3);
        assert_eq!(mat, CooMat { rows : 3, columns : 3, values: HashMap::new() })
    }

    #[test]
    fn void_coo_to_csr() {
        let mat = new_coo_mat(3, 3);
        assert_eq!(mat.to_csr(),CsrMat { values : vec![], columns_index : vec![], rows_index : vec![0] });
    }

    #[test]
    fn coo_to_csr() {
        let mut mat = new_coo_mat(3, 3);
        mat.add(0, 0, 1.);
        mat.add(0, 1, 2.);
        mat.add(1, 0, 3.);
        mat.add(1, 1, 4.);
        assert_eq!(mat.to_csr(),CsrMat { values : vec![1., 2., 3., 4.], columns_index : vec![0,1,0,1], rows_index : vec![0,2,4] });
    }
}
