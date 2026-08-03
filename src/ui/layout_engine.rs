use eframe::egui;
use crate::engine::ultralight_engine::DirtyRect;

#[derive(Clone)]
pub struct LayoutRects {
    pub egui_rects: Vec<egui::Rect>,
    pub wgpu_rects: Vec<egui::Rect>,
    pub html_punch_rects: Vec<egui::Rect>,
}

/// Dynamic Layout Allocator & Layering Punching Manager
/// Calculates precise Layering Punching zones for WebGPU/WebGL native chart canvas.
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

    /// Calculates dynamic tiling and Layering Punching zones matching QuantaAI Trading UI (Image 1)
    pub fn calculate_tiling(&mut self, screen_size: egui::Vec2, dynamic_html_rects: &[DirtyRect]) {
        self.current_layout.egui_rects.clear();
        self.current_layout.wgpu_rects.clear();
        self.current_layout.html_punch_rects.clear();

        // 1. QuantaAI Trading UI Layering Punching Dimensions
        let top_header_height = 42.0;
        let left_toolbar_width = 52.0;
        let right_price_scale_width = 56.0;
        let bottom_status_height = 36.0;

        // Center WebGPU Candlestick Chart Area (Layering Punching Zone)
        let chart_x = left_toolbar_width;
        let chart_y = top_header_height;
        let chart_width = (screen_size.x - left_toolbar_width - right_price_scale_width).max(100.0);
        let chart_height = (screen_size.y - top_header_height - bottom_status_height).max(100.0);

        let main_chart_rect = egui::Rect::from_min_size(
            egui::pos2(chart_x, chart_y),
            egui::vec2(chart_width, chart_height),
        );
        self.current_layout.wgpu_rects.push(main_chart_rect);

        // 2. Register Dynamic Layering Punching Rectangles
        for dirty_rect in dynamic_html_rects {
            if dirty_rect.is_active {
                let rect = dirty_rect.to_egui_rect();
                self.current_layout.html_punch_rects.push(rect);
            }
        }
    }
}
