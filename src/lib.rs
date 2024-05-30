pub mod matrix;
pub mod vector;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{matrix::{coo_mat::CooMat, csr_mat::CsrMat}, vector::Vector};

    #[test]
    fn create_coo_mat() {
        let mat = CooMat::new(3, 3);
        assert_eq!(mat, CooMat { rows : 3, columns : 3, values: HashMap::new() })
    }

    #[test]
    fn void_coo_to_csr() {
        let mat = CooMat::new(3, 3);
        assert_eq!(mat.to_csr(),CsrMat { rows : 3, columns : 3, values : vec![], columns_index : vec![], rows_index : vec![0] });
    }

    #[test]
    fn coo_to_csr() {
        let mut mat = CooMat::new(2, 2);
        mat.add_value(1, 0, 3.);
        mat.add_value(1, 1, 4.);
        mat.add_value(0, 0, 1.);
        mat.add_value(0, 1, 2.);
        assert_eq!(mat.to_csr(),CsrMat { rows : 2, columns : 2, values : vec![1., 2., 3., 4.], columns_index : vec![0,1,0,1], rows_index : vec![0,2,4] });
    }

    #[test]
    fn matrix_vector_product() {
        let mut mat = CooMat::new(2, 2);
        mat.add_value(0, 0, 1.);
        mat.add_value(0, 1, 2.);
        mat.add_value(1, 0, 3.);
        mat.add_value(1, 1, 4.);
        let csrmat = mat.to_csr();
        let res = &csrmat * &Vector { values : vec![0.,1.]};
        assert_eq!(res, Ok(Vector { values : vec![2., 4.]}))
    }

    #[test]
    fn add_vectors() {
        let vec1 = Vector {values : vec![1.,1.]};
        let vec2 = Vector {values : vec![1.,2.]};
        assert_eq!(&vec1 + &vec2, Ok(Vector{values : vec![2., 3.]}))
    }

    #[test]
    fn sub_vectors() {
        let vec1 = Vector {values : vec![1.,1.]};
        let vec2 = Vector {values : vec![1.,2.]};
        assert_eq!(&vec1 - &vec2, Ok(Vector{values : vec![0., -1.]}))
    }

    #[test]
    fn mul_vectors() {
        let vec1 = Vector {values : vec![1.,1.]};
        let vec2 = Vector {values : vec![1.,2.]};
        assert_eq!(&vec1 * &vec2, Ok(3.))
    }

    #[test]
    fn vector0() {
        assert_eq!(Vector::null(2), Vector { values : vec![0.;2] })
    }

    #[test]
    fn minres_solver() {
        let mut a = CooMat::new(1000000, 1000000);
        for i in 0..1000000{
            a.add_value(i, i, 2.);
        }
        let a = a.to_csr();
        let b = Vector { values : vec![1.;1000000] };
        let res = &a.minres(&b, 0.01).unwrap();
        assert_eq!(*res,Vector { values : vec![0.5;1000000] })
    }

    #[test]
    fn removing_a_value_from_a_coomat() {
        let mut mat = CooMat::new(2, 2);
        mat.add_value(1, 1, 123.);
        mat.drop(1, 1);
        assert_eq!(mat, CooMat::new(2, 2))
    }

    #[test]
    fn matrix_multiplication(){
        let mut mat1 = CooMat::new(2,2);
        mat1.add_value(1, 0, 3.);
        mat1.add_value(1, 1, 4.);
        mat1.add_value(0, 0, 1.);
        mat1.add_value(0, 1, 2.);
        let mut mat2 = CooMat::new(2,2);
        mat2.add_value(0,0,1.);
        mat2.add_value(1,1,1.);
        assert_eq!(&mat1 * &mat2, mat1)
    }

    #[test]
    fn matrix_transposition(){
        let mut mat1 = CooMat::new(2,2);
        mat1.add_value(1, 0, 3.);
        mat1.add_value(1, 1, 4.);
        mat1.add_value(0, 0, 1.);
        mat1.add_value(0, 1, 2.);
        let mut mat2 = CooMat::new(2,2);
        mat2.add_value(0, 1, 3.);
        mat2.add_value(1, 1, 4.);
        mat2.add_value(0, 0, 1.);
        mat2.add_value(1, 0, 2.);
        assert_eq!(mat1.transposed(), mat2);
    }
}
