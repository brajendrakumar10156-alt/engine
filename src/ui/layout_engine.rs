use eframe::egui;
use crate::engine::ultralight_engine::DirtyRect;

#[derive(Clone)]
pub struct LayoutRects {
    pub egui_rects: Vec<egui::Rect>,
    pub wgpu_rects: Vec<egui::Rect>,
    pub html_punch_rects: Vec<egui::Rect>,
}

/// Dynamic Layout Allocator & Punching Manager
/// Manages variable-sized UI regions (chota 10px tooltips to bada 1920px modals)
/// and calculates stencil/scissor punching zones for the native WGPU WebGPU/WebGL canvas.
pub struct LayoutEngine {
    pub current_layout: LayoutRects,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            current_layout: LayoutRects {
                egui_rects: Vec::new(),
                wgpu_rects: Vec::new(),
                html_punch_rects: Vec::new(),
            },
        }
    }

    /// Calculates dynamic tiling and punching zones based on screen size and active UI elements
    pub fn calculate_tiling(&mut self, screen_size: egui::Vec2, dynamic_html_rects: &[DirtyRect]) {
        self.current_layout.egui_rects.clear();
        self.current_layout.wgpu_rects.clear();
        self.current_layout.html_punch_rects.clear();

        // 1. Egui Left Panel (Native Control Sidebar)
        let ui_width = (screen_size.x * 0.20).max(200.0);
        self.current_layout.egui_rects.push(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(ui_width, screen_size.y),
        ));

        // 2. Native WebGPU Canvas gets remaining screen area
        let main_canvas = egui::Rect::from_min_size(
            egui::pos2(ui_width, 0.0),
            egui::vec2((screen_size.x - ui_width).max(100.0), screen_size.y),
        );
        self.current_layout.wgpu_rects.push(main_canvas);

        // 3. Register Layering Punching Zones (both chota buttons and bada modals)
        for dirty_rect in dynamic_html_rects {
            if dirty_rect.is_active {
                let rect = dirty_rect.to_egui_rect();
                self.current_layout.html_punch_rects.push(rect);
            }
        }
    }
}
