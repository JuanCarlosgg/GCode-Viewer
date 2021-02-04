use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};

use crate::gcode_plugin::PolyElement;

pub struct Poly {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub points: Vec<PolyElement>,
    pub origen: Vec<(f32, f32, f32)>,
}

impl Poly {
    pub fn new(
        x_length: f32,
        z_length: f32,
        y_lengths: &Vec<PolyElement>,
        origen: &Vec<(f32, f32, f32)>,
    ) -> Poly {
        Poly {
            min_x: -x_length / 2.0,
            max_x: x_length / 2.0,
            min_z: -z_length / 2.0,
            max_z: z_length / 2.0,
            points: y_lengths.clone(),
            origen: origen.clone(),
        }
    }
}


impl From<Poly> for Mesh {
    fn from(sp: Poly) -> Self {
        let mut vert: Vec<[f32; 3]> = vec![];
        let mut norm: Vec<[f32; 3]> = vec![];
        let mut indices = vec![];


        let mut polys : Vec<Vec<Vec3>> = vec![];
        sp.points.iter().for_each(|element| {
            match *element {
                PolyElement::Change => {
                    polys.push(vec![]);
                },
                PolyElement::Point(x_, y_, z_) => {
                    if polys.is_empty() { polys.push(vec![Vec3::new(x_, y_, z_)]); }
                    else { polys.last_mut().unwrap().push(Vec3::new(x_, y_, z_)) };
                }
            }
        });


        for (i, last) in sp.origen.iter().enumerate() {
            let mut last = Vec3::new(last.0, last.1, last.2);
            let y = &polys[i];

            let idx = vert.len();

            //Back
            indices.extend_from_slice(&[idx, idx +1, idx+2,idx+ 2, idx+3, idx+0]);
            

            vert.extend_from_slice(&[
                //Abajo derecha
                [last.x + sp.max_x, last.y, last.z + sp.min_z],
                //Abajo izquierda
                [last.x + sp.min_x, last.y, last.z + sp.min_z],
                //Arriba izquierda
                [last.x + sp.min_x, last.y, last.z + sp.max_z],
                //Arriba derecha
                [last.x + sp.max_x, last.y, last.z + sp.max_z],
            ]);

            norm.extend_from_slice(&[[0., -1.0, 0.], [0., -1.0, 0.], [0., -1.0, 0.], [0., -1.0, 0.]]);

            for distances in y.iter() {


                let Poly {    
                    min_x,
                    mut max_x,
                    mut min_z,
                    mut max_z, 
                    ..} = sp;

                let idx = vert.len();

                let ( next_x,  next_y,  next_z) = (distances.x, distances.y, distances.z);

                let mut y_extra = 0.0;
                if next_y == last.y {
                max_x = min_x;
                max_z = min_z; 
                min_z = sp.max_z;
                y_extra= sp.max_x*2.0;
                }

                //Front.
                indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 2, idx + 3, idx]);
                vert.extend_from_slice(&[
                    //Abajo derecha
                    [next_x + max_x, next_y, next_z + min_z],
                    //Abajo izquierda
                    [next_x + min_x, next_y, next_z + sp.min_z],
                    //Arriba izquierda
                    [next_x + min_x, next_y + y_extra, next_z + max_z],
                    //Arriba derecha
                    [next_x + max_x, next_y + y_extra, next_z + sp.max_z],
                ]);
                norm.extend_from_slice(&[ [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0],  [1.0, 1.0, 1.0], ]);
                //norm.extend_from_slice(&[[0., 1.0, 0.], [0., 1.0, 0.], [0., 1.0, 0.], [0., 1.0, 0.]]);
                //uvs.extend_from_slice(&[[1.0, 0.], [0., 0.], [0.0, 1.0], [1.0, 1.0]]);

                    
                //indices.extend_from_slice(&[idx + 6, idx + 5, idx + 4, idx + 4, idx + 7, idx + 6]); //LEFT
                indices.extend_from_slice(&[idx + 5, idx + 4, idx + 7, idx + 7, idx + 6, idx + 5]); //LEFT
                vert.extend_from_slice(&[
                    //Abajo viejo izquierda
                    [last.x + min_x, last.y, last.z + sp.min_z],
                    //Abajo nuevo izquierda
                    [next_x + min_x, next_y, next_z + sp.min_z],
                    //Arriba nuevo izquierda
                    [next_x + min_x, next_y + y_extra, next_z + max_z],
                    //Arriba viejo izquierda
                    [last.x + min_x, last.y + y_extra, last.z + max_z],
                ]);
                norm.extend_from_slice(&[ [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0],  [1.0, 1.0, 1.0], ]);

                //norm.extend_from_slice(&[[-1.0, 0., 0.], [-1.0, 0., 0.], [-1.0, 0., 0.], [-1.0, 0., 0.], ]);
                //uvs.extend_from_slice(&[[1.0, 0.], [0.0, 0.], [0.0, 1.0], [1.0, 1.0]]);
                
                
                indices.extend_from_slice(&[idx + 9, idx + 8, idx + 11, idx + 11, idx + 10, idx + 9]); //RIGHT
                vert.extend_from_slice(&[
                    //Abajo nuevo derecha
                    [next_x + max_x, next_y, next_z + min_z],
                    //Abajo viejo derecha
                    [last.x + max_x, last.y, last.z + min_z],
                    //Arriba viejo derecha
                    [last.x + max_x, last.y + y_extra, last.z + sp.max_z],
                    //Arriba nuevo derecha
                    [next_x + max_x, next_y + y_extra, next_z + sp.max_z],
                ]);
                norm.extend_from_slice(&[ [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0],  [1.0, 1.0, 1.0], ]);

                //norm.extend_from_slice(&[ [-1.0, 0., 0.], [-1.0, 0., 0.], [-1.0, 0., 0.],  [-1.0, 0., 0.], ]);
                //uvs.extend_from_slice(&[[0., 0.], [1.0, 0.], [1.0, 1.0], [0., 1.0]]);

                
                indices.extend_from_slice(&[ idx + 14, idx + 12, idx + 13, idx + 13, idx + 15, idx + 14, ]); //TOP
                vert.extend_from_slice(&[
                    //Arriba viejo izquierda
                    [last.x + min_x, last.y + y_extra, last.z + max_z],
                    //Arriba viejo derecha
                    [last.x + max_x, last.y + y_extra, last.z + sp.max_z],
                    //Arriba nuevo izquierda
                    [next_x + min_x, next_y + y_extra, next_z + max_z],
                    //Arriba nuevo derecha
                    [next_x + max_x, next_y + y_extra, next_z + sp.max_z],
                ]);
                norm.extend_from_slice(&[[0., 0., 1.0], [0., 0., 1.0], [0., 0., 1.0], [0., 0., 1.0]]);
                //uvs.extend_from_slice(&[[0., 0.], [1.0, 0.], [1.0, 1.0], [0., 1.0]]);

                indices.extend_from_slice(&[idx + 18, idx + 16, idx + 17, idx + 17, idx + 19, idx + 18, ]); //DOWN
                vert.extend_from_slice(&[
                    //Abajo viejo derecha
                    [last.x + max_x, last.y, last.z + min_z],
                    //Abajo viejo izquierda
                    [last.x + min_x, last.y, last.z + sp.min_z],
                    //Abajo nuevo derecha
                    [next_x + max_x, next_y, next_z + min_z],
                    //Abajo nuevo izquierda
                    [next_x + min_x, next_y, next_z + sp.min_z],
                ]);
                norm.extend_from_slice(&[[0., 0., 1.0], [0., 0., 1.0], [0., 0., 1.0], [0., 0., 1.0]]);

                //norm.extend_from_slice(&[ [0., 0., -1.0], [0., 0., -1.0], [0., 0., -1.0],  [0., 0., -1.0] ]);
                //uvs.extend_from_slice(&[[1., 0.], [0.0, 0.0], [0.0, 1.0], [1., 1.0]]);


                last = *distances;
            }
        }

        let indices = Indices::U16(indices.iter().map(|a| *a as u16).collect());

        let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0]; vert.len()];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vert);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, norm);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}
