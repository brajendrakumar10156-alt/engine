use eframe::egui;

/// Beautiful Icon & Design Engine
/// Supports rendering sleek SVG & Font icons in 32-bit RGBA colors.
pub struct IconEngine;

impl IconEngine {
    /// Renders a beautiful icon with text label in custom RGBA color
    pub fn render_icon_button(
        ui: &mut egui::Ui,
        icon_symbol: &str,
        label: &str,
        color: egui::Color32,
    ) -> bool {
        let mut clicked = false;
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(icon_symbol).size(16.0).color(color));
            if ui.button(label).clicked() {
                clicked = true;
            }
        });
        clicked
    }
}
