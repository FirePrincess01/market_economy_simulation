use cgmath::{Matrix, SquareMatrix};

use super::{joint::Joint, keyframe::Keyframe};

pub struct Skeleton {
    joints: Vec<Joint>,
}

impl Skeleton {

    pub(crate) fn new(
        animation_data: &crate::animated_object::animated_object_data::AnimatedObjectData,
    ) -> Self {
        let joint_names = &animation_data.skeleton.joint_names;
        let joint_children = &animation_data.skeleton.joint_children;
        let joint_translations = &animation_data.skeleton.joint_translations;
        let joint_rotations = &animation_data.skeleton.joint_rotations;
        let inverse_bind_transforms = &animation_data.skeleton.inverse_bind_transforms;
        
        let nr_joints = joint_names.len();
        assert_eq!(joint_children.len(), nr_joints);
        assert_eq!(joint_translations.len(), nr_joints);
        assert_eq!(joint_rotations.len(), nr_joints);
        assert_eq!(inverse_bind_transforms.len(), nr_joints);

        let mut joints = Vec::new();

        for i in 0..nr_joints {
            let name = joint_names[i].clone();
            let child_names = &joint_children[i];
            let child_indices = animation_data.joint_children_indices(i);
            let translation = joint_translations[i];
            let rotation = joint_rotations[i];
            let inverse_bind_transform = &inverse_bind_transforms[i];

            let joint = Joint::new(name, child_names.clone(), child_indices, translation, rotation, inverse_bind_transform.clone());
            joints.push(joint);
        }

        Self { joints }
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
        for i in 0..sample_poses.len() {
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
