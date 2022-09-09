use std::ops::Index;

pub struct Matrix {
    pub size: usize,
    pub rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(rows: Vec<Vec<u32>>) -> Self {
        let n_rows = rows.len();
        let n_cols = rows[0].len();

        if n_rows != n_cols {
            panic!("Not a square matrix")
        }

        Matrix {
            size: n_rows,
            rows: rows,
        }
    }
}

impl Index<u32> for Matrix {
    type Output = Vec<u32>;
    fn index(&self, index: u32) -> &Self::Output {
        &self.rows[index as usize]
    }
}

impl Matrix {
    pub fn print(&self) {
        for row in &self.rows {
            for col in row {
                print!("{} ", col);
            }
            println!("");
        }
    }
}

impl Matrix {
    pub fn tp(&self) -> u32 {
        self[0][0]
    }
    pub fn tn(&self) -> u32 {
        self[1][1]
    }
    pub fn fp(&self) -> u32 {
        self[0][1]
    }
    pub fn fn_(&self) -> u32 {
        self[1][0]
    }
}

impl Matrix {
    pub fn accuracy(&self) -> f32 {
        (self.tp() + self.tn()) as f32 / (self.tp() + self.tn() + self.fp() + self.fn_()) as f32
    }
    pub fn precision(&self) -> f32 {
        self.tp() as f32 / (self.tp() + self.fp()) as f32
    }
    pub fn sensitivity(&self) -> f32 {
        self.tp() as f32 / (self.tp() + self.fn_()) as f32
    }
    pub fn tpr(&self) -> f32 {
        self.tn() as f32 / (self.tn() + self.fp()) as f32
    }
    pub fn fpr(&self) -> f32 {
        1. - self.specificity()
    }
    pub fn recall(&self) -> f32 {
        self.tpr()
    }
    pub fn specificity(&self) -> f32 {
        self.tpr()
    }
    pub fn f1_score(&self) -> f32 {
        (2 * self.tp()) as f32 / ((2 * self.tp()) + self.fp() + self.fn_()) as f32
    }
}