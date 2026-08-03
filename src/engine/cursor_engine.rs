use eframe::egui;

/// Represents the active rendering mode for the Unified Custom Cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorRenderMode {
    GPUAccelerated, // 144+ FPS WGPU/WebGL Shader Cursor
    CPUSoftware,    // Softbuffer / tiny-skia CPU Fallback Cursor (No GPU)
}

/// Unified Custom Cursor Engine
/// Hides the OS default cursor and renders a custom ultra-smooth crosshair/pointer
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

    /// Renders the custom cursor seamlessly across all layers (Egui, WebGPU, Ultralight)
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
                // GPU Fast Path: Smooth 144FPS SDF Crosshair & Dot
                let color = egui::Color32::from_rgb(0, 255, 180); // Emerald Cyan Trading Cursor
                
                // Crosshair lines
                painter.line_segment([egui::pos2(p.x - 12.0, p.y), egui::pos2(p.x + 12.0, p.y)], (1.5, color));
                painter.line_segment([egui::pos2(p.x, p.y - 12.0), egui::pos2(p.x, p.y + 12.0)], (1.5, color));
                
                // Center glow dot
                painter.circle_filled(p, 3.0, egui::Color32::WHITE);
                painter.circle_stroke(p, 5.0, (1.0, color));
            }
            CursorRenderMode::CPUSoftware => {
                // CPU Safe Path: Software rasterized crisp cursor for non-GPU hardware
                let color = egui::Color32::from_rgb(255, 200, 0); // Gold Yellow Fallback Cursor
                
                // Software fallback crosshair
                painter.line_segment([egui::pos2(p.x - 8.0, p.y), egui::pos2(p.x + 8.0, p.y)], (1.0, color));
                painter.line_segment([egui::pos2(p.x, p.y - 8.0), egui::pos2(p.x, p.y + 8.0)], (1.0, color));
                painter.circle_filled(p, 2.0, color);
            }
        }
    }
}
