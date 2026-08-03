use eframe::egui;

/// Pre-defined Vector & System Icon Types for Egui & Rust
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconType {
    Chart,
    Settings,
    LightningHFT,
    ExportExcel,
    ExportPdf,
    SecurityShield,
    Bluetooth,
    WebRTC,
    USB,
    ThemePalette,
    NotificationBell,
}

impl IconType {
    pub fn symbol(&self) -> &'static str {
        match self {
            IconType::Chart => "📊",
            IconType::Settings => "⚙",
            IconType::LightningHFT => "⚡",
            IconType::ExportExcel => "📋",
            IconType::ExportPdf => "📄",
            IconType::SecurityShield => "🛡",
            IconType::Bluetooth => "📶",
            IconType::WebRTC => "🎥",
            IconType::USB => "🔌",
            IconType::ThemePalette => "🎨",
            IconType::NotificationBell => "🔔",
        }
    }
}

/// Native Rust & Egui RGBA Icon Library Engine
/// Supports rendering vector icons with full 32-bit RGBA Color Tinting (16.7M Colors + Alpha)
pub struct IconEngine;

impl IconEngine {
    /// Renders a single icon in custom 32-bit RGBA color
    pub fn render_icon(
        ui: &mut egui::Ui,
        icon: IconType,
        size: f32,
        rgba_color: egui::Color32,
    ) {
        ui.label(
            egui::RichText::new(icon.symbol())
                .size(size)
                .color(rgba_color),
        );
    }

    /// Renders an interactive button with RGBA Icon & Label
    pub fn render_icon_button(
        ui: &mut egui::Ui,
        icon: IconType,
        label: &str,
        rgba_color: egui::Color32,
    ) -> bool {
        let mut clicked = false;
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(icon.symbol()).size(16.0).color(rgba_color));
            if ui.button(label).clicked() {
                clicked = true;
            }
        });
        clicked
    }
}
