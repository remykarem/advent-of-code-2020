#[macro_use]
extern crate ndarray;

use ndarray::prelude::*;

fn main() {
    let X = array![[1, 2, 3, 4, 5], [1, 2, 3, 4, 5],];
    println!("{}", X);

    let mut Y = X.clone();

    Y.indexed_iter_mut()
        .zip(X.indexed_iter())
        // .for_each(|((pos, y), (_, x))| {
        //     let (i,j) = pos;
        //     *y = *x + 1;
        // });
    println!("{}", Y);
}
