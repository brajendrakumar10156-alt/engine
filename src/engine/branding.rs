use eframe::egui;

/// Brand Customization Manager
pub struct BrandingEngine {
    #[allow(dead_code)]
    pub app_name: String,
    #[allow(dead_code)]
    pub developer_brand: String,
}

impl BrandingEngine {
    pub fn new(app_name: &str, developer_brand: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            developer_brand: developer_brand.to_string(),
        }
    }

    /// Empty footer renderer (Clean UI without bottom text)
    pub fn render_credit_footer(&self, _ui: &mut egui::Ui) {
        // Footer line removed completely
    }
}
