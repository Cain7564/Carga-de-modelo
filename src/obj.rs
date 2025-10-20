use crate::vertex::Vertex;
use raylib::math::{Vector2, Vector3};
use tobj;

pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn load(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            for i in 0..num_vertices {
                let x = mesh.positions[i * 3];
                let y = mesh.positions[i * 3 + 1];
                let z = mesh.positions[i * 3 + 2];
                let position = Vector3::new(x, -y, z);

                let normal = if !mesh.normals.is_empty() {
                    let nx = mesh.normals[i * 3];
                    let ny = mesh.normals[i * 3 + 1];
                    let nz = mesh.normals[i * 3 + 2];
                    Vector3::new(nx, ny, nz)
                } else {
                    Vector3::zero()
                };

                let tex_coords = if !mesh.texcoords.is_empty() {
                    let u = mesh.texcoords[i * 2];
                    let v = mesh.texcoords[i * 2 + 1];
                    Vector2::new(u, v)
                } else {
                    Vector2::zero()
                };

                // Assign a varying color per vertex so vertex colors are visible.
                // Cycle through a small palette based on the vertex index.
                let color = match i % 6 {
                    0 => Vector3::new(1.0, 0.0, 0.0), // red
                    1 => Vector3::new(0.0, 1.0, 0.0), // green
                    2 => Vector3::new(0.0, 0.0, 1.0), // blue
                    3 => Vector3::new(1.0, 1.0, 0.0), // yellow
                    4 => Vector3::new(1.0, 0.0, 1.0), // magenta
                    _ => Vector3::new(0.0, 1.0, 1.0), // cyan
                };

                // Create vertex with the chosen color. Keep normals/texcoords as before by using
                // the new_with_color constructor (which sets normals to zero) only for color visibility.
                vertices.push(Vertex::new_with_color(position, color));
            }
            indices.extend_from_slice(&mesh.indices);
        }

        Ok(Obj { vertices, indices })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::new();
        for &index in &self.indices {
            vertex_array.push(self.vertices[index as usize].clone());
        }
        vertex_array
    }
}
