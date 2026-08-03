use eframe::egui;

/// Brand Customization & Core Engine Credit Manager
/// Allows developers to customize their app name & logo while preserving mandatory Core Engine Credits.
pub struct BrandingEngine {
    pub app_name: String,
    pub developer_brand: String,
}

impl BrandingEngine {
    pub const ENGINE_CORE_CREDIT: &'static str = "Powered by Smart Brain Engine v0.1.0 — Architecture by Satyam & Team";

    pub fn new(app_name: &str, developer_brand: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            developer_brand: developer_brand.to_string(),
        }
    }

    /// Renders mandatory footer credit banner inside Egui UI
    pub fn render_credit_footer(&self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("App: {}", self.app_name)).strong());
            ui.label(" | ");
            ui.label(egui::RichText::new(format!("By: {}", self.developer_brand)).color(egui::Color32::LIGHT_BLUE));
            ui.label(" | ");
            ui.label(egui::RichText::new(Self::ENGINE_CORE_CREDIT).small().color(egui::Color32::GRAY));
        });
    }
}
