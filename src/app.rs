use std::path::Path;

use oxyde::{wgpu_utils::shaders, AppState};

use oxyde::wgpu_utils::uniform_buffer::UniformBuffer;
use oxyde::wgpu_utils::binding_builder::{BindGroupLayoutBuilder, BindGroupBuilder};
use oxyde::wgpu_utils::binding_glsl;
use anyhow::Result;
use wgpu::RenderPipeline;

use crate::camera::{Camera, CameraUniformBufferContent, UpdatableFromInputState};

use crate::runtime_shader_builder::gen_scene_shader;

pub struct B0oundsApp {
    pipeline: RenderPipeline,
    camera: Camera,
    camera_uniform_buffer: UniformBuffer<CameraUniformBufferContent>,
    camera_bind_group: wgpu::BindGroup,
}

fn aspect_ratio(app_state: &AppState) -> f32 {
    app_state.config.width as f32 / app_state.config.height as f32
}

impl oxyde::App for B0oundsApp {
    fn create(_app_state: &mut AppState) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<wgpu::Error>();
        _app_state.device.on_uncaptured_error( Box::new(move |e: wgpu::Error| {
            tx.send(e).expect("sending error failed");
        }));

        let vert_shader_module = shaders::load_glsl_shader_module_from_path(
            &_app_state.device,
            Path::new("shaders/Screen.vert"),
            "main").unwrap();
        
        let shader_code = gen_scene_shader();
        let frag_shader_module = shaders::load_glsl_shader_module_from_string(
            &_app_state.device,
            &shader_code,
            shaders::ShaderKind::Fragment,
            "main",
            vec!["shaders/"],
            Some("mainFragmentShader"),
        ).unwrap();
            
        // Pipeline
        let primitive_state = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        };

        let camera = Camera::default().with_position(nalgebra_glm::vec3(1.0, 0.0, 1.0));

        let camera_uniform_content = camera.uniform_buffer_content(aspect_ratio(_app_state));
        let camera_uniform_buffer = UniformBuffer::new_with_data(&_app_state.device, &camera_uniform_content);

        let multisample_state = wgpu::MultisampleState::default();
        
        let camera_bind_group_desc = BindGroupLayoutBuilder::new()
            .add_binding_rendering(binding_glsl::uniform())
            .create(&_app_state.device, Some("Camera"));
        
        let camera_bind_group = BindGroupBuilder::new(&camera_bind_group_desc)
            .resource(camera_uniform_buffer.binding_resource())
            .create(&_app_state.device, Some("Camera"));

        let pipeline = _app_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Screen Render Pipeline"),
            layout: Some(&_app_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Screen Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_desc.layout],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState {
                module: &vert_shader_module.module,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader_module.module,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: _app_state.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
        });

        _app_state.device.on_uncaptured_error(Box::new(|err| panic!("{}", err)));

        if let Ok(err) = rx.try_recv() {
            panic!("{}", err);
        }

        Self {
            pipeline,
            camera,
            camera_uniform_buffer,
            camera_bind_group,
        }
    }

    fn handle_event(&mut self, _app_state: &mut AppState, _event: &winit::event::Event<()>) -> Result<()> { Ok(()) }

    fn render_gui(&mut self, _ctx: &egui::Context) -> Result<()> {
        egui::TopBottomPanel::top("top_panel").resizable(true).show(&_ctx, |ui| {
            egui::menu::bar(ui, |_ui| {});
        });

        egui::SidePanel::right("Inspector").resizable(true).show(&_ctx, |ui| {
            ui.heading("Inspector");

            egui::CollapsingHeader::new("Display Options").default_open(true).show(ui, |_ui| {});
        });

        Ok(())
    }

    fn update(&mut self, _app_state: &mut AppState) -> Result<()> { 
        self.camera.update_from_input_state(&_app_state.input_state, _app_state.system_state.delta_time as f32);
        
        self.camera_uniform_buffer.update_content(&_app_state.queue, self.camera.uniform_buffer_content(aspect_ratio(_app_state)));

        Ok(())
    }

    fn render(
        &mut self,
        _app_state: &mut AppState,
        _encoder: &mut wgpu::CommandEncoder,
        _output_view: &wgpu::TextureView,
    ) -> Result<(), wgpu::SurfaceError> {
        // render on screen
        {
            let mut screen_render_pass = _encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Screen Render Pass"),
                color_attachments: &[
                    // This is what [[location(0)]] in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &_output_view,
                        resolve_target: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true },
                    }),
                ],
                depth_stencil_attachment: None,
            });

            // Update viewport accordingly to the Ui to display in the available rect
            // It must be multiplied by window scale factor as render pass use physical pixels screen size

            let window_scale_factor = _app_state.window.scale_factor() as f32;
            let available_rect = _app_state.gui.available_rect;
            let available_rect_size = available_rect.size();
            
            screen_render_pass.set_viewport(
                available_rect.min.x * window_scale_factor,
                available_rect.min.y * window_scale_factor,
                available_rect_size.x * window_scale_factor,
                available_rect_size.y * window_scale_factor,
                0.0,
                1.0,
            );

            screen_render_pass.set_pipeline(&self.pipeline);
            screen_render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            screen_render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}
