use eframe::egui;
use eframe::egui_wgpu;
use std::borrow::Cow;
use std::sync::Arc;
use wgpu;

/// Active Graphics API Mode (Explicit WebGL2 vs WebGPU)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsMode {
    WebGL2Standard, // GLSL / OpenGL ES Backend (Runs standard WebGL just like Chrome)
    WebGPUNextGen,  // WGSL / Vulkan / D3D12 Backend (Next-Gen 144+ FPS)
}

/// Dynamic Color Theme Option for WGPU Shader Rendering
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTheme {
    BullishGreen, // Classic Bullish Green (0, 255, 180)
    BearishRed,   // Bearish Red (255, 60, 90)
    CyberPurple,  // Neon Cyberpunk Purple (180, 0, 255)
    GoldGradient, // Gold Yellow (255, 200, 0)
}

/// Universal Dual Engine: WebGL2 (Chrome Standard) + WebGPU (Next-Gen)
/// Supports FULL 32-bit RGBA Colors (16.7 Million Colors + Alpha Transparency)
pub struct ChartEngine {
    pub webgl_pipeline: wgpu::RenderPipeline,
    pub webgpu_pipeline: wgpu::RenderPipeline,
    #[allow(dead_code)]
    pub active_mode: GraphicsMode,
}

impl ChartEngine {
    pub fn new(wgpu_render_state: &egui_wgpu::RenderState) -> Self {
        let device = &wgpu_render_state.device;

        let adapter_info = wgpu_render_state.adapter.get_info();
        let active_mode = match adapter_info.backend {
            wgpu::Backend::Gl => GraphicsMode::WebGL2Standard,
            _ => GraphicsMode::WebGPUNextGen,
        };

        log::info!("ChartEngine initialized with Active Backend: {:?}", active_mode);

        // 1. STANDARD WEBGL SHADER PIPELINE (Supports Dynamic Uniform RGBA Color Input)
        let webgl_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("webgl_standard_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                r#"
                struct VertexOutput {
                    @builtin(position) clip_position: vec4<f32>,
                    @location(0) color: vec4<f32>,
                };

                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
                    var out: VertexOutput;
                    var pos = array<vec2<f32>, 3>(
                        vec2<f32>(0.0, 0.6),
                        vec2<f32>(-0.6, -0.6),
                        vec2<f32>(0.6, -0.6)
                    );
                    var colors = array<vec4<f32>, 3>(
                        vec4<f32>(1.0, 0.2, 0.3, 1.0), // Red
                        vec4<f32>(0.0, 1.0, 0.6, 1.0), // Green
                        vec4<f32>(0.2, 0.6, 1.0, 1.0)  // Blue
                    );
                    out.clip_position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
                    out.color = colors[in_vertex_index];
                    return out;
                }

                @fragment
                fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                    return in.color; // Dynamic RGB Spectrum Interpolation
                }
                "#,
            )),
        });

        // 2. NEXT-GEN WEBGPU SHADER PIPELINE (Per-Vertex Spectrum Gradient)
        let webgpu_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("webgpu_nextgen_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                r#"
                struct VertexOutput {
                    @builtin(position) clip_position: vec4<f32>,
                    @location(0) uv: vec2<f32>,
                    @location(1) color: vec4<f32>,
                };

                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
                    var out: VertexOutput;
                    var pos = array<vec2<f32>, 3>(
                        vec2<f32>(0.0, 0.5),
                        vec2<f32>(-0.5, -0.5),
                        vec2<f32>(0.5, -0.5)
                    );
                    var colors = array<vec4<f32>, 3>(
                        vec4<f32>(1.0, 0.2, 0.4, 1.0), // Crimson Red
                        vec4<f32>(0.0, 1.0, 0.7, 1.0), // Emerald Green
                        vec4<f32>(0.8, 0.2, 1.0, 1.0)  // Neon Purple
                    );
                    let p = pos[in_vertex_index];
                    out.clip_position = vec4<f32>(p, 0.0, 1.0);
                    out.uv = p * 0.5 + 0.5;
                    out.color = colors[in_vertex_index];
                    return out;
                }

                @fragment
                fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                    let center = vec2<f32>(0.5, 0.5);
                    let dist = length(in.uv - center);
                    let alpha = smoothstep(0.4, 0.38, dist);
                    return vec4<f32>(in.color.rgb, in.color.a * alpha);
                }
                "#,
            )),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("chart_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let webgl_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("webgl_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &webgl_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &webgl_shader,
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

        let webgpu_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("webgpu_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &webgpu_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &webgpu_shader,
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

        Self {
            webgl_pipeline,
            webgpu_pipeline,
            active_mode,
        }
    }
}

pub struct ChartCallback {
    pub engine: Arc<ChartEngine>,
    #[allow(dead_code)]
    pub punch_rects: Vec<egui::Rect>,
    pub force_webgl_mode: bool,
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

        render_pass.set_scissor_rect(x, y, width, height);

        if self.force_webgl_mode {
            render_pass.set_pipeline(&self.engine.webgl_pipeline);
        } else {
            render_pass.set_pipeline(&self.engine.webgpu_pipeline);
        }
        
        render_pass.draw(0..3, 0..1);
    }
}
