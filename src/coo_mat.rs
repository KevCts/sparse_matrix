use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
pub struct CooMat {
    rows:usize,
    columns:usize,
    coordinates: HashMap<(usize, usize), f32>,
}

impl CooMat {
    pub fn new(rows:usize, columns:usize) -> CooMat {
        CooMat {
            rows,
            columns,
            coordinates: HashMap::new(),
        }
    }
    pub fn add_value(&mut self, i: usize, j: usize, value: f32) -> Result<&mut Self, &'static str> {
        if i >= self.rows || j >= self.columns {
            Err("Index out of bound")
        }
        else {
            if value !=0. {
                self.coordinates
                    .entry((i, j))
                    .and_modify(|a| *a += value)
                    .or_insert(value);
            }
            Ok(self)
        }
    }
    pub fn to_dense(&self) -> Vec<Vec<f32>> {
        let mut dense = vec![];
        for i in 0..self.rows{
            dense.push(vec![]);
            for j in 0..self.columns{
                dense[i].push(self[(i,j)])
            }
        }
        dense
    }

    pub fn transpose (&self) -> CooMat {
        let mut result = CooMat::new(self.columns, self.rows);

        for ((i,j), value) in &self.coordinates {
            result.add_value(*j, *i, *value).expect("An error occured while transposing");
        }

        result
    }

    pub fn to_scalar(&self) -> Result<f32, &str> {
        if self.rows == 1 && self.columns == 1 {
            Ok(self[(0,0)])
        }

        else {
            Err("This matrix isn't a scalar")
        }
    }

    pub fn conjugate_gradient_solve(&self, b:CooMat) -> Result<CooMat, &'static str> {
        if b.rows != self.rows {
            Err("Invalid size for b vector")
        }
        else {
            let mut x = CooMat::new(self.rows, 1);

            for i in 0..self.rows{
                x.add_value(i, 0, 0.).unwrap();
            }

            let mut r = (b.clone() - (self.clone()*x.clone()).unwrap()).unwrap();

            let mut rm1 : CooMat;

            let mut p = r.clone();

            let mut a : f32;

            let mut b : f32;

            for _ in 0..2*self.rows {
                a = ((r.clone().transpose() * r.clone()).unwrap().to_scalar().unwrap()) / ((p.clone().transpose() * (self.clone() * p.clone()).unwrap()).unwrap().to_scalar().unwrap());
                x = (x + p.clone() * a).unwrap();
                rm1 = r.clone();
                r = (r - (self.clone() * p.clone()).unwrap() * a).unwrap();
                b = (r.clone().transpose() * r.clone()).unwrap().to_scalar().unwrap() / ((rm1.clone().transpose() * rm1.clone()).unwrap().to_scalar().unwrap());
                p = (r.clone() + (p * b)).unwrap();
            }

            Ok(x)
        }
    }
}

impl std::ops::Sub<CooMat> for CooMat {
    type Output = Result<CooMat, &'static str>;
    fn sub(self, rhs: CooMat) -> Self::Output {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            Err("Matrix dimensions aren't the same")
        }
        else {

            let mut rescoo = HashMap::new();
            let coo1 = self.coordinates;
            let coo2 = rhs.coordinates;
            for ((i, j), value) in coo1 {
                rescoo.insert((i, j), value);
            }
            for ((i, j), value) in coo2 {
                rescoo
                    .entry((i, j))
                    .and_modify(|a| *a -= value)
                    .or_insert(-1.*value);
            }
            Ok(CooMat {
                rows: self.rows,
                columns: self.columns,
                coordinates: rescoo,
            })
        }
    }
}

impl std::ops::Add<CooMat> for CooMat {
    type Output = Result<CooMat, &'static str>;
    fn add(self, rhs: CooMat) -> Self::Output {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            Err("Matrix dimensions aren't the same")
        }
        else {

            let mut rescoo = HashMap::new();
            let coo1 = self.coordinates;
            let coo2 = rhs.coordinates;
            for ((i, j), value) in coo1 {
                rescoo.insert((i, j), value);
            }
            for ((i, j), value) in coo2 {
                rescoo
                    .entry((i, j))
                    .and_modify(|a| *a += value)
                    .or_insert(value);
            }
            Ok(CooMat {
                rows: self.rows,
                columns: self.columns,
                coordinates: rescoo,
            })
        }
    }
}

impl std::ops::AddAssign for CooMat {
    fn add_assign(&mut self, rhs: Self) {
        let coo = rhs.coordinates;
        for ((i, j), value) in coo {
            match self.add_value(i, j, value) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error : {}", e);
                    break;
                }
            }
        }
    }
}

impl std::ops::Index<(usize, usize)> for CooMat{
    type Output = f32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match self.coordinates.get(&index) {
            Some(value) => value,
            None => &0.,
        }
    }
}

impl std::ops::Mul<CooMat> for CooMat {
    type Output = Result<CooMat, &'static str>;
    fn mul(self, rhs: CooMat) -> Self::Output {
        if self.columns != rhs.rows {
            Err("Matrix dimensions aren't corresponding")
        }
        else {
            let a = self.coordinates;
            let b = rhs.coordinates;

            let mut c = HashMap::new();

            let mut t = HashMap::<usize,Vec<(usize,f32)>>::new();

            for ((i,j),value) in b {
                t.entry(i).and_modify(|x| x.push((j, value))).or_insert(vec![(j,value)]);
            }

            for ((i,j),vala) in a {
                match t.get(&j) {
                    Some(vector) => {
                        for (k, valb) in vector {
                            c.entry((i,*k)).and_modify(|valc| *valc += vala*valb).or_insert(vala*valb);
                        }
                    }
                    None => ()
                }
            }

            Ok(CooMat {
                rows: self.rows,
                columns: rhs.columns,
                coordinates : c 
            })
        }
    }
}

impl std::ops::Mul<Vec<f32>> for CooMat {
    type Output = Result<Vec<f32>, &'static str>;

    fn mul(self, rhs: Vec<f32>) -> Self::Output {
        if rhs.len() != self.rows {
            Err("Invalid vector dimensions")
        }
        else {
            let mut res = vec![0.;self.columns];
            let coo = self.coordinates;

            for ((i, j), a) in coo {
                res[i] += a * rhs[j];
            }

            Ok(res)
        }
    }
}

impl std::ops::Mul<f32> for CooMat {
    type Output = CooMat;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = CooMat::new(self.rows, self.columns);
        for ((i, j), value) in self.coordinates {
            result.add_value(i, j, rhs * value).unwrap();
        }
        result
    }
}
