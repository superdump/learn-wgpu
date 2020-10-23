pub struct Instance {
    pub position: glam::Vec3,
    pub rotation: glam::Quat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceRaw {
    pub model: glam::Mat4,
}

unsafe impl bytemuck::Pod for InstanceRaw {}
unsafe impl bytemuck::Zeroable for InstanceRaw {}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: glam::Mat4::from_rotation_translation(self.rotation, self.position),
        }
    }
}
