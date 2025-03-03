use cgmath::{Matrix, SquareMatrix};

use super::{joint::Joint, keyframe::Keyframe};

pub struct Skeleton {
    joints: Vec<Joint>,
}

impl Skeleton {
    // pub fn new_old(collada_skeleton: &collada::Skeleton) -> Self {
    //     let nr_joints: usize = collada_skeleton.joints.len();
    //     let mut joints = Vec::new();

    //     for i in 0..nr_joints {
    //         let collada_joint = &collada_skeleton.joints[i];
    //         let collada_bind_pose = collada_skeleton.bind_poses[i];

    //         let id = collada_joint.id.clone();
    //         let name = collada_joint.name.clone();
    //         let parent_index = collada_joint.parent_index;
    //         let inverse_bind_pose = collada_joint.inverse_bind_pose;

    //         let joint = Joint::new(
    //             id,
    //             name,
    //             i,
    //             parent_index as usize,
    //             collada_bind_pose.into(),
    //             inverse_bind_pose.into(),
    //         );

    //         joints.push(joint);
    //     }

    //     Self { joints }
    // }

    pub(crate) fn new(
        animation_data: &crate::animated_object::animated_object_data::AnimatedObjectData,
    ) -> Self {
        let nr_joints = animation_data.joint_name.len();

        assert_eq!(animation_data.joint_children.len(), nr_joints);
        assert_eq!(animation_data.joint_translation.len(), nr_joints);
        assert_eq!(animation_data.joint_rotation.len(), nr_joints);
        assert_eq!(animation_data.inverse_bind_transform.len(), nr_joints);

        let mut joints = Vec::new();

        for i in 0..nr_joints {
            let name = animation_data.joint_name[i].clone();
            let child_names = &animation_data.joint_children[i];
            let child_indices = animation_data.joint_children_indices(i);
            let translation = animation_data.joint_translation[i];
            let rotation = animation_data.joint_rotation[i];
            let inverse_bind_transform = &animation_data.inverse_bind_transform[i];

            let joint = Joint::new(name, child_names.clone(), child_indices, translation, rotation, inverse_bind_transform.clone());
            joints.push(joint);
        }

        Self { joints }
    }

    // fn find_joint(&self, id: &str) -> Option<usize> {
    //     for (i, joint) in self.joints.iter().enumerate() {
    //         if joint.id == id {
    //             return Some(i);
    //         }
    //     }

    //     None
    // }

    // fn get_root_joint(&self) -> Option<usize> {
    //     for (i, joint) in self.joints.iter().enumerate() {
    //         if joint.parent == 255 {
    //             return Some(i);
    //         }
    //     }

    //     None
    // }

    // fn get_children(&self, joint_index: usize) -> Vec<usize> {
    //     let mut res = Vec::new();

    //     for (i, joint) in self.joints.iter().enumerate() {
    //         if joint.parent == joint_index {
    //             res.push(i);
    //         }
    //     }

    //     res
    // }

    fn calculate_joint_transforms(
        &self,
        local_transforms: &[cgmath::Matrix4<f32>],
        joint_transforms: &mut [cgmath::Matrix4<f32>],
        parent_transform: &cgmath::Matrix4<f32>,
        joint_index: usize,
    ) {
        let joint = &self.joints[joint_index];

        // calculate current transformation
        // let current_transform = parent_transform * local_transforms[joint_index].transpose();
        let current_transform = parent_transform * local_transforms[joint_index];

        // calculate current transformation applicable to a vertex
        // let current_joint_transform = current_transform * joint.get_inverse_bind_transform().transpose();
        let current_joint_transform = current_transform * joint.get_inverse_bind_transform();
        // let current_joint_transform = cgmath::Matrix4::identity();
        joint_transforms[joint_index] = current_joint_transform;

        let children = joint.get_children_indices();
        for child in children {
            self.calculate_joint_transforms(
                local_transforms,
                joint_transforms,
                &current_transform,
                *child,
            )
        }
    }

    pub fn create_key_frame(
        &self,
        sample_poses: &[cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>],
    ) -> Vec<cgmath::Matrix4<f32>> {
        let size = self.joints.len();
        let mut local_transforms: Vec<cgmath::Matrix4<f32>> =
            vec![cgmath::Matrix4::identity(); size];
        let mut joint_transforms: Vec<cgmath::Matrix4<f32>> =
            vec![cgmath::Matrix4::identity(); size];

        // set local transforms
        for i in 0..size {
            local_transforms[i] = self.joints[i].get_transform();
        }

        // apply sample poses
        for i in 0..size {
            local_transforms[i] = cgmath::Matrix4::from(sample_poses[i]);
        }

        // calculate joint transforms
        let root_joint_index = 0;
        let parent_transform = cgmath::Matrix4::identity();
        self.calculate_joint_transforms(
            &local_transforms,
            &mut joint_transforms,
            &parent_transform,
            root_joint_index,
        );

        joint_transforms
    }
}
