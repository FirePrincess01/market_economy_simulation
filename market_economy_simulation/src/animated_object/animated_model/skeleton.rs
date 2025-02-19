use std::{convert::identity, usize};

use cgmath::{Matrix, Quaternion, SquareMatrix, Transform, Zero};

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

    fn get_root_joint(&self) -> Option<usize> {
        for (i, joint) in self.joints.iter().enumerate() {
            if joint.parent == 255 {
                return Some(i);
            }
        }

        None
    }

    fn get_children(&self, joint_index: usize) -> Vec<usize> {
        let mut res = Vec::new();

        for (i, joint) in self.joints.iter().enumerate() {
            if joint.parent == joint_index {
                res.push(i);
            }
        }

        res
    }

    fn calculate_joint_transforms(
        &self,
        local_transforms: &[cgmath::Matrix4<f32>],
        joint_transforms: &mut [cgmath::Matrix4<f32>],
        parent_transform: &cgmath::Matrix4<f32>,
        joint_index: usize,
    ) {
        let joint = &self.joints[joint_index];

        // calculate current transformation
        let current_transform = parent_transform * local_transforms[joint_index];

        // calculate current transformation applicable to a vertex
        let current_joint_transform = current_transform * joint.inverse_bind_transform;
        joint_transforms[joint_index] = current_joint_transform;

        let children = self.get_children(joint_index);
        for child in children {
            self.calculate_joint_transforms(
                local_transforms,
                joint_transforms,
                &current_transform,
                child,
            )
        }
    }

    pub fn create_key_frame(&self, name: &str, sample_pose: cgmath::Matrix4<f32>) -> Keyframe {
        let size = self.joints.len();
        let mut local_transforms: Vec<cgmath::Matrix4<f32>> =
            vec![cgmath::Matrix4::identity(); size];
        let mut joint_transforms: Vec<cgmath::Matrix4<f32>> =
            vec![cgmath::Matrix4::identity(); size];

        // set local transforms
        for i in 0..size {
            let local_transform = &mut local_transforms[i];
            let joint = &self.joints[i];

            *local_transform = joint.bind_transform;
        }

        // apply animation poses
        let joint_index = self.find_joint(name).expect("Joint not found");
        local_transforms[joint_index] = sample_pose;

        // test
        local_transforms[1] = cgmath::Matrix4::from_angle_y(cgmath::Deg(90.0));


        // calculate joint transforms
        let root_joint_index = self.get_root_joint().expect("Root Joint not found");
        let parent_transform = cgmath::Matrix4::identity();
        self.calculate_joint_transforms(
            &local_transforms,
            &mut joint_transforms,
            &parent_transform,
            root_joint_index,
        );

        Keyframe::new(joint_transforms)
    }
}
