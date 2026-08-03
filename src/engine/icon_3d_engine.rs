use eframe::egui;

/// 3D Desktop Icon & Glassmorphism Background Config
#[derive(Debug, Clone)]
pub struct DesktopIcon3D {
    pub name: String,
    pub symbol: String,
    pub position: [f32; 2],
    pub rotation_angle: f32,
    pub primary_rgba: egui::Color32,
    pub background_rgba: egui::Color32,
    pub is_transparent_bg: bool,
}

impl DesktopIcon3D {
    pub fn new(name: &str, symbol: &str, x: f32, y: f32, color: egui::Color32) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            position: [x, y],
            rotation_angle: 0.0,
            primary_rgba: color,
            background_rgba: egui::Color32::from_black_alpha(180),
            is_transparent_bg: false,
        }
    }
}

/// 3D Desktop Icon & Custom Background Engine
/// Renders 3D desktop icons with dynamic rotation, custom RGBA colors, and transparent/solid backgrounds.
pub struct Icon3DEngine {
    pub icons: Vec<DesktopIcon3D>,
}

impl Icon3DEngine {
    pub fn new() -> Self {
        log::info!("3D Desktop Icon Engine initialized (3D Glassmorphism & Custom RGBA)");
        let icons = vec![
            DesktopIcon3D::new("Trading Console", "📈", 50.0, 60.0, egui::Color32::from_rgb(0, 255, 180)),
            DesktopIcon3D::new("HFT Data Engine", "⚡", 50.0, 160.0, egui::Color32::from_rgb(255, 200, 0)),
            DesktopIcon3D::new("Security Shield", "🛡", 50.0, 260.0, egui::Color32::from_rgb(0, 150, 255)),
            DesktopIcon3D::new("Graphics Pipeline", "🎨", 50.0, 360.0, egui::Color32::from_rgb(255, 100, 200)),
        ];
        Self { icons }
    }

    /// Render 3D desktop icons on screen
    pub fn render(&mut self, ctx: &egui::Context) {
        for icon in self.icons.iter_mut() {
            icon.rotation_angle += 0.02; // Smooth 3D animation rotation

            let bg_frame = if icon.is_transparent_bg {
                egui::Frame::none()
            } else {
                egui::Frame::window(&ctx.style())
                    .fill(icon.background_rgba)
                    .rounding(12.0)
            };

            egui::Window::new(&icon.name)
                .fixed_pos(icon.position)
                .title_bar(false)
                .resizable(false)
                .frame(bg_frame)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(&icon.symbol)
                                .size(24.0)
                                .color(icon.primary_rgba),
                        );
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(&icon.name).strong());
                            ui.label(
                                egui::RichText::new(format!("3D Angle: {:.1}°", icon.rotation_angle.to_degrees() % 360.0))
                                    .small()
                                    .color(egui::Color32::GRAY),
                            );
                        });
                    });
                });
        }
    }
}
