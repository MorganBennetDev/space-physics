mod abstract_vector;
mod quaternion;
mod vector;
mod linear_map;

pub type AbstractVector<const N: usize> = abstract_vector::AbstractVector<N>;
pub type Quaternion = quaternion::Quaternion;
pub type Vector<const N: usize> = vector::Vector<N>;
pub type LinearMap<const R: usize, const C: usize> = linear_map::LinearMap<R, C>;