use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};

pub struct Poly {
    pub min_x: f32,
    pub max_x: f32,

    pub y: Vec<(f32, f32, f32)>,
    pub origen: Vec3,
    pub min_z: f32,
    pub max_z: f32,
}

impl Poly {
    pub fn new(
        x_length: f32,
        z_length: f32,
        y_lengths: Vec<(f32, f32, f32)>,
        origen: Vec3,
    ) -> Poly {
        Poly {
            max_x: x_length / 2.0,
            min_x: -x_length / 2.0,
            max_z: z_length / 2.0,
            min_z: -z_length / 2.0,
            y: y_lengths,
            origen: origen,
        }
    }
}

impl Default for Poly {
    fn default() -> Self {
        Poly::new(2.0, 2.0, vec![], Vec3::default())
    }
}

impl From<Poly> for Mesh {
    fn from(sp: Poly) -> Self {
        let mut vert: Vec<[f32; 3]> = vec![];
        let mut norm: Vec<[f32; 3]> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];


        let mut last = sp.origen;

        let y =
            sp.y.iter()
                .map(|a| Vec3::new(a.0, a.1, a.2))
                .collect::<Vec<_>>();

        let mut indices = vec![];

        //Back
        indices.extend_from_slice(&[0, 1, 2, 2, 3, 0]);
        
        if last.y != y[0].y {

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

        norm.extend_from_slice(&[[0., 1.0, 0.], [0., 1.0, 0.], [0., 1.0, 0.], [0., 1.0, 0.]]);


        } else {

            vert.extend_from_slice(&[
                //Abajo derecha
                [last.x + sp.min_z, last.y, last.z + sp.min_z],
                //Abajo izquierda
                [last.x + sp.min_x, last.y, last.z + sp.max_z],
                //Arriba izquierda
                [last.x + sp.min_x, last.y, last.z + sp.min_z],
                //Arriba derecha
                [last.x + sp.min_z, last.y, last.z + sp.max_z],
            ]);
            norm.extend_from_slice(&[[0., -1.0, 0.], [0., -1.0, 0.], [0., -1.0, 0.], [0., -1.0, 0.]]);

        }
      
        uvs.extend_from_slice(&[[0., 0.], [1.0, 0.], [1.0, 1.0], [0., 1.0]]);


        for distances in y.iter() {


            let Poly {    
                mut min_x,
                mut max_x,
                mut min_z,
                mut max_z, 
                ..} = sp;

            let idx = vert.len();

            //println!("{:?}", distances);

            let (mut next_x, mut next_y, mut next_z) = (distances.x, distances.y, distances.z);

            let mut y_extra = 0.0;
            if next_y == last.y {
               max_x = min_x;
               max_z = min_z; 
               min_z = sp.max_z;
               y_extra= sp.max_x*2.0;
                /* 
                //Abajo derecha
                println!("Abajo derecha {:?}",[next_x + max_x, next_y, next_z + sp.min_z]);
                //Abajo izquierda
                println!("Abajo izquierda {:?}",[next_x + min_x, next_y, next_z + min_z]);
                //Arriba izquierda
                println!("Arriba izquierda {:?}",[next_x + min_x, next_y  + y_extra, next_z + max_z]);
                //Arriba derecha
                println!("Arriba derecha {:?}",[next_x + max_x , next_y+ y_extra, next_z + sp.max_z]);*/
            }
            //Front.
            indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 2, idx + 3, idx]);
            //indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 2, idx + 3, idx+4]);
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
            norm.extend_from_slice(&[[0., 1.0, 0.], [0., 1.0, 0.], [0., 1.0, 0.], [0., 1.0, 0.]]);
            uvs.extend_from_slice(&[[1.0, 0.], [0., 0.], [0.0, 1.0], [1.0, 1.0]]);

                 
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
            norm.extend_from_slice(&[
                [-1.0, 0., 0.],
                [-1.0, 0., 0.],
                [-1.0, 0., 0.],
                [-1.0, 0., 0.],
            ]);
            uvs.extend_from_slice(&[[1.0, 0.], [0.0, 0.], [0.0, 1.0], [1.0, 1.0]]);
            
            
            indices.extend_from_slice(&[idx + 9, idx + 8, idx + 11, idx + 11, idx + 10, idx + 9]); //RIGHT
            //indices.extend_from_slice(&[idx + 8, idx + 9, idx + 10, idx + 10, idx + 11, idx + 8]); //RIGHT

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
            norm.extend_from_slice(&[
                [-1.0, 0., 0.],
                [-1.0, 0., 0.],
                [-1.0, 0., 0.],
                [-1.0, 0., 0.],
            ]);
            uvs.extend_from_slice(&[[0., 0.], [1.0, 0.], [1.0, 1.0], [0., 1.0]]);

            
            indices.extend_from_slice(&[
                idx + 14,
                idx + 12,
                idx + 13,
                idx + 13,
                idx + 15,
                idx + 14,
            ]); //TOP
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

            uvs.extend_from_slice(&[[0., 0.], [1.0, 0.], [1.0, 1.0], [0., 1.0]]);

            indices.extend_from_slice(&[
                idx + 18,
                idx + 16,
                idx + 17,
                idx + 17,
                idx + 19,
                idx + 18,
            ]); //DOWN
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
            norm.extend_from_slice(&[
                [0., 0., -1.0],
                [0., 0., -1.0],
                [0., 0., -1.0],
                [0., 0., -1.0],
            ]);

            uvs.extend_from_slice(&[[1., 0.], [0.0, 0.0], [0.0, 1.0], [1., 1.0]]);
           /* 
            */

            last = *distances;
        }

    
    
        /*
        let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; vert.len()];
        
        for chunk in indices.chunks(3) {
            let (idx, idx2, idx3) = (chunk[0], chunk[1], chunk[2]);
            //println!("{} {} {}", idx, idx2, idx3);
            let vec1 = Vec3::from_slice_unaligned(&vert[idx2])
                - Vec3::from_slice_unaligned(&vert[idx]);
            let vec2 = Vec3::from_slice_unaligned(&vert[idx3])
                - Vec3::from_slice_unaligned(&vert[idx]);

            let cross = vec1.cross(vec2).normalize();
            //if vec1 == Vec3::default() || vec2 == Vec3::default() { println!("wodafa");}
            //if cross.x.is_nan() { println!("Es nan"); }
            if !cross.x.is_nan(){
                normals[idx] += cross;
                normals[idx2] += cross;
                normals[idx3] += cross;
            } else {

                println!("idx : {} {} {} | v0: {} , v1: {}, v2: {} ", idx, idx2, idx3, Vec3::from_slice_unaligned(&vert[idx]), Vec3::from_slice_unaligned(&vert[idx2]), Vec3::from_slice_unaligned(&vert[idx3]));

            }
        }

        let normals: Vec<[f32; 3]> = normals
            .iter()
            .map(|x| {
                let x = x.normalize();
                println!("{:?}", x);
                [x.x, x.y, x.z]
            })
            .collect::<Vec<[f32; 3]>>();*/

        let indices = Indices::U16(indices.iter().map(|a| *a as u16).collect());

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vert);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, norm);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}
