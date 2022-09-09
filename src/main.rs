use nalgebra::matrix;
use confusion_matrix::get_tp;
use confusion_matrix::get_tn;
use confusion_matrix::get_fp;
use confusion_matrix::get_fn;

fn main() {
    let a = matrix![ 1, 2;
                     3, 4];

    println!("True Positive = {}", get_tp(&a));
    println!("True Negative = {}", get_tn(&a));
    println!("False Positive = {}", get_fp(&a));
    println!("False Negative = {}", get_fn(&a));
}