mod quaternion;
mod vector;

pub type Quaternion = quaternion::Quaternion;
pub type Vector<const N: usize> = vector::Vector<N>;