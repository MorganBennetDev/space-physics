use std::{
    rc::{
        Rc
    }
};



extern crate nalgebra as na;



use na::{
    Vector5
};



mod polytope;



fn main() {
    let pi = polytope::Polytope::<5>::new(vec![Rc::new(Vector5::new(1.0, 2.0, 3.0, 4.0, 5.0))]);

    println!("Hello, world!");
}
