

pub struct Keyframe {
    // pub joint_transforms: Vec<JointTransform>,
    pub joint_transforms: Vec<cgmath::Matrix4<f32>>,
}

impl Keyframe {
    // pub fn new(joint_transforms: Vec<JointTransform>) -> Self {
    //     Self { joint_transforms }
    // }

    pub fn new(joint_transforms: Vec<cgmath::Matrix4<f32>>) -> Self {
        Self { joint_transforms }
    }
}