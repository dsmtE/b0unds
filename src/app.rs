use std::path::Path;

use oxyde::{wgpu_utils::shaders::load_glsl_shader_module_from_path, AppState};

use anyhow::Result;
use wgpu::RenderPipeline;

pub struct B0oundsApp {
    pipeline: RenderPipeline,
}

impl oxyde::App for B0oundsApp {
    fn create(_app_state: &mut AppState) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<wgpu::Error>();
        _app_state.device.on_uncaptured_error(move |e: wgpu::Error| {
            tx.send(e).expect("sending error failed");
        });

        let frag_shader_module = load_glsl_shader_module_from_path(&_app_state.device, Path::new("shaders/Screen.frag"), "main").unwrap();
        let vert_shader_module = load_glsl_shader_module_from_path(&_app_state.device, Path::new("shaders/Screen.vert"), "main").unwrap();

        // Pipeline
        let primitive_state = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        };

        let multisample_state = wgpu::MultisampleState::default();

        let pipeline = _app_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Screen Render Pipeline"),
            layout: Some(&_app_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Screen Pipeline Layout"),
                bind_group_layouts: &[],
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
                targets: &[wgpu::ColorTargetState {
                    format: _app_state.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
        });

        _app_state.device.on_uncaptured_error(|err| panic!("{}", err));

        if let Ok(err) = rx.try_recv() {
            panic!("{}", err);
        }

        Self { pipeline }
    }

    fn handle_event(&mut self, _app_state: &mut AppState, _event: &winit::event::Event<()>) -> Result<()> { Ok(()) }

    fn render_gui(&mut self, _ctx: &epi::egui::Context) -> Result<()> {
        egui::TopBottomPanel::top("top_panel").resizable(true).show(&_ctx, |ui| {
            egui::menu::bar(ui, |ui| {});
        });

        egui::SidePanel::right("Inspector").resizable(true).show(&_ctx, |ui| {
            ui.heading("Inspector");

            egui::CollapsingHeader::new("Display Options").default_open(true).show(ui, |ui| {});
        });

        Ok(())
    }

    fn update(&mut self, _app_state: &mut AppState) -> Result<()> { Ok(()) }

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
                    wgpu::RenderPassColorAttachment {
                        view: &_output_view,
                        resolve_target: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true },
                    },
                ],
                depth_stencil_attachment: None,
            });

            screen_render_pass.set_pipeline(&self.pipeline);
            screen_render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}
