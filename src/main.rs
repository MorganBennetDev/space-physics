mod vectors;
mod util;

use crate::vectors::{
    Quaternion,
    Vec3
};


struct Box {
    position: Quaternion,
    size: Vec3
}

fn main() {
    println!("Hello, world!");
}
