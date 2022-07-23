#version 410

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) out vec2 uv;

vec2 positions[3] = vec2[](
    vec2(-1., -1.),
    vec2(-1., 3.),
    vec2(3., -1.)
);

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    uv = (positions[gl_VertexIndex]+vec2(1.0))/vec2(2.0);
}