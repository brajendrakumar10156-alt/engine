use eframe::egui;

pub struct EventRouter {
    pub is_mouse_over_html: bool,
}

impl EventRouter {
    pub fn new() -> Self {
        Self {
            is_mouse_over_html: false,
        }
    }

    /// Evaluates if mouse is over HTML layer, dynamically routes event
    pub fn route_mouse_event(&mut self, pos: egui::Pos2, html_layers: &[egui::Rect]) -> bool {
        self.is_mouse_over_html = html_layers.iter().any(|r| r.contains(pos));
        
        if self.is_mouse_over_html {
            log::info!("Mouse is over HTML layer. Routing event to Ultralight/Webview");
            // Forward to webview...
            true // handled by HTML
        } else {
            false // Handled by Egui / Native WebGPU
        }
    }
}
