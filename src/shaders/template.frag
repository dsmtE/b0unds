#version 420

#include "camera.glsl"

layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 out_Color;

layout(set = 0, binding = 0) uniform CameraUniforms
{
    CameraData Camera;
};

#include "rayMarchUtils.glsl"

[SDF_FUNCTION]

#include "rayMarchCore.glsl"

void main() {
    // Centered coordinates (from -1.0 to 1.0) 
    vec2 centeredUv = uv*2.0-1.0;

    vec3 origin = rayOrigin(Camera, centeredUv);
    vec3 dir = rayDirection(Camera, centeredUv);

    out_Color = vec4(render(origin, dir), 1.);
}
