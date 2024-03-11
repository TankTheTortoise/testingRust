use std::any::*;
use nalgebra as na;
use nalgebra::DMatrix;

/*

F(x) = wx+b
E(y1, y2) = (y1-y2)^2

dE = E'(F(x), answers)

dF/dw = x
dF/db = 1

w = w-a*dE/dw
b = b-E'(end)*a

*/

// fn forward()


pub fn mul(x: i32, w: i32) -> i32{
    return w*x;
}

pub fn add(x: i32, b: i32) -> i32 {
    return x+b;
}

pub fn mul_d(x: i32, w: i32) -> i32{
    return x;
}

pub fn add_d(x: i32, b: i32) -> i32 {
    return 1;
}


fn main() {

    let col = 2;
    let row = 2;
    let mut X = DMatrix::from_vec(row, col, vec![1, 2,
                                                                      3, 4]);

    for i in 0..row{
        for j in 0..col{
            println!("{}", add(X[(i, j)], 10))
        }
    }
    X = X.add_scalar(10);
    println!("{}", X[0]);
}
