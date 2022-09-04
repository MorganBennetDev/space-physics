use std::rc::{
    Rc
};



use crate::vectors::{
    Vector
};



pub struct Complex<const N: usize> {
    vertices: Vec<Vector<N>>,
    simplices: Vec<Simplex<N>>
}

impl<const N: usize> Complex <N>{
    
}



fn rank_check<const N: usize>(columns: &Vec<Rc<Vector<N>>>, rank_target: usize) -> bool {
    let mut rank = 0;

    let mut cols: Vec<Vector<N>> = columns.iter().map(|v| (**v).clone()).collect();

    for i in 0..cols.len() {
        if cols[i][i] == 0.0 {
            let mut k = i + 1;

            while k < cols.len() && cols[k][i] == 0.0 {
                k += 1;
            }

            let temp = cols[i];
            cols[i] = cols[k];
            cols[k] = temp;
        }

        for j in i + 1..cols.len() {
            let new = cols[i] * cols[j][i] / cols[i][i];

            cols[j] -= new;
        }

        rank += 1;

        if rank > rank_target {
            return false;
        }
    }

    return rank == rank_target;
}



// N is the dimension of the containing space
pub struct Simplex<const N: usize> {
    vertices: Vec<Rc<Vector<N>>>
}


// Can't get the faces because doing arithmetic on const generics was too hard to implement in the compiler ig
impl<const N: usize> Simplex<N> {
    // Vertices are ordered counterclockwise
    pub fn new(vertices: Vec<Rc<Vector<N>>>) -> Simplex<N> {
        if vertices.len() > N {
            panic!("Too many vertices for a simplex of dimension {}", N);
        }

        if vertices.len() == 1 {
            return Simplex { vertices }
        }

        // Make sure rank is equal to the number of vertices
        if !rank_check(&vertices, vertices.len()) {
            panic!("Vertices are not linearly independent");
        }



        Simplex { vertices }
    }

    pub fn faces(&self) -> Vec<Simplex<N>> {
        let mut faces: Vec<Simplex<N>> = Vec::new();

        for i in 0..self.vertices.len() {
            let mut face_vertices = self.vertices.clone();
            face_vertices.remove(i);

            faces.push(Simplex::new(face_vertices));
        }

        faces
    }

    pub fn dimension(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn intersects(u: Simplex<N>, v: Simplex<N>) -> bool {
        let u_dim = u.dimension();
        let v_dim = v.dimension();

        if u_dim <= v_dim {
            Simplex::intersects_simple(u, v, v_dim)
        } else {
            Simplex::intersects_simple(v, u, u_dim)
        }
    }

    // Intersection where the dimension of u is guaranteed to be less than or equal to the dimension of v which is equal to k
    fn intersects_simple(u: Simplex<N>, v: Simplex<N>, k: usize) -> bool {
        // Determine if u is contained in the plane defined by v
        for p in u.vertices {
            if !v.point_in_hyperplane(&p) {
                return false;
            }
        }



        // Project to normals of faces of v within the linear subspace spanned by v's vertices to determine intersection

        false
    }

    fn point_in_hyperplane(&self, p: &Vector<N>) -> bool {
        let mut cols = self.vertices.clone();
        cols.push(Rc::new(*p));

        rank_check(&cols, self.vertices.len())
    }
}