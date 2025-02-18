use cgmath::SquareMatrix;

pub struct Joint {
    pub id: String,
    pub name: String,
    pub index: usize,  // ID of the joint (exported value from blender)
    pub parent: usize, // ID of the parent joint

    pub bind_transform: cgmath::Matrix4<f32>, // Parent-relative transforms for the joint (at time of vertex binding)
    pub inverse_bind_transform: cgmath::Matrix4<f32>, // Matrix transforming vertex coordinates from model-space to joint-space

    // pub joint_transform: cgmath::Matrix4<f32>, // Joint transform used to upload to the GPU
}

impl Joint {
    pub fn new(
        id: String,
        name: String,
        index: usize,
        parent: usize,
        bind_transform: cgmath::Matrix4<f32>,
        inverse_bind_transform: cgmath::Matrix4<f32>,
    ) -> Self {

        // let joint_transform = cgmath::Matrix4::identity();

        Self {
            id,
            name,
            index,
            parent,
            bind_transform,
            inverse_bind_transform,
            // joint_transform,
        }
    }
}
