use std::{convert::identity, usize};

use cgmath::{Matrix, Quaternion, SquareMatrix, Transform};

use super::{joint::Joint, joint_transform::JointTransform, keyframe::Keyframe};

pub struct Skeleton {
    joints: Vec<Joint>,
}

impl Skeleton {
    pub fn new(collada_skeleton: &collada::Skeleton) -> Self {
        let nr_joints: usize = collada_skeleton.joints.len();
        let mut joints = Vec::new();

        for i in 0..nr_joints {
            let collada_joint = &collada_skeleton.joints[i];
            let collada_bind_pose = collada_skeleton.bind_poses[i];

            let id = collada_joint.id.clone();
            let name = collada_joint.name.clone();
            let parent_index = collada_joint.parent_index;
            let inverse_bind_pose = collada_joint.inverse_bind_pose;

            let joint = Joint::new(
                id,
                name,
                i,
                parent_index as usize,
                collada_bind_pose.into(),
                inverse_bind_pose.into(),
            );

            joints.push(joint);
        }

        Self { joints }
    }

    fn find_joint(&self, id: &str) -> Option<usize> {
        for (i, joint) in self.joints.iter().enumerate() {
            if joint.id == id {
                return Some(i);
            }
        }

        None
    }

    pub fn create_key_frame(&self, name: &str, sample_pose: cgmath::Matrix4<f32>) -> Keyframe {
        let size = self.joints.len();
        let mut joint_transforms: Vec<JointTransform> = vec![JointTransform::zero(); size];

        let transform = JointTransform::from_mat4(sample_pose);

        let joint = self.find_joint(name);
        match joint {
            Some(index) => {
                joint_transforms[index] = transform;
            }
            None => todo!(),
        }

        Keyframe::new(joint_transforms)
    }
}
