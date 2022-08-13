static TEMPLATE: &'static str = include_str!("shaders/template.frag");

pub fn gen_scene_shader() -> String {
    let mut code: String = TEMPLATE.to_owned();
    
    code = code.replace("[SDF_FUNCTION]", "float sdf(vec3 p) { return sphereSDF(arrayOp(p, vec3(10.)), 1.0); }");

    code
}