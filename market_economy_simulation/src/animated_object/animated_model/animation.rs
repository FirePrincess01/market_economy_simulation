use cgmath::Matrix;

use crate::deferred_animation_shader;

use super::{keyframe::Keyframe, skeleton::Skeleton};

pub struct Animation {
    name: String,

    key_times: Vec<f32>,
    key_frames: Vec<Keyframe>,

    // caching for faster iterations
    // last_index: usize,

    current_key_frame_time: instant::Duration,
}

impl Animation {
    pub fn new(
        skeleton: &Skeleton,
        name: &String,
        sample_times: &Vec<f32>,
        animation_channels: &Vec<collada::AnimationChannel>,
    ) -> Self {

        let key_times = sample_times.clone();

        let mut key_frames = Vec::new();

        let mut joint_names: Vec<String> = Vec::new();
        for animation_channel in animation_channels {
            let name_split = animation_channel.target.split('/');

            let mut joint_name: &str = "";
            for elem in name_split {
                joint_name = elem;
                break;
            };

            joint_names.push(joint_name.to_string());
        }

        for i in 0..sample_times.len() {
            let mut poses: Vec<cgmath::Matrix4<f32>> = Vec::new();
            for animation_channel in animation_channels {
                poses.push(animation_channel.sample_poses[i].into());
            }

            let keyframe = skeleton.create_key_frame(&joint_names, &poses);
            key_frames.push(keyframe);
        }

        Self {
            name: name.clone(),
            key_times,
            key_frames,
            // last_index: 0,
            current_key_frame_time: instant::Duration::ZERO,
        }
    }

    pub fn increment_time(&mut self, dt: &instant::Duration) {
        let max_time = self.key_times.last().unwrap_or(&0.0);

        self.current_key_frame_time += *dt;

        let time_passed = self.current_key_frame_time.as_secs_f32();

        if self.current_key_frame_time.as_secs_f32() > *max_time {
            self.current_key_frame_time = instant::Duration::ZERO;
        }
    }

    fn find_key_frame(key_times: &Vec<f32>, time: instant::Duration) -> usize {
        for (i, key_time) in key_times.iter().enumerate() {
            if time.as_secs_f32() <= *key_time {
                return i;
            }
        }

        return 0;
    }

    pub fn update_animation_uniform(&self, animation_uniform: &mut deferred_animation_shader::AnimationUniform) {
        
        let index = Self::find_key_frame(&self.key_times, self.current_key_frame_time);
        let key_frame = &self.key_frames[index];

        assert!(animation_uniform.joint_transform.len() >= key_frame.joint_transforms.len());

        let len = key_frame.joint_transforms.len();
        for i in 0..len {
            let joint_transform =  key_frame.joint_transforms[i];
            let uniform_element = &mut animation_uniform.joint_transform[i];

            *uniform_element = joint_transform.into();
        }
    }

}
