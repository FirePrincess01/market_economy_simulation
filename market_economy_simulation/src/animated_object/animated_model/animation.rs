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
        sample_poses: &Vec<[[f32; 4]; 4]>,
    ) -> Self {
        let name_split = name.split('/');

        let mut joint_name: &str = "";
        for elem in name_split {
            joint_name = elem;
            break;
        };

        let key_times = sample_times.clone();

        let mut key_frames = Vec::new();

        for sample_pose in sample_poses {
            let sample_pose: cgmath::Matrix4<f32> = cgmath::Matrix4::from(*sample_pose);

            let keyframe = skeleton.create_key_frame(joint_name, sample_pose);
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

        assert_eq!(animation_uniform.joint_transform.len(), key_frame.joint_transforms.len());

        for (i, elem) in &mut animation_uniform.joint_transform.iter_mut().enumerate() {
            
            let joint_transform = key_frame.joint_transforms[2].to_mat4();
            
            *elem = joint_transform.into();
        }
    }

}
