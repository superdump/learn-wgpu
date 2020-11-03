#version 450

layout (location = 0) in vec3 uvw;
layout (location = 1) in vec3 local_camera_pos;
layout (location = 2) in vec3 local_pos;

layout (location = 0) out vec4 f_color;

void main() {
    f_color = vec4(uvw.rgb, 1.0);
}
