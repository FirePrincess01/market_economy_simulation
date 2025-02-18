use super::joint_transform::JointTransform;


pub struct Keyframe {
    pub joint_transforms: Vec<JointTransform>,
}

impl Keyframe {
    pub fn new(joint_transforms: Vec<JointTransform>) -> Self {
        Self { joint_transforms }
    }
}