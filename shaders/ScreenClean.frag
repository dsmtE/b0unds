#version 410

layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 out_Color;

void main() {
    out_Color = vec4(uv, 0., 1.);
}