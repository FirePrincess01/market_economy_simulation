//! Calculates the mouse direction on the screen

pub struct MouseSelector {}

impl MouseSelector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_mouse_direction(
        &self,
        surface_width: u32,
        surface_height: u32,
        fovy_half_tan: f32,
        mouse_position: &cgmath::Vector2<u32>,
        camera_yaw: cgmath::Rad<f32>,
        camera_pitch: cgmath::Rad<f32>,
    ) -> cgmath::Vector3<f32> {
        // Invert y
        let mouse_pos_x = mouse_position.x as i32;
        let mouse_pos_y = surface_height as i32 - mouse_position.y as i32;

        // Centering
        let mouse_pos_x = mouse_pos_x - surface_width as i32 / 2;
        let mouse_pos_y = mouse_pos_y - surface_height as i32 / 2;

        // Norm amd switch coordinates to global coordinate system
        let mouse_pos_z = mouse_pos_x as f32 * fovy_half_tan * 2.0 / surface_height as f32;
        let mouse_pos_x = 1.0;
        let mouse_pos_y = mouse_pos_y as f32 * fovy_half_tan * 2.0 / surface_height as f32;

        // Transform vector
        let cam_trans = cgmath::Matrix3::from_angle_y(-camera_yaw)
            * cgmath::Matrix3::from_angle_z(camera_pitch);

        cam_trans * cgmath::Vector3::new(mouse_pos_x, mouse_pos_y, mouse_pos_z)
    }
}
