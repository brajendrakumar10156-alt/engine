use eframe::egui;

/// Dynamic Universal 32-bit RGBA Theme Palette
pub struct ThemeEngine {
    pub bg_rgba: [u8; 4],
    pub panel_rgba: [u8; 4],
    pub accent_rgba: [u8; 4],
    pub text_rgba: [u8; 4],
    pub is_glassmorphism: bool,
}

impl ThemeEngine {
    pub fn new() -> Self {
        Self {
            bg_rgba: [18, 24, 38, 255],     // Dark Cyber Blue (#121826)
            panel_rgba: [30, 40, 60, 220],   // Translucent Panel Blue
            accent_rgba: [0, 255, 180, 255],  // Neon Emerald Cyan
            text_rgba: [240, 245, 255, 255],  // Soft Pure White
            is_glassmorphism: true,
        }
    }

    #[allow(dead_code)]
    pub fn bg_color32(&self) -> egui::Color32 {
        let a = if self.is_glassmorphism { 180 } else { self.bg_rgba[3] };
        egui::Color32::from_rgba_unmultiplied(self.bg_rgba[0], self.bg_rgba[1], self.bg_rgba[2], a)
    }

    pub fn panel_color32(&self) -> egui::Color32 {
        let a = if self.is_glassmorphism { 200 } else { self.panel_rgba[3] };
        egui::Color32::from_rgba_unmultiplied(self.panel_rgba[0], self.panel_rgba[1], self.panel_rgba[2], a)
    }

    pub fn accent_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(self.accent_rgba[0], self.accent_rgba[1], self.accent_rgba[2], self.accent_rgba[3])
    }

    pub fn text_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(self.text_rgba[0], self.text_rgba[1], self.text_rgba[2], self.text_rgba[3])
    }

    /// Render live RGBA theme customizer control UI
    pub fn render_customizer(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎨 Universal RGBA Theme Customizer");
        ui.checkbox(&mut self.is_glassmorphism, "Enable Translucent Glassmorphism");

        ui.horizontal(|ui| {
            ui.label("Background RGBA:");
            ui.color_edit_button_srgba_unmultiplied(&mut self.bg_rgba);
        });

        ui.horizontal(|ui| {
            ui.label("Panel RGBA:");
            ui.color_edit_button_srgba_unmultiplied(&mut self.panel_rgba);
        });

        ui.horizontal(|ui| {
            ui.label("Accent RGBA:");
            ui.color_edit_button_srgba_unmultiplied(&mut self.accent_rgba);
        });

        ui.horizontal(|ui| {
            ui.label("Text RGBA:");
            ui.color_edit_button_srgba_unmultiplied(&mut self.text_rgba);
        });
    }
}
