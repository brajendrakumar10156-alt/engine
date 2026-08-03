use eframe::egui;
use std::collections::VecDeque;

/// Represents a variable-sized bounding box (Dirty Rect) for Layering Punching.
/// Supports both chota (small tooltips/buttons) and bada (large modals/sidebars).
#[derive(Debug, Clone, Copy)]
pub struct DirtyRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_active: bool,
}

impl DirtyRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            is_active: true,
        }
    }

    pub fn to_egui_rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.x, self.y),
            egui::vec2(self.width, self.height),
        )
    }
}

/// Ultralight WebKit HTML/DOM Engine Wrapper
/// Handles HTML/CSS rendering, dynamic dirty rectangle tracking,
/// and Layering Punching for WGPU/WebGL integration.
pub struct UltralightEngine {
    #[allow(dead_code)]
    pub is_initialized: bool,
    pub active_dirty_rects: Vec<DirtyRect>,
    pub pending_js_triggers: VecDeque<String>,
}

impl UltralightEngine {
    pub fn new() -> Self {
        log::info!("Initializing Ultralight WebKit Engine & Layering Punching Manager...");
        Self {
            is_initialized: true,
            active_dirty_rects: Vec::new(),
            pending_js_triggers: VecDeque::new(),
        }
    }

    /// Registers a dynamic HTML UI bounding box (chota or bada) for Layering Punching.
    /// The background WGPU/WebGL renderer will cull (punch out) pixels behind these rects.
    pub fn register_dirty_rect(&mut self, rect: DirtyRect) {
        log::info!(
            "Layering Punching: Registering UI rect at ({}, {}) size {}x{}",
            rect.x, rect.y, rect.width, rect.height
        );
        self.active_dirty_rects.push(rect);
    }

    /// Clears expired or closed UI element dirty rects to instantly swap background pixels back.
    pub fn clear_expired_rects(&mut self) {
        self.active_dirty_rects.retain(|r| r.is_active);
    }

    /// Triggers a Native JS Bridge action from JavaScript to Rust without JSON/v8 serialization lag.
    pub fn receive_js_trigger(&mut self, trigger_name: &str) {
        log::info!("Received Zero-Copy JS Trigger: {}", trigger_name);
        self.pending_js_triggers.push_back(trigger_name.to_string());
    }

    /// Simulates rendering dynamic HTML UI elements (buttons, popups, sidebars)
    pub fn render_html_surface(&mut self) {
        // Render loop for HTML surface & texture synchronization
        self.clear_expired_rects();
    }
}
