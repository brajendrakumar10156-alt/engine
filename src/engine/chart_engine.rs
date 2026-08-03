use eframe::egui;
use eframe::egui_wgpu;
use std::borrow::Cow;
use std::sync::Arc;
use wgpu;

pub struct ChartEngine {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ChartEngine {
    pub fn new(wgpu_render_state: &egui_wgpu::RenderState) -> Self {
        let device = &wgpu_render_state.device;

        // Basic WGSL Shader for testing native WebGPU Chart rendering inside Egui
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("chart_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                r#"
                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                    var pos = array<vec2<f32>, 3>(
                        vec2<f32>(0.0, 0.5),
                        vec2<f32>(-0.5, -0.5),
                        vec2<f32>(0.5, -0.5)
                    );
                    return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(0.0, 1.0, 0.0, 1.0); // Green color for Chart
                }
                "#,
            )),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("chart_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("chart_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu_render_state.target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { render_pipeline }
    }
}

pub struct ChartCallback {
    pub engine: Arc<ChartEngine>,
}

impl egui_wgpu::CallbackTrait for ChartCallback {
    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        _cb_resources: &'a egui_wgpu::CallbackResources,
    ) {
        // Here we draw directly to the GPU bypassing HTML entirely
        render_pass.set_pipeline(&self.engine.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
