float modulo(float x, float y) { return x - y * floor(x/y); }
vec2  modulo(vec2  x, vec2  y) { return x - y * floor(x/y); }
vec3  modulo(vec3  x, vec3  y) { return x - y * floor(x/y); }

float sphereSDF(vec3 p, float radius) { return length(p) - radius; }

// vec3 arrayOp(vec3 p, vec3 offset) {
//     const vec3 halfOffset = 0.5 * offset;
//     return mod(p + halfOffset, offset) - halfOffset;
// }
vec3 arrayOp(vec3 p, vec3 offset) {
    const vec3 halfOffset = 0.5 * offset;
    return modulo(p + halfOffset, offset) - halfOffset;
}

