#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone)] // This is so we can store this in a buffer
pub struct Uniforms {
    world_to_screen: glam::Mat4,
    camera_position: glam::Vec4, // vec4
    center_to_edge: glam::Vec4,  // vec4
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            world_to_screen: glam::Mat4::identity(),
            camera_position: glam::Vec4::zero(),
            center_to_edge: glam::Vec4::new(0.5, 0.5, 0.5, 0.5),
        }
    }

    pub fn update(&mut self, camera: &crate::camera::Camera) {
        self.world_to_screen = camera.build_view_projection_matrix();
        self.camera_position = camera.eye.extend(1.0);
    }
}
