#version 450

layout (set = 0, binding = 0) uniform UBO {
    mat4 world_to_screen;
    vec4 camera_position;
    vec4 center_to_edge;
} ubo;

struct InstanceData {
    vec4 position;
};

layout(set = 0, binding = 1) buffer Instances {
    InstanceData instances[];
};

layout (location = 0) out vec3 uvw;
layout (location = 1) out vec3 local_camera_pos;
layout (location = 2) out vec3 local_pos;

void main() {
    uint vx = gl_VertexIndex;
    // 8 vertices per cube so >> 3 to get instance index
    uint instance = vx >> 3;

    vec3 instance_pos = instances[instance].position.xyz;
    local_camera_pos = ubo.camera_position.xyz - instance_pos;

    uvec3 xyz = uvec3(vx & 0x1, (vx & 0x4) >> 2, (vx & 0x2) >> 1);

    if (local_camera_pos.x > 0) xyz.x = 1 - xyz.x;
    if (local_camera_pos.y > 0) xyz.y = 1 - xyz.y;
    if (local_camera_pos.z > 0) xyz.z = 1 - xyz.z;

    uvw = vec3(xyz);
    vec3 pos = uvw * 2.0 - 1.0;

    local_pos = pos.xyz * ubo.center_to_edge.xyz;

    gl_Position = ubo.world_to_screen * vec4(instance_pos + local_pos, 1.0);
}
