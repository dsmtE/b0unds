struct CameraData {
    mat4 ViewProjection;
};

vec3 PerspectiveClipToWorldSpace(CameraData cam, vec3 pos) {
    vec4 v = (inverse(cam.ViewProjection) * vec4(pos, 1.));
    return v.xyz / v.w;
}

vec3 rayOrigin(CameraData cam, vec2 uv) {
    return PerspectiveClipToWorldSpace(cam, vec3(uv, -1.));
}

vec3 rayDirection(CameraData cam, vec2 uv) {
    vec3 begin = vec3(uv, -1.);
    vec3 end   = vec3(uv, 0.);
    return normalize(PerspectiveClipToWorldSpace(cam, end) - PerspectiveClipToWorldSpace(cam, begin));
}