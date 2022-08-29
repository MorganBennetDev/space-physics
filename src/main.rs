use std::ops;



struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}



impl Vec3 {

}



impl Add for Vec3 {

}

impl AddAssign for Vec3 {

}

impl Div for Vec3 {

}

impl DivAssign for Vec3 {

}

impl Index for Vec3 {

}

impl IndexMut for Vec3 {

}

impl Mul for Vec3 {

}

impl MulAssign for Vec3 {

}

impl Neg for Vec3 {

}

impl Sub for Vec3 {

}

impl SubAssign for Vec3 {

}



struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64
}

struct Box {
    position: Quaternion,
    size: Vec3
}



fn main() {
    println!("Hello, world!");
}
