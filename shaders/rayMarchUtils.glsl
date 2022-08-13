float sphereSDF(vec3 p, float radius) { return length(p) - radius; }

vec3 arrayOp(vec3 p, vec3 offset) {
    return mod(p + 0.5 * offset, offset) - 0.5 * offset;
}