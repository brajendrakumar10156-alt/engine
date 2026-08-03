use eframe::egui;
use crate::engine::ultralight_engine::DirtyRect;
use crate::engine::code_inspector::{CodeInspector, LayerAllocationPlan, LayerTechnology};
use std::path::Path;

#[derive(Clone)]
pub struct LayoutRects {
    pub egui_rects: Vec<egui::Rect>,
    pub wgpu_rects: Vec<egui::Rect>,
    pub html_punch_rects: Vec<egui::Rect>,
}

/// Adaptive Dynamic Layout Engine (Test No. 3 Blank Container)
/// Calculates Pure Code-Driven Layering Punching zones without hardcoded predefined pixel offsets.
pub struct LayoutEngine {
    pub current_layout: LayoutRects,
    pub current_plan: Option<LayerAllocationPlan>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            current_layout: LayoutRects {
                egui_rects: Vec::new(),
                wgpu_rects: Vec::new(),
                html_punch_rects: Vec::new(),
            },
            current_plan: None,
        }
    }

    /// Performs Zero-Config Code Inspection and calculates pure code-driven layout layers
    pub fn calculate_adaptive_tiling<P: AsRef<Path>>(
        &mut self,
        project_dir: P,
        screen_size: egui::Vec2,
        dynamic_html_rects: &[DirtyRect],
    ) {
        self.current_layout.egui_rects.clear();
        self.current_layout.wgpu_rects.clear();
        self.current_layout.html_punch_rects.clear();

        // 1. Run Smart Code Inspection on project bundle
        let plan = CodeInspector::inspect_project(project_dir, screen_size);

        // 2. Allocate Layers dynamically based on Code Inspection Results (Zero Hardcoding)
        for region in &plan.regions {
            match region.tech {
                LayerTechnology::WebGPUNative | LayerTechnology::WebGL2Fallback | LayerTechnology::Canvas2DFast => {
                    self.current_layout.wgpu_rects.push(region.rect);
                }
                LayerTechnology::EguiRustNative => {
                    self.current_layout.egui_rects.push(region.rect);
                }
                LayerTechnology::UltralightHTML => {
                    self.current_layout.html_punch_rects.push(region.rect);
                }
            }
        }

        // 3. Fallback WebGPU Canvas fitting full viewport if undetected
        if self.current_layout.wgpu_rects.is_empty() {
            self.current_layout.wgpu_rects.push(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                screen_size,
            ));
        }

        // 4. Register dynamic Layering Punching rectangles
        for dirty_rect in dynamic_html_rects {
            if dirty_rect.is_active {
                let rect = dirty_rect.to_egui_rect();
                self.current_layout.html_punch_rects.push(rect);
            }
        }

        self.current_plan = Some(plan);
    }
}
