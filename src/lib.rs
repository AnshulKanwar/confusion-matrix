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
            rows: rows
        }
    }

    pub fn print(&self) {
        for row in &self.rows {
            for col in row {
                print!("{} ", col);
            }
            println!("");
        }
    }
}