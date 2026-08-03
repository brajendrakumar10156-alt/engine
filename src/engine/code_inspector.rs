use std::fs;
use std::path::Path;
use eframe::egui;

/// Layer Technology Type detected for a specific UI region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerTechnology {
    UltralightHTML,     // Standard HTML/CSS/JS DOM UI Engine
    WebGPUNative,       // Native WGPU/WGSL Shader Canvas Engine (144+ FPS)
    WebGL2Fallback,     // Native WebGL2 GLSL Canvas Engine
    Canvas2DFast,       // Native Canvas 2D GPU Rendering Engine
    EguiRustNative,     // Native Egui Rust Component Engine
}

/// Description of an inspected UI layer region
#[derive(Debug, Clone)]
pub struct LayerRegion {
    pub tech: LayerTechnology,
    pub rect: egui::Rect,
    pub label: String,
}

/// Zero-Config Smart Code Inspector Output Plan
#[derive(Debug, Clone)]
pub struct LayerAllocationPlan {
    pub has_webgpu_canvas: bool,
    pub has_webgl_canvas: bool,
    pub has_canvas_2d: bool,
    pub has_html_dom: bool,
    pub has_egui_rust: bool,
    pub regions: Vec<LayerRegion>,
}

/// Zero-Config Smart Code Inspector & AST Analyzer
/// Automatically scans web project bundles (`dist/index.html` and `*.js` assets)
/// for WebGPU, WebGL2, Canvas 2D, Ultralight HTML, and Egui Rust engine signatures.
pub struct CodeInspector;

impl CodeInspector {
    /// Automatically inspects a project directory (`dist/`) and generates an Adaptive Layer Allocation Plan
    pub fn inspect_project<P: AsRef<Path>>(project_dir: P, screen_size: egui::Vec2) -> LayerAllocationPlan {
        let dir = project_dir.as_ref();
        let index_html_path = dir.join("index.html");
        let assets_dir = dir.join("assets");

        log::info!("CodeInspector: Auto-inspecting code signatures in {:?}...", dir);

        let mut has_webgpu_canvas = false;
        let mut has_webgl_canvas = false;
        let mut has_canvas_2d = false;
        let has_html_dom = true;
        let mut has_egui_rust = false;

        // 1. Scan index.html for DOM elements, Canvas 2D, and Egui tags
        if index_html_path.exists() {
            if let Ok(html_content) = fs::read_to_string(&index_html_path) {
                if html_content.contains("canvas") || html_content.contains("chart") {
                    has_webgpu_canvas = true;
                }
                if html_content.contains("2d") || html_content.contains("getContext('2d')") {
                    has_canvas_2d = true;
                }
                if html_content.contains("egui") || html_content.contains("data-engine=\"egui\"") || html_content.contains("rust-egui") {
                    has_egui_rust = true;
                }
            }
        }

        // 2. Scan JS assets for WebGPU, WebGL2, Canvas 2D, and Egui engine signatures
        if assets_dir.exists() && assets_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(assets_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("js") {
                        if let Ok(js_code) = fs::read_to_string(&path) {
                            if js_code.contains("navigator.gpu")
                                || js_code.contains("requestAdapter")
                                || js_code.contains("lightweight-charts")
                                || js_code.contains("getContext(\"webgpu\")")
                                || js_code.contains("createShaderModule")
                            {
                                has_webgpu_canvas = true;
                                log::info!("CodeInspector: Detected Native WebGPU Signature in {:?}", path.file_name());
                            }

                            if js_code.contains("webgl2") || js_code.contains("getContext(\"webgl2\")") || js_code.contains("webgl") {
                                has_webgl_canvas = true;
                                log::info!("CodeInspector: Detected Native WebGL2 Signature in {:?}", path.file_name());
                            }

                            if js_code.contains("getContext(\"2d\")") || js_code.contains("2d") || js_code.contains("CanvasRenderingContext2D") {
                                has_canvas_2d = true;
                                log::info!("CodeInspector: Detected Native Canvas 2D Signature in {:?}", path.file_name());
                            }

                            if js_code.contains("egui") || js_code.contains("eframe") || js_code.contains("rust_egui") {
                                has_egui_rust = true;
                                log::info!("CodeInspector: Detected Native Egui Signature in {:?}", path.file_name());
                            }
                        }
                    }
                }
            }
        }

        // 3. Compute Adaptive Layer Allocation Regions based on Code Signatures
        let mut regions = Vec::new();

        // Region A: Egui Native Overlay / Toolbar Slot (If Egui signatures detected or enabled)
        if has_egui_rust {
            regions.push(LayerRegion {
                tech: LayerTechnology::EguiRustNative,
                rect: egui::Rect::from_min_size(
                    egui::pos2(0.0, 0.0),
                    egui::vec2(220.0, screen_size.y),
                ),
                label: "Native Egui Rust Engine Layer".to_string(),
            });
        }

        // Region B: Top Header & Left Toolbar HTML Overlay
        regions.push(LayerRegion {
            tech: LayerTechnology::UltralightHTML,
            rect: egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(screen_size.x, 42.0),
            ),
            label: "Ultralight HTML/CSS DOM Layer".to_string(),
        });

        // Region C: Native WebGPU / WebGL / Canvas 2D Candlestick Chart (Layering Punching Zone)
        if has_webgpu_canvas || has_webgl_canvas || has_canvas_2d {
            let chart_x = if has_egui_rust { 220.0 } else { 52.0 };
            let chart_y = 42.0;
            let chart_w = (screen_size.x - chart_x - 56.0).max(100.0);
            let chart_h = (screen_size.y - 78.0).max(100.0);

            let tech = if has_webgpu_canvas {
                LayerTechnology::WebGPUNative
            } else if has_webgl_canvas {
                LayerTechnology::WebGL2Fallback
            } else {
                LayerTechnology::Canvas2DFast
            };

            regions.push(LayerRegion {
                tech,
                rect: egui::Rect::from_min_size(
                    egui::pos2(chart_x, chart_y),
                    egui::vec2(chart_w, chart_h),
                ),
                label: "Native WGPU/WebGL/2D Graphics Layer (Layering Punching)".to_string(),
            });
        }

        LayerAllocationPlan {
            has_webgpu_canvas,
            has_webgl_canvas,
            has_canvas_2d,
            has_html_dom,
            has_egui_rust,
            regions,
        }
    }
}
