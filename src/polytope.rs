use std::{
    ops::{
        Index
    },
    rc::{
        Rc
    }
};



extern crate nalgebra as na;



use na::{
    SVector
};

pub struct Polytope<const SpaceDimension: usize> {
    vertices: Vec<Rc<SVector<f64, SpaceDimension>>>
}



impl<const SpaceDimension: usize> Polytope<SpaceDimension> {
    pub fn new(vertices: Vec<Rc<SVector<f64, SpaceDimension>>>) -> Self {
        Self {
            vertices: vertices
        }
    }

    pub fn dimension(&self) -> usize {
        self.vertices.len()
    }

    pub fn faces(&self) -> Vec<Polytope<SpaceDimension>> {
        let mut faces = Vec::new();

        for i in 0..self.vertices.len() {
            let mut face_vertices = Vec::new();

            for j in 0..self.vertices.len() {
                if i != j {
                    face_vertices.push(self.vertices[j].clone());
                }
            }

            faces.push(Polytope::new(face_vertices));
        }

        faces
    }

    pub fn vertices(&self) -> &Vec<Rc<SVector<f64, SpaceDimension>>> {
        &self.vertices
    }
}



impl<const SpaceDimension: usize> Index<usize> for Polytope<SpaceDimension> {
    type Output = Rc<SVector<f64, SpaceDimension>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}