#define MAX_STEPS 100
#define MAX_DIST  200.
#define SURF_DIST .001

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