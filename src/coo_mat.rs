use std::{collections::HashMap, vec};

#[derive(Debug)]
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
