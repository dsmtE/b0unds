use oxyde::InputsState;

use nalgebra_glm as glm;

use bytemuck::{Pod, Zeroable};

// #[cfg_attr(rustfmt, rustfmt_skip)]
// const OPENGL_PROJECTION_TO_WGPU_PROJECTION: glm::Mat4 = glm::mat4(
//     1.0, 0.0, 0.0, 0.0,
//     0.0, 1.0, 0.0, 0.0,
//     0.0, 0.0, 0.5, 0.0,
//     0.0, 0.0, 0.5, 1.0,
// );

const VERTICAL_FOV: f32 = 80.0f32;

pub struct Camera {
    pub position: glm::Vec3,
    pub direction: glm::Vec3,
    up: glm::Vec3,

    translation_speed: f32,
    rotation_speed: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniformBufferContent {
    view_projection: glm::Mat4,
    position: glm::Vec3,
    direction: glm::Vec3,
}

trait UpdatableFromInputState {
    fn update_from_input_State(&mut self, input_state: &InputsState, delta_time: f32);
}

impl Default for Camera {
    fn default() -> Self {
        let position = glm::vec3(1.0, 1.0, 1.0);
        Self {
            position,
            direction: (glm::Vec3::zeros() - position).normalize(),
            up: glm::Vec3::y(),

            translation_speed: 0.5,
            rotation_speed: 0.001,
        }
    }
}

impl UpdatableFromInputState for Camera {
    fn update_from_input_State(&mut self, input_state: &InputsState, delta_time: f32) {
        todo!();

        let right = glm::cross(&self.direction, &self.up).normalize();

        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::Z) {
            self.position += self.direction * self.translation_speed;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::S) {
            self.position -= self.direction * self.translation_speed;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::Q) {
            self.position -= right * self.translation_speed;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::D) {
            self.position += right * self.translation_speed;
        }

        let rotation_updown= glm::quat_angle_axis(-input_state.mouse.position_delta.y * glm::pi::<f32>() / 180.0f32 * self.rotation_speed, &right);
        let rotation_leftright = glm::quat_angle_axis(-input_state.mouse.position_delta.x * glm::pi::<f32>() / 180.0f32 * self.rotation_speed, &self.up);

        
        self.direction = glm::quat_rotate_vec3(&(rotation_updown + rotation_leftright), &self.direction);

        // let mult_dir: glm::Vec3 = self.direction * self.translation_speed;
        // let mut transform: glm::Mat4 = glm::identity();
        // if inputState.is_key_pressed(&winit::event::VirtualKeyCode::Z) {
        //     transform *= glm::translation(&mult_dir);
        // }
        // if inputState.is_key_pressed(&winit::event::VirtualKeyCode::S) {
        //     transform *= glm::translation(&-mult_dir);
        // }

        // if inputState.is_key_pressed(&winit::event::VirtualKeyCode::Q) {
        //     transform *= glm::translation(&(-right * self.translation_speed));
        // }
        // if inputState.is_key_pressed(&winit::event::VirtualKeyCode::D) {
        //     transform *= glm::translation(&(right * self.translation_speed));
        // }
        
        // let rotation_updown= glm::quat_angle_axis(-inputState.mouse.position_delta.y * TO_RAD * self.rotation_speed, &right);
        // let rotation_leftright = glm::quat_angle_axis(-inputState.mouse.position_delta.x * TO_RAD * self.rotation_speed, &self.up);
        // transform * glm::quat_to_mat4(&(rotation_updown + rotation_leftright));
            // glm::rotate_vec3(v, angle, normal);

        // self.direction *= transform;
//         let rotation_leftright =
//             cgmath::Quaternion::from_axis_angle(self.rotational_up, cgmath::Rad(-self.mouse_delta.0 as f32 * self.rotation_speed));
//         self.direction = (rotation_updown + rotation_leftright).rotate_vector(self.direction).normalize();

//         self.position += translation;

        
    }
}

impl Camera {
    fn Get_uniform_buffer_content(&self, aspect_ratio: f32) -> CameraUniformBufferContent {
        let view = glm::look_at_rh(&self.position, &(self.position + self.direction), &self.up);
        let projection = glm::perspective(aspect_ratio, VERTICAL_FOV * glm::pi::<f32>() / 180.0f32, 0.01, 1000.0);
        let view_projection = projection * view;
        let inverse_projection = glm::inverse(&projection);
        assert!(inverse_projection != glm::Mat4::zeros(), "Unable to inverse the projection matrix");

        CameraUniformBufferContent {
            view_projection,
            position: self.position,
            direction: self.direction,
        }
    }
}
