mod abstract_vector;
mod quaternion;
mod vector;

pub type AbstractVector<const N: usize> = abstract_vector::AbstractVector<N>;
pub type Quaternion = quaternion::Quaternion;
pub type Vector<const N: usize> = vector::Vector<N>;