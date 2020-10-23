#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

// set corresponds to the first parameter of set_bind_group
// binding corresponds to the binding specified when creating the BindGroupLayout and BindGroup
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
}
