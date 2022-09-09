use confusion_matrix::Matrix;

fn main() {
    let rows = vec![vec![1, 2], vec![3, 4]];

    let a = Matrix::new(rows);

    println!("{}", a.f1_score());
}
