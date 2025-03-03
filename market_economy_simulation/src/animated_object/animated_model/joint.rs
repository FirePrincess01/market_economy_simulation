use cgmath::Matrix;




pub struct Joint {
    name: String,
    child_names: Vec<String>,
    children: Vec<usize>,

    translation: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    inverse_bind_transform: cgmath::Matrix4<f32>, // Matrix transforming vertex coordinates from model-space to joint-space
}

impl Joint {
    pub fn new(
        name: String,
        child_names: Vec<String>,
        children: Vec<usize>,
        translation: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
        inverse_bind_transform: cgmath::Matrix4<f32>,
    ) -> Self {
        Self {
            name,
            child_names,
            children,
            translation,
            rotation,
            inverse_bind_transform,
        }
    }

    pub fn get_transform(&self) -> cgmath::Matrix4<f32> {
        let transfrom_decomposed: cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>> = cgmath::Decomposed {
            scale: 1.0,
            rot: self.rotation,
            disp: self.translation,
        };
        let transfrom = cgmath::Matrix4::from(transfrom_decomposed);

        transfrom
    }

    pub fn get_inverse_bind_transform(&self) -> cgmath::Matrix4<f32> {
        self.inverse_bind_transform.transpose()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_children_indices(&self) -> &[usize] {
        &self.children
    }
}
