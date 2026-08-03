use eframe::egui;

/// Realistic System Hardware Cursor Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealisticCursorStyle {
    DefaultArrow,     // Normal Hardware OS Arrow Pointer (Clean, 0-Latency)
    CoordinateCrosshair, // Crosshair guidelines (ONLY for coordinate graphs / charts)
    HandGrab,         // Hand Grab Icon
    #[allow(dead_code)]
    TextIBeam,        // Text I-Beam
}

/// Universal Production-Grade Cursor Engine
/// Defaults to the clean Normal OS Hardware Cursor everywhere.
/// Crosshair guidelines are ONLY activated when explicitly enabled for coordinate graphs.
pub struct CursorEngine {
    pub position: egui::Pos2,
    pub active_style: RealisticCursorStyle,
    pub is_mouse_down: bool,
    pub enable_coordinate_crosshair: bool, // Set to true ONLY when rendering a coordinate graph
}

impl CursorEngine {
    pub fn new(has_gpu: bool) -> Self {
        log::info!("Universal Hardware Cursor Engine initialized (GPU: {})", has_gpu);

        Self {
            position: egui::pos2(0.0, 0.0),
            active_style: RealisticCursorStyle::DefaultArrow,
            is_mouse_down: false,
            enable_coordinate_crosshair: false, // Default is FALSE (Normal cursor everywhere)
        }
    }

    /// Updates cursor position and mouse press state from OS events
    pub fn update_state(&mut self, pos: egui::Pos2, is_down: bool) {
        self.position = pos;
        self.is_mouse_down = is_down;
    }

    /// Evaluates hover region: Uses Normal Cursor everywhere unless a coordinate graph specifically requests crosshair
    pub fn evaluate_context(&mut self, is_over_coordinate_graph: bool, is_dragging: bool) {
        if is_dragging {
            self.active_style = RealisticCursorStyle::HandGrab;
        } else if is_over_coordinate_graph && self.enable_coordinate_crosshair {
            self.active_style = RealisticCursorStyle::CoordinateCrosshair;
        } else {
            // NORMAL CURSOR FOR EVERYTHING ELSE (Egui, WebGPU 3D, HTML, General Apps)
            self.active_style = RealisticCursorStyle::DefaultArrow;
        }
    }

    /// Renders the cursor state:
    /// - Normal Cursor (Default everywhere): Clean Hardware OS Mouse Pointer
    /// - Coordinate Graph: Crosshair + Axis Guidelines ONLY if requested
    pub fn render(&self, ctx: &egui::Context, screen_size: egui::Vec2) {
        let p = self.position;

        match self.active_style {
            RealisticCursorStyle::DefaultArrow => {
                // NORMAL OS CURSOR EVERYWHERE (Clean, 0-Latency, No Background Overlay)
                if self.is_mouse_down {
                    ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                } else {
                    ctx.set_cursor_icon(egui::CursorIcon::Default);
                }
            }
            RealisticCursorStyle::HandGrab => {
                if self.is_mouse_down {
                    ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                } else {
                    ctx.set_cursor_icon(egui::CursorIcon::Grab);
                }
            }
            RealisticCursorStyle::TextIBeam => {
                ctx.set_cursor_icon(egui::CursorIcon::Text);
            }
            RealisticCursorStyle::CoordinateCrosshair => {
                // Crosshair ONLY when over a Coordinate Graph / Chart
                ctx.set_cursor_icon(egui::CursorIcon::Crosshair);

                let layer_id = egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("coordinate_axis_overlay"));
                let painter = ctx.layer_painter(layer_id);

                let cyan = egui::Color32::from_rgb(0, 255, 180);
                let axis_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_rgba_unmultiplied(0, 255, 180, 100));

                // Full-screen X & Y Guidelines
                painter.line_segment([egui::pos2(0.0, p.y), egui::pos2(screen_size.x, p.y)], axis_stroke);
                painter.line_segment([egui::pos2(p.x, 0.0), egui::pos2(p.x, screen_size.y)], axis_stroke);

                // Center node dot
                painter.circle_filled(p, 2.5_f32, egui::Color32::WHITE);
                painter.circle_stroke(p, 4.5_f32, (1.0_f32, cyan));
            }
        }
    }
}
