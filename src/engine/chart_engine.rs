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

        // WGSL Shader for Native WebGPU Chart rendering with SDF Smoothness
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("chart_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                r#"
                struct VertexOutput {
                    @builtin(position) clip_position: vec4<f32>,
                    @location(0) uv: vec2<f32>,
                };

                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
                    var out: VertexOutput;
                    var pos = array<vec2<f32>, 3>(
                        vec2<f32>(0.0, 0.5),
                        vec2<f32>(-0.5, -0.5),
                        vec2<f32>(0.5, -0.5)
                    );
                    let p = pos[in_vertex_index];
                    out.clip_position = vec4<f32>(p, 0.0, 1.0);
                    out.uv = p * 0.5 + 0.5;
                    return out;
                }

                @fragment
                fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                    // SDF Circle Dot calculation for non-pixelated smooth chart nodes
                    let center = vec2<f32>(0.5, 0.5);
                    let dist = length(in.uv - center);
                    let alpha = smoothstep(0.4, 0.38, dist);
                    return vec4<f32>(0.0, 1.0, 0.5, alpha); // Emerald Green WebGPU Candlestick Node
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
    #[allow(dead_code)]
    pub punch_rects: Vec<egui::Rect>,
}

impl egui_wgpu::CallbackTrait for ChartCallback {
    fn paint<'a>(
        &'a self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        _cb_resources: &'a egui_wgpu::CallbackResources,
    ) {
        let clip_rect = info.clip_rect;
        let ppp = info.pixels_per_point;
        
        let x = (clip_rect.min.x * ppp).max(0.0) as u32;
        let y = (clip_rect.min.y * ppp).max(0.0) as u32;
        let width = (clip_rect.width() * ppp).max(1.0) as u32;
        let height = (clip_rect.height() * ppp).max(1.0) as u32;

        // --- LAYERING PUNCHING (Scissor Rect GPU Culling) ---
        // Apply GPU scissor rect so WGPU only draws pixels inside the punched canvas viewport
        render_pass.set_scissor_rect(x, y, width, height);

        render_pass.set_pipeline(&self.engine.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
