mod abstract_vector;
mod quaternion;
mod vector;
mod matrix;

pub type AbstractVector<const N: usize> = abstract_vector::AbstractVector<N>;
pub type Quaternion = quaternion::Quaternion;
pub type Vector<const N: usize> = vector::Vector<N>;
pub type Matrix<const N: usize, const M: usize> = matrix::Matrix<N, M>;