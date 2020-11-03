pub struct Instance {
    pub position: glam::Vec3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceRaw {
    pub position: glam::Vec4,
}

unsafe impl bytemuck::Pod for InstanceRaw {}
unsafe impl bytemuck::Zeroable for InstanceRaw {}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            position: self.position.extend(1.0),
        }
    }
}
