#version 460

layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 out_Color;

struct CameraData {
    mat4 ViewProjection;
    vec3 Position;
    vec3 Direction;
};

layout(set = 0, binding = 0)  uniform CameraData camera_uniforms;

#define MAX_STEPS 100
#define MAX_DIST  100.
#define SURF_DIST .001

float sphereSDF(vec3 p, float radius) { return length(p) - radius; }

vec3 arrayOp(vec3 p, vec3 offset) {
    return mod(p + 0.5 * offset, offset) - 0.5 * offset;
}

float sdf(vec3 p) {
    return sphereSDF(arrayOp(p, vec3(10.)), 1.0);
}

float rayMarch(vec3 rO, vec3 rDir) {
    float dist = 0.;
    for(int i = 0; i < MAX_STEPS; i++) {
        vec3 p = rO + rDir * dist;
        float distSDF = sdf(p);
        dist += distSDF;
        if( dist > MAX_DIST || distSDF < SURF_DIST) break;
    }
    return dist;
}

vec3 normal(vec3 p) {
    const vec2 e = vec2(.001, 0);
    return normalize(sdf(p) - vec3(sdf(p - e.xyy), sdf(p - e.yxy), sdf(p - e.yyx)));
}

vec3 computeCamDir(vec2 uv, vec3 camPos, vec3 camUp, vec3 lookAtPos) {
	vec3 camVec = normalize(lookAtPos - camPos);
	vec3 sideNorm = normalize(cross(camUp, camVec));
	vec3 upNorm = cross(camVec, sideNorm);
	vec3 worldFacing = (camPos + camVec);
	vec3 worldPix = worldFacing + uv.x * sideNorm + uv.y * upNorm;
	return normalize(worldPix - camPos);
}

vec3 render(vec3 ro, vec3 rd) {
    float d   = rayMarch(ro, rd);
    vec3  col = vec3(0.);

    if (d < MAX_DIST) {
        vec3 p = ro + rd * d;
        col = normal(p) * 0.5 + 0.5;
    }

    col = pow(col, vec3(.4545)); // gamma correction
    return col;
}

void main() {

    // Centered coordinates (from -0.5 to 0.5) 
    vec2 centeredUv = uv-0.5;

    // vec3 origin = vec3(5., 5., -5.);
    vec3 origin = camera_uniforms.Position;
    vec3 dir = normalize( vec3(centeredUv, 1.) ); 
    out_Color = vec4(render(origin, dir), 1.);
}
