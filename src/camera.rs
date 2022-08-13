use oxyde::InputsState;

use nalgebra_glm as glm;

use bytemuck::{Pod, Zeroable};

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
}

pub 
trait UpdatableFromInputState {
    fn update_from_input_state(&mut self, input_state: &InputsState, delta_time: f32);
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: glm::Vec3::zeros(),
            direction: glm::Vec3::z(),
            up: glm::Vec3::y(),

            translation_speed: 8.0,
            rotation_speed: 1.0,
        }
    }
}

impl Camera {
    pub fn with_position(self, position: glm::Vec3) -> Self {
        Self { position, ..self }
    }

    pub fn with_direction(self, direction: glm::Vec3) -> Self {
        Self { direction, ..self }
    }
}

impl UpdatableFromInputState for Camera {
    fn update_from_input_state(&mut self, input_state: &InputsState, delta_time: f32) {
        let right: glm::Vec3 = self.direction.cross(&self.up).normalize();

        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::Z) {
            self.position += self.direction * self.translation_speed * delta_time;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::S) {
            self.position -= self.direction * self.translation_speed * delta_time;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::Q) {
            self.position -= right * self.translation_speed * delta_time;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::D) {
            self.position += right * self.translation_speed * delta_time;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::Space) {
            self.position += self.up * self.translation_speed * delta_time;
        }
        if input_state.is_key_pressed(&winit::event::VirtualKeyCode::LControl) {
            self.position -= self.up * self.translation_speed * delta_time;
        }

        if input_state.mouse.is_left_clicked {
            let rotation_updown= glm::quat_angle_axis(-input_state.mouse.position_delta.y * glm::pi::<f32>() / 180.0f32 * self.rotation_speed * delta_time, &right);
            let rotation_leftright = glm::quat_angle_axis(-input_state.mouse.position_delta.x * glm::pi::<f32>() / 180.0f32 * self.rotation_speed * delta_time, &self.up);

            self.direction = glm::quat_rotate_vec3(&(rotation_updown + rotation_leftright), &self.direction);
        }
    }
}

impl Camera {
    pub fn uniform_buffer_content(&self, aspect_ratio: f32) -> CameraUniformBufferContent {

        let opengl_projection_to_wgpu_projection: glm::Mat4 = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );

        let view = glm::look_at_rh(&self.position, &(self.position + self.direction), &self.up);
        let projection = opengl_projection_to_wgpu_projection * glm::perspective(aspect_ratio, VERTICAL_FOV * glm::pi::<f32>() / 180.0f32, 0.01, 1000.0);
        let view_projection = projection * view;
        let inverse_projection = glm::inverse(&projection);
        assert!(inverse_projection != glm::Mat4::zeros(), "Unable to inverse the projection matrix");

        CameraUniformBufferContent {
            view_projection,
        }
    }
}
