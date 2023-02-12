use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

fn to_3d(pos: usize, width: usize, height: usize) -> (usize, usize, usize) {
    (pos % width, (pos / width) % height, pos / (width * height))
}

fn to_1d(x: usize, y: usize, z: usize, width: usize, height: usize) -> usize {
    (z * width * height) + (y * width) + x
}

pub enum QuadSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

pub struct Quad {
    pub pos: Vec3,
    pub quad_side: QuadSide,
}

impl Quad {
    fn new(pos: Vec3, quad_side: QuadSide) -> Self {
        Quad { pos, quad_side }
    }

    fn into_data(quads: &Vec<Quad>) -> Vec<([f32; 3], [f32; 3], [f32; 2])> {
        let mut formatted_quads: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
        for quad in quads {
            formatted_quads.append(
                &mut quad
                    .quad_side
                    .get_side_data(quad.pos.x, quad.pos.y, quad.pos.z)
                    .to_vec(),
            );
        }
        formatted_quads
    }
}

impl QuadSide {
    fn get_side_data(
        &self,
        min_x: f32,
        min_y: f32,
        min_z: f32,
    ) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let max_x = min_x + 1.;
        let max_y = min_y + 1.;
        let max_z = min_z + 1.;

        match *self {
            QuadSide::Front => [
                ([min_x, min_y, max_z], [0., 0., 1.0], [0., 0.]),
                ([max_x, min_y, max_z], [0., 0., 1.0], [1.0, 0.]),
                ([max_x, max_y, max_z], [0., 0., 1.0], [1.0, 1.0]),
                ([min_x, max_y, max_z], [0., 0., 1.0], [0., 1.0]),
            ],
            QuadSide::Back => [
                ([min_x, max_y, min_z], [0., 0., -1.0], [1.0, 0.]),
                ([max_x, max_y, min_z], [0., 0., -1.0], [0., 0.]),
                ([max_x, min_y, min_z], [0., 0., -1.0], [0., 1.0]),
                ([min_x, min_y, min_z], [0., 0., -1.0], [1.0, 1.0]),
            ],
            QuadSide::Right => [
                ([max_x, min_y, min_z], [1.0, 0., 0.], [0., 0.]),
                ([max_x, max_y, min_z], [1.0, 0., 0.], [1.0, 0.]),
                ([max_x, max_y, max_z], [1.0, 0., 0.], [1.0, 1.0]),
                ([max_x, min_y, max_z], [1.0, 0., 0.], [0., 1.0]),
            ],
            QuadSide::Left => [
                ([min_x, min_y, max_z], [-1.0, 0., 0.], [1.0, 0.]),
                ([min_x, max_y, max_z], [-1.0, 0., 0.], [0., 0.]),
                ([min_x, max_y, min_z], [-1.0, 0., 0.], [0., 1.0]),
                ([min_x, min_y, min_z], [-1.0, 0., 0.], [1.0, 1.0]),
            ],
            QuadSide::Top => [
                ([max_x, max_y, min_z], [0., 1.0, 0.], [1.0, 0.]),
                ([min_x, max_y, min_z], [0., 1.0, 0.], [0., 0.]),
                ([min_x, max_y, max_z], [0., 1.0, 0.], [0., 1.0]),
                ([max_x, max_y, max_z], [0., 1.0, 0.], [1.0, 1.0]),
            ],
            QuadSide::Bottom => [
                ([max_x, min_y, max_z], [0., -1.0, 0.], [0., 0.]),
                ([min_x, min_y, max_z], [0., -1.0, 0.], [1.0, 0.]),
                ([min_x, min_y, min_z], [0., -1.0, 0.], [1.0, 1.0]),
                ([max_x, min_y, min_z], [0., -1.0, 0.], [0., 1.0]),
            ],
        }
    }
}

pub struct VoxelBox {
    pub voxel_box_size: usize,
    pub voxels: Vec<usize>,
    pub quads: Vec<Quad>,
}

impl VoxelBox {
    pub fn new(voxel_box_size: usize) -> Self {
        let mut quads: Vec<Quad> = vec![];
        let voxels = vec![1; voxel_box_size * voxel_box_size * voxel_box_size];

        for (pos, voxel) in voxels.iter().enumerate() {
            if *voxel == 1 {
                let (x, y, z) = to_3d(pos, voxel_box_size, voxel_box_size);

                quads.append(&mut Self::create_voxel(voxel_box_size, &voxels, x, y, z))
            }
        }

        VoxelBox {
            voxel_box_size,
            voxels,
            quads,
        }
    }

    fn create_voxel(
        voxel_box_size: usize,
        voxels: &Vec<usize>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<Quad> {
        let mut quads: Vec<Quad> = vec![];
        
        if !(x + 1 < voxel_box_size && voxels.get(to_1d(x + 1, y, z, voxel_box_size, voxel_box_size)) == Some(&1)) {
            quads.push(Quad::new(Vec3::new(x as f32, y as f32, z as f32), QuadSide::Right));
        }
    
        if !(x > 0 && voxels.get(to_1d(x - 1, y, z, voxel_box_size, voxel_box_size)) == Some(&1)) {
            quads.push(Quad::new(Vec3::new(x as f32, y as f32, z as f32), QuadSide::Left));
        }
    
        if !(y + 1 < voxel_box_size && voxels.get(to_1d(x, y + 1, z, voxel_box_size, voxel_box_size)) == Some(&1)) {
            quads.push(Quad::new(Vec3::new(x as f32, y as f32, z as f32), QuadSide::Top));
        }
    
        if !(y > 0 && voxels.get(to_1d(x, y - 1, z, voxel_box_size, voxel_box_size)) == Some(&1)) {
            quads.push(Quad::new(Vec3::new(x as f32, y as f32, z as f32), QuadSide::Bottom));
        }
    
        if !(z + 1 < voxel_box_size && voxels.get(to_1d(x, y, z + 1, voxel_box_size, voxel_box_size)) == Some(&1)) {
            quads.push(Quad::new(Vec3::new(x as f32, y as f32, z as f32), QuadSide::Front));
        }

        if !(z > 0 && voxels.get(to_1d(x, y, z - 1, voxel_box_size, voxel_box_size)) == Some(&1)) {
            quads.push(Quad::new(Vec3::new(x as f32, y as f32, z as f32), QuadSide::Back));
        }
       
        quads
    }
}

impl From<VoxelBox> for Mesh {
    fn from(voxel_box: VoxelBox) -> Self {
        let quads = Quad::into_data(&voxel_box.quads);

        let positions: Vec<_> = quads.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = quads.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = quads.iter().map(|(_, _, uv)| *uv).collect();

        let mut indices: Vec<u32> = vec![];
        for quad in 0..(voxel_box.quads.len()) {
            indices.append(&mut vec![
                4 * quad as u32,
                1 + 4 * quad as u32,
                2 + 4 * quad as u32,
                2 + 4 * quad as u32,
                3 + 4 * quad as u32,
                4 * quad as u32,
            ]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.to_vec());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
