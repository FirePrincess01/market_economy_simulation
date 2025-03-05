
#[derive(Clone)]
pub struct AnimationTranslation {
    pub key_times: Vec<f32>,
    pub joint_translations: Vec<cgmath::Vector3<f32>>,
}

impl AnimationTranslation {
    pub fn get_translation(&self, key_time: f32) -> &cgmath::Vector3<f32> {
        for (i, time) in self.key_times.iter().enumerate()
        {
            if key_time <= *time {
                let res = self.joint_translations.get(i);
                match res {
                    Some(res) => return res,
                    None => return self.joint_translations.last().unwrap(),
                }
            }
        }
        
        self.joint_translations.last().unwrap()
    }
}


#[derive(Clone)]
pub struct AnimationRotation {
    pub key_times: Vec<f32>,
    pub joint_rotations: Vec<cgmath::Quaternion<f32>>,
}

impl AnimationRotation {
    pub fn get_rotation(&self, key_time: f32) -> &cgmath::Quaternion<f32> {
        for (i, time) in self.key_times.iter().enumerate()
        {
            if key_time <= *time {
                let res = self.joint_rotations.get(i);
                match res {
                    Some(res) => return res,
                    None => return self.joint_rotations.last().unwrap(),
                }
            }
        }
        
        self.joint_rotations.last().unwrap()
    }
}

#[derive(Clone)]
pub struct AnimationData {
    pub name: String,
    pub joint_translations: Vec<AnimationTranslation>,
    pub joint_rotations: Vec<AnimationRotation>,
}

pub struct MeshData {
    // vertex data
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub joints: Vec<[u8; 4]>,
    pub weights: Vec<[f32; 4]>,

    // indices
    pub indices: Vec<u16>,
}

pub struct SkeletonData {
    pub joint_names: Vec<String>,
    pub joint_children: Vec<Vec<String>>,
    pub joint_translations: Vec<cgmath::Vector3<f32>>,
    pub joint_rotations: Vec<cgmath::Quaternion<f32>>,
    pub inverse_bind_transforms: Vec<cgmath::Matrix4<f32>>,
}

pub struct AnimatedObjectData {
    pub mesh: MeshData,
    pub skeleton: SkeletonData,
    pub animations: Vec<AnimationData>,   
}

impl AnimatedObjectData {
    pub fn joint_children_indices(&self, index:usize) -> Vec<usize> {
        let mut res = Vec::new();
        
        let children = &self.skeleton.joint_children[index];
        for child in children {
            let joint_names = &self.skeleton.joint_names;
            for (i, name) in joint_names.iter().enumerate() {
                if child == name {
                    res.push(i);
                }
            }
        }

        res
    }
}
