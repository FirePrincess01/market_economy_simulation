use crate::{animated_object::animated_object_data::AnimationData, deferred_animation_shader};

use super::{keyframe::Keyframe, skeleton::Skeleton};

pub struct Animation {
    // _name: String,

    // key_times: Vec<f32>,
    // key_frames: Vec<Keyframe>,

    animation_data: AnimationData,
    max_key_frame_time: f32,

    current_key_frame_time: instant::Duration,

}

impl Animation {
    // pub fn new_old(
    //     skeleton: &Skeleton,
    //     name: &str,
    //     sample_times: &[f32],
    //     animation_channels: &Vec<collada::AnimationChannel>,
    // ) -> Self {
    //     let mut key_frames = Vec::new();

    //     let mut joint_names: Vec<String> = Vec::new();
    //     for animation_channel in animation_channels {
    //         let mut name_split = animation_channel.target.split('/');

    //         let joint_name: &str = name_split.next().unwrap_or("");

    //         joint_names.push(joint_name.to_string());
    //     }

    //     for i in 0..sample_times.len() {
    //         let mut poses: Vec<cgmath::Matrix4<f32>> = Vec::new();
    //         for animation_channel in animation_channels {
    //             poses.push(animation_channel.sample_poses[i].into());
    //         }

    //         let keyframe = skeleton.create_key_frame(&joint_names, &poses);
    //         key_frames.push(keyframe);
    //     }

    //     Self {
    //         _name: name.to_string(),
    //         key_times: sample_times.to_owned(),
    //         key_frames,
    //         // last_index: 0,
    //         current_key_frame_time: instant::Duration::ZERO,
    //     }
    // }

    pub fn new(animation_data: crate::animated_object::animated_object_data::AnimationData) -> Self
    {
        let mut max_key_frame_time: f32 = 0.0;
        for elem in &animation_data.joint_rotations {
            let max_time = *elem.key_times.last().unwrap();
            max_key_frame_time = max_key_frame_time.max(max_time);

        }

        for elem in &animation_data.joint_translations {
            let max_time = *elem.key_times.last().unwrap();
            max_key_frame_time = max_key_frame_time.max(max_time);
        }

        Self {
            animation_data,
            max_key_frame_time,

            current_key_frame_time: instant::Duration::ZERO,
        }
    }

    pub fn increment_time(&mut self, dt: &instant::Duration) {

        self.current_key_frame_time += *dt;

        if self.current_key_frame_time.as_secs_f32() > self.max_key_frame_time {
            self.current_key_frame_time = instant::Duration::ZERO;
        }
    }

    fn find_key_frame(key_times: &[f32], time: instant::Duration) -> usize {
        for (i, key_time) in key_times.iter().enumerate() {
            if time.as_secs_f32() <= *key_time {
                return i;
            }
        }

        0
    }

    fn get_sample_poses(&self) -> Vec<cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>> {
        let current_time = self.current_key_frame_time.as_secs_f32();
        let joint_translations = &self.animation_data.joint_translations;
        let joint_rotations = &self.animation_data.joint_rotations;
        let len = joint_translations.len();
        assert_eq!(joint_rotations.len(), len);

        let mut res: Vec<cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>> = Vec::new();

        for i in 0..len {
            let translation = joint_translations[i].get_translation(current_time); 
            let rotation = joint_rotations[i].get_rotation(current_time); 

            let decomposed = cgmath::Decomposed {
                scale: 1.0,
                rot: *rotation,
                disp: *translation,
            };

            res.push(decomposed);
        }

        res
    }

    pub fn update_animation_uniform(
        &self,
        skeleton: &Skeleton,
        animation_uniform: &mut deferred_animation_shader::AnimationUniform,
    ) {
        let sample_poses: Vec<cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>> = self.get_sample_poses();
        let joint_transforms = skeleton.create_key_frame(&sample_poses);

        assert!(animation_uniform.joint_transform.len() >= joint_transforms.len());

        let len = joint_transforms.len();
        for i in 0..len {
            animation_uniform.joint_transform[i] = joint_transforms[i].into();
        }
    }
}
