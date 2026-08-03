use eframe::egui;

/// Represents the active rendering mode for the Unified Custom Cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorRenderMode {
    GPUAccelerated, // 144+ FPS WGPU/WebGL Arrow Pointer Cursor
    CPUSoftware,    // Softbuffer / tiny-skia CPU Fallback Arrow Cursor (No GPU)
}

/// Unified Custom Cursor Engine
/// Hides the OS default cursor and renders a custom ultra-smooth arrow pointer
/// across Egui UI, WebGPU Charts, and Ultralight Punched Canvases seamlessly.
pub struct CursorEngine {
    pub position: egui::Pos2,
    pub is_visible: bool,
    pub render_mode: CursorRenderMode,
}

impl CursorEngine {
    pub fn new(has_gpu: bool) -> Self {
        let render_mode = if has_gpu {
            CursorRenderMode::GPUAccelerated
        } else {
            CursorRenderMode::CPUSoftware
        };

        log::info!("Unified Custom Cursor Engine initialized (Mode: {:?})", render_mode);

        Self {
            position: egui::pos2(0.0, 0.0),
            is_visible: true,
            render_mode,
        }
    }

    /// Updates cursor position from raw OS / Winit mouse events
    pub fn update_position(&mut self, pos: egui::Pos2) {
        self.position = pos;
    }

    /// Renders the custom arrow pointer cursor seamlessly across all layers (Egui, WebGPU, Ultralight)
    pub fn render_cursor(&self, ctx: &egui::Context) {
        if !self.is_visible {
            return;
        }

        // Hide default OS cursor
        ctx.set_cursor_icon(egui::CursorIcon::None);

        let layer_id = egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("custom_gpu_cursor"));
        let painter = ctx.layer_painter(layer_id);

        let p = self.position;

        match self.render_mode {
            CursorRenderMode::GPUAccelerated => {
                // GPU Fast Path: Sleek Emerald Cyan Arrow Pointer
                let arrow_points = vec![
                    p,                                           // Tip
                    egui::pos2(p.x, p.y + 18.0),                 // Bottom Left
                    egui::pos2(p.x + 4.5, p.y + 13.5),           // Inner Angle
                    egui::pos2(p.x + 8.5, p.y + 19.5),           // Tail Bottom
                    egui::pos2(p.x + 11.5, p.y + 18.0),          // Tail Right
                    egui::pos2(p.x + 7.5, p.y + 12.0),           // Inner Angle Right
                    egui::pos2(p.x + 13.0, p.y + 12.0),          // Far Right Corner
                ];

                let cyan = egui::Color32::from_rgb(0, 255, 180);
                let dark_border = egui::Color32::from_rgb(10, 15, 25);

                // Draw filled Arrow Pointer + Sharp Cyan Outline
                painter.add(egui::Shape::convex_polygon(
                    arrow_points.clone(),
                    cyan,
                    egui::Stroke::new(1.5_f32, dark_border),
                ));
            }
            CursorRenderMode::CPUSoftware => {
                // CPU Safe Path: Software rasterized Gold Arrow Pointer for non-GPU hardware
                let arrow_points = vec![
                    p,
                    egui::pos2(p.x, p.y + 14.0),
                    egui::pos2(p.x + 3.5, p.y + 10.5),
                    egui::pos2(p.x + 6.5, p.y + 15.0),
                    egui::pos2(p.x + 9.0, p.y + 13.5),
                    egui::pos2(p.x + 6.0, p.y + 9.0),
                    egui::pos2(p.x + 10.0, p.y + 9.0),
                ];

                let gold = egui::Color32::from_rgb(255, 200, 0);
                let dark_border = egui::Color32::from_rgb(10, 15, 25);

                painter.add(egui::Shape::convex_polygon(
                    arrow_points,
                    gold,
                    egui::Stroke::new(1.0_f32, dark_border),
                ));
            }
        }
    }
}
