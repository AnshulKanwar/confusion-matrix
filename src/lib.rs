use nalgebra::ArrayStorage;
use nalgebra::base::{Matrix, Const};

pub fn get_tp(mat: &Matrix<i32, Const<2>, Const<2>, ArrayStorage<i32, 2, 2>>) -> i32 {
    return mat.m11;
}

pub fn get_tn(mat: &Matrix<i32, Const<2>, Const<2>, ArrayStorage<i32, 2, 2>>) -> i32 {
    return mat.m22;
}

pub fn get_fp(mat: &Matrix<i32, Const<2>, Const<2>, ArrayStorage<i32, 2, 2>>) -> i32 {
    return mat.m12;
}

pub fn get_fn(mat: &Matrix<i32, Const<2>, Const<2>, ArrayStorage<i32, 2, 2>>) -> i32 {
    return mat.m21;
}