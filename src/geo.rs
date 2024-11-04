use glam::{Affine3A, Vec3};

use crate::{
    buffer::Buffer,
    color::Material,
    poly::{self, Tri},
};

pub trait Shape {
    fn render(&self, buffer: &mut Buffer, transform: Affine3A);
}

type Vertex = Vec3;

#[derive(Debug)]
pub struct IndexedTriangle<'a> {
    /// indices that correspond to vertiecs in Mesh
    pub index: (usize, usize, usize),
    pub color: &'a Material, // TODO change all these to materials
}

#[derive(Debug)]
pub struct Mesh<'a> {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<IndexedTriangle<'a>>,
}

impl Shape for Mesh<'_> {
    fn render(&self, buffer: &mut Buffer, transform: Affine3A) {
        let transformed_verts: Vec<Vertex> = (&self.vertices)
            .into_iter()
            .map(|v| transform.transform_point3(*v))
            .collect();

        for triangle in &self.triangles {
            let (t1, t2, t3) = triangle.index;
            let vert_tri = Tri::new(
                transformed_verts[t1],
                transformed_verts[t2],
                transformed_verts[t3],
                triangle.color,
            );
            poly::draw_tri(buffer, &vert_tri);
        }
    }
}

pub struct Geo {
    pub shape: Box<dyn Shape>,
    pub transform: Affine3A,
    _children: Vec<Geo>,
}

impl Geo {
    pub fn new(shape: Box<dyn Shape>, transform: Affine3A) -> Geo {
        Geo {
            shape,
            transform,
            _children: Vec::new(),
        }
    }
    pub fn render(&self, buffer: &mut Buffer) {
        self.shape.render(buffer, self.transform);
    }
}
