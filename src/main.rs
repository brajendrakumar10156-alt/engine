#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod engine;

use eframe::egui;
use eframe::egui_wgpu;
use ui::layout_engine::LayoutEngine;
use ui::event_router::EventRouter;
use engine::chart_engine::ChartEngine;
use engine::ultralight_engine::{UltralightEngine, DirtyRect};
use engine::cursor_engine::CursorEngine;
use engine::hft_engine::HftEngine;
use engine::qt_engine::QtDataEngine;
use engine::permission_engine::PermissionEngine;
use engine::branding::BrandingEngine;
use engine::icon_3d_engine::Icon3DEngine;
use engine::theme_engine::ThemeEngine;
use std::sync::Arc;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    env_logger::init(); 

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Smart Brain Engine — Blank Native Container"),
        ..Default::default()
    };

    eframe::run_native(
        "Smart Brain Engine Container",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let wgpu_render_state = cc.wgpu_render_state.clone().expect("WGPU not enabled");
            Box::new(SmartBrainApp::new(&wgpu_render_state))
        }),
    )
}

struct SmartBrainApp {
    layout_engine: LayoutEngine,
    chart_engine: Arc<ChartEngine>,
    ultralight_engine: UltralightEngine,
    cursor_engine: CursorEngine,
    #[allow(dead_code)]
    hft_engine: HftEngine,
    permission_engine: PermissionEngine,
    #[allow(dead_code)]
    branding_engine: BrandingEngine,
    icon_3d_engine: Icon3DEngine,
    #[allow(dead_code)]
    theme_engine: ThemeEngine,
    event_router: EventRouter,
    #[allow(dead_code)]
    qt_engine: QtDataEngine,
    show_dev_panel: bool,
    show_3d_icons: bool,
    use_webgl_mode: bool,
    export_status_msg: String,
    project_dir: PathBuf,
}

impl SmartBrainApp {
    fn new(wgpu_render_state: &egui_wgpu::RenderState) -> Self {
        let ultralight_engine = UltralightEngine::new();
        let project_dir = PathBuf::from(r"C:\Users\satya\OneDrive\Pictures\satyam\dist");

        Self {
            layout_engine: LayoutEngine::new(),
            chart_engine: Arc::new(ChartEngine::new(wgpu_render_state)),
            ultralight_engine,
            cursor_engine: CursorEngine::new(true),
            hft_engine: HftEngine::new(),
            permission_engine: PermissionEngine::new(),
            branding_engine: BrandingEngine::new("Smart Brain Native Container", "Satyam Enterprise"),
            icon_3d_engine: Icon3DEngine::new(),
            theme_engine: ThemeEngine::new(),
            event_router: EventRouter::new(),
            qt_engine: QtDataEngine::new(),
            show_dev_panel: false,
            show_3d_icons: false,
            use_webgl_mode: false,
            export_status_msg: "Blank Engine Container: 100% Code-Driven Layout Active".to_string(),
            project_dir,
        }
    }
}

impl eframe::App for SmartBrainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();

        // Key Listeners for DevTools (Ctrl+D), Normal Refresh (F5/Ctrl+R), Hard Refresh (Ctrl+Shift+R)
        ctx.input(|i| {
            let is_ctrl = i.modifiers.ctrl;
            let is_shift = i.modifiers.shift;

            if i.key_pressed(egui::Key::F5) || (is_ctrl && i.key_pressed(egui::Key::R)) {
                if is_shift || i.key_pressed(egui::Key::F5) && is_ctrl {
                    self.ultralight_engine.hard_refresh();
                    self.export_status_msg = "⚡ HARD REFRESH: Cache Wiped & GPU VRAM Flushed!".to_string();
                } else {
                    self.ultralight_engine.normal_refresh();
                    self.export_status_msg = "🔄 Normal Refresh: DOM Surface Reloaded".to_string();
                }
            }

            if is_ctrl && i.key_pressed(egui::Key::D) {
                self.show_dev_panel = !self.show_dev_panel;
            }
        });

        self.ultralight_engine.render_html_surface();

        let is_mouse_down = ctx.input(|i| i.pointer.primary_down());
        if let Some(pointer_pos) = ctx.pointer_latest_pos() {
            self.cursor_engine.update_state(pointer_pos, is_mouse_down);
            self.event_router.route_mouse_event(pointer_pos, &self.layout_engine.current_layout.html_punch_rects);
        }
        
        // Zero-Hardcoding Adaptive Layout: Pure Code-Driven Space Allocation
        self.layout_engine.calculate_adaptive_tiling(
            &self.project_dir,
            screen_rect.size(),
            &self.ultralight_engine.active_dirty_rects,
        );
        let layout = self.layout_engine.current_layout.clone();

        if self.show_3d_icons {
            self.icon_3d_engine.render(ctx);
        }

        // --- LAYER 1: NATIVE WEBGPU CANDLESTICK CHART (LAYERING PUNCHING ZONE) ---
        let mut is_over_coordinate_graph = false;
        for wgpu_rect in layout.wgpu_rects.iter() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                if wgpu_rect.contains(pos) {
                    is_over_coordinate_graph = true;
                }
            }

            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(19, 23, 34)))
                .show(ctx, |_ui| {});

            egui::Window::new("WebGPU Native Chart Engine")
                .fixed_rect(*wgpu_rect)
                .title_bar(false)
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    let (rect, _response) = ui.allocate_exact_size(wgpu_rect.size(), egui::Sense::hover());
                    
                    let cb = egui_wgpu::Callback::new_paint_callback(
                        rect,
                        engine::chart_engine::ChartCallback {
                            engine: self.chart_engine.clone(),
                            punch_rects: layout.html_punch_rects.clone(),
                            force_webgl_mode: self.use_webgl_mode,
                        },
                    );
                    ui.painter().add(cb);
                });
        }

        // --- OPTIONAL DEVTOOLS OVERLAY (PRESS CTRL+D) ---
        if self.show_dev_panel {
            egui::Window::new("Smart Brain DevTools & Diagnostics")
                .fixed_pos([100.0, 80.0])
                .fixed_size([400.0, 350.0])
                .show(ctx, |ui| {
                    ui.heading("Blank Container Code Diagnostics:");
                    if let Some(ref plan) = self.layout_engine.current_plan {
                        ui.label(format!("• WebGPU Canvas Detected: {}", plan.has_webgpu_canvas));
                        ui.label(format!("• WebGL2 Canvas Detected: {}", plan.has_webgl_canvas));
                        ui.label(format!("• HTML DOM Surface Active: {}", plan.has_html_dom));
                        ui.label(format!("• Egui Native Overlay Active: {}", plan.has_egui_rust));
                    }
                    ui.separator();
                    ui.label(format!("Status: {}", self.export_status_msg));
                    ui.separator();
                    ui.checkbox(&mut self.show_3d_icons, "Show 3D Desktop Icons");
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("🔄 Normal Refresh (F5)").clicked() {
                            self.ultralight_engine.normal_refresh();
                        }
                        if ui.button("⚡ Hard Refresh (Ctrl+Shift+R)").clicked() {
                            self.ultralight_engine.hard_refresh();
                        }
                    });
                });
        }

        // --- RUNTIME END-USER PERMISSION PROMPT DIALOG MODAL ---
        if let Some(pending_perm) = self.permission_engine.pending_prompt {
            egui::Window::new("⚠️ System Permission Request")
                .fixed_pos([450.0, 250.0])
                .fixed_size([380.0, 180.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Hardware Access Requested!");
                    ui.separator();
                    ui.label("Application is requesting permission to access:");
                    ui.label(egui::RichText::new(pending_perm.display_name()).strong().color(egui::Color32::LIGHT_BLUE));
                    ui.label("Do you grant permission to this hardware subsystem?");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button(egui::RichText::new("  Allow  ").color(egui::Color32::GREEN)).clicked() {
                            self.permission_engine.respond_permission(pending_perm, true);
                            self.export_status_msg = format!("Permission GRANTED for {}", pending_perm.display_name());
                        }
                        if ui.button(egui::RichText::new("  Deny  ").color(egui::Color32::RED)).clicked() {
                            self.permission_engine.respond_permission(pending_perm, false);
                            self.export_status_msg = format!("Permission DENIED for {}", pending_perm.display_name());
                        }
                    });
                });
        }

        self.cursor_engine.evaluate_context(is_over_coordinate_graph, is_mouse_down);
        self.cursor_engine.render(ctx, screen_rect.size());
    }
}
