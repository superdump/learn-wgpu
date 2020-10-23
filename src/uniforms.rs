#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone)] // This is so we can store this in a buffer
pub struct Uniforms {
    view_proj: glam::Mat4,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &crate::camera::Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}
