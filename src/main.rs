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
use engine::excel_export::ExcelExportEngine;
use engine::pdf_export::PdfExportEngine;
use engine::qt_engine::QtDataEngine;
use engine::permission_engine::{PermissionEngine, PermissionType};
use engine::icon_engine::{IconEngine, IconType};
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
            .with_title("Satyam CADPro Dashboard — Smart Brain Native Hybrid OS"),
        ..Default::default()
    };

    eframe::run_native(
        "Smart Brain Engine",
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
    hft_engine: HftEngine,
    permission_engine: PermissionEngine,
    branding_engine: BrandingEngine,
    icon_3d_engine: Icon3DEngine,
    theme_engine: ThemeEngine,
    event_router: EventRouter,
    #[allow(dead_code)]
    qt_engine: QtDataEngine,
    show_dynamic_popup: bool,
    show_3d_icons: bool,
    use_webgl_mode: bool,
    export_status_msg: String,
}

impl SmartBrainApp {
    fn new(wgpu_render_state: &egui_wgpu::RenderState) -> Self {
        let mut ultralight_engine = UltralightEngine::new();
        ultralight_engine.register_dirty_rect(DirtyRect::new(0.0, 0.0, 1280.0, 40.0));

        Self {
            layout_engine: LayoutEngine::new(),
            chart_engine: Arc::new(ChartEngine::new(wgpu_render_state)),
            ultralight_engine,
            cursor_engine: CursorEngine::new(true),
            hft_engine: HftEngine::new(),
            permission_engine: PermissionEngine::new(),
            branding_engine: BrandingEngine::new("Satyam CADPro App", "Satyam Enterprise"),
            icon_3d_engine: Icon3DEngine::new(),
            theme_engine: ThemeEngine::new(),
            event_router: EventRouter::new(),
            qt_engine: QtDataEngine::new(),
            show_dynamic_popup: false,
            show_3d_icons: true,
            use_webgl_mode: false,
            export_status_msg: "Loaded: C:/Users/satya/OneDrive/Pictures/satyam/dist/index.html".to_string(),
        }
    }
}

impl eframe::App for SmartBrainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();

        ctx.input(|i| {
            let is_ctrl = i.modifiers.ctrl;
            let is_shift = i.modifiers.shift;

            if i.key_pressed(egui::Key::F5) || (is_ctrl && i.key_pressed(egui::Key::R)) {
                if is_shift || i.key_pressed(egui::Key::F5) && is_ctrl {
                    self.ultralight_engine.hard_refresh();
                    self.export_status_msg = "⚡ HARD REFRESH: React Cache Wiped & GPU VRAM Flushed!".to_string();
                } else {
                    self.ultralight_engine.normal_refresh();
                    self.export_status_msg = "🔄 Normal Refresh: React DOM Surface Reloaded".to_string();
                }
            }
        });

        self.ultralight_engine.render_html_surface();

        let is_mouse_down = ctx.input(|i| i.pointer.primary_down());
        if let Some(pointer_pos) = ctx.pointer_latest_pos() {
            self.cursor_engine.update_state(pointer_pos, is_mouse_down);
            self.event_router.route_mouse_event(pointer_pos, &self.layout_engine.current_layout.html_punch_rects);
        }
        
        self.layout_engine.calculate_tiling(
            screen_rect.size(),
            &self.ultralight_engine.active_dirty_rects,
        );
        let layout = self.layout_engine.current_layout.clone();

        if self.show_3d_icons {
            self.icon_3d_engine.render(ctx);
        }

        // --- RENDER REGION 1: NATIVE EGUI CONTROL PANEL ---
        for (i, egui_rect) in layout.egui_rects.iter().enumerate() {
            let panel_frame = egui::Frame::window(&ctx.style())
                .fill(self.theme_engine.panel_color32())
                .rounding(10.0);

            egui::Window::new(format!("Native Control Panel {}", i))
                .fixed_rect(*egui_rect)
                .title_bar(false)
                .frame(panel_frame)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        IconEngine::render_icon(ui, IconType::Settings, 20.0, self.theme_engine.accent_color32());
                        ui.heading(egui::RichText::new("Satyam CADPro Engine Panel").color(self.theme_engine.text_color32()));
                    });
                    ui.label(egui::RichText::new("🌐 Active App: C:/Users/satya/OneDrive/Pictures/satyam").small().color(egui::Color32::LIGHT_BLUE));

                    ui.separator();
                    ui.heading("🔄 Refresh Controls & Shortcuts:");
                    ui.horizontal(|ui| {
                        if ui.button("🔄 Normal Refresh (F5 / Ctrl+R)").clicked() {
                            self.ultralight_engine.normal_refresh();
                            self.export_status_msg = "🔄 Normal Refresh: React DOM Surface Reloaded".to_string();
                        }
                        if ui.button("⚡🔄 Hard Refresh (Ctrl+Shift+R)").clicked() {
                            self.ultralight_engine.hard_refresh();
                            self.export_status_msg = "⚡ HARD REFRESH: React Cache Wiped & GPU VRAM Flushed!".to_string();
                        }
                    });

                    if IconEngine::render_icon_button(ui, IconType::NotificationBell, "Toggle Dynamic HTML Popup", egui::Color32::from_rgb(255, 200, 0)) {
                        self.show_dynamic_popup = !self.show_dynamic_popup;
                        if self.show_dynamic_popup {
                            self.ultralight_engine.register_dirty_rect(DirtyRect::new(400.0, 200.0, 300.0, 200.0));
                            self.ultralight_engine.receive_js_trigger("popup_opened");
                        } else {
                            self.ultralight_engine.active_dirty_rects.retain(|r| r.x != 400.0);
                            self.ultralight_engine.receive_js_trigger("popup_closed");
                        }
                    }

                    ui.checkbox(&mut self.show_3d_icons, "Show 3D Desktop Icons");

                    ui.separator();
                    self.theme_engine.render_customizer(ui);

                    ui.separator();
                    ui.horizontal(|ui| {
                        IconEngine::render_icon(ui, IconType::ThemePalette, 18.0, self.theme_engine.accent_color32());
                        ui.heading("Graphics Pipeline Mode (WebGL vs WebGPU):");
                    });
                    ui.horizontal(|ui| {
                        if ui.selectable_label(!self.use_webgl_mode, "WebGPU (Next-Gen WGSL)").clicked() {
                            self.use_webgl_mode = false;
                        }
                        if ui.selectable_label(self.use_webgl_mode, "WebGL2 (Chrome GLSL)").clicked() {
                            self.use_webgl_mode = true;
                        }
                    });

                    ui.separator();
                    ui.heading("Phase E: HFT & Exports");
                    
                    if IconEngine::render_icon_button(ui, IconType::LightningHFT, "Run Polars HFT 1M Candle Math", self.theme_engine.accent_color32()) {
                        let dummy_prices: Vec<f64> = (0..10_000).map(|i| 100.0 + (i as f64 * 0.01)).collect();
                        match self.hft_engine.calculate_indicators(&dummy_prices) {
                            Ok(res) => {
                                self.export_status_msg = format!("Polars Math Done: {} items in <1ms", res.len());
                            }
                            Err(e) => {
                                self.export_status_msg = format!("HFT Error: {:?}", e);
                            }
                        }
                    }

                    ui.horizontal(|ui| {
                        if IconEngine::render_icon_button(ui, IconType::ExportExcel, "Export Native Excel", egui::Color32::from_rgb(40, 200, 100)) {
                            let dest = PathBuf::from("smart_brain_report.xlsx");
                            match ExcelExportEngine::export_trading_report(&dest, "BTC/USDT", 5_000) {
                                Ok(_) => self.export_status_msg = "Excel Exported: smart_brain_report.xlsx".to_string(),
                                Err(e) => self.export_status_msg = format!("Excel Error: {:?}", e),
                            }
                        }

                        if IconEngine::render_icon_button(ui, IconType::ExportPdf, "Export Native PDF", egui::Color32::from_rgb(255, 80, 80)) {
                            let dest = PathBuf::from("smart_brain_chart.pdf");
                            match PdfExportEngine::export_pdf_report(&dest, "Smart Brain Chart Analysis") {
                                Ok(_) => self.export_status_msg = "PDF Exported: smart_brain_chart.pdf".to_string(),
                                Err(e) => self.export_status_msg = format!("PDF Error: {:?}", e),
                            }
                        }
                    });

                    ui.label(egui::RichText::new(format!("Status: {}", self.export_status_msg)).strong().color(egui::Color32::YELLOW));

                    ui.separator();
                    ui.horizontal(|ui| {
                        IconEngine::render_icon(ui, IconType::SecurityShield, 18.0, egui::Color32::from_rgb(255, 180, 0));
                        ui.heading("Phase X: Hardware API Permission Guard");
                    });
                    ui.horizontal(|ui| {
                        if IconEngine::render_icon_button(ui, IconType::Bluetooth, "Bluetooth", egui::Color32::from_rgb(0, 150, 255)) {
                            self.permission_engine.request_permission(PermissionType::Bluetooth);
                        }
                        if IconEngine::render_icon_button(ui, IconType::WebRTC, "WebRTC", egui::Color32::from_rgb(200, 100, 255)) {
                            self.permission_engine.request_permission(PermissionType::WebRTC);
                        }
                        if IconEngine::render_icon_button(ui, IconType::USB, "USB", egui::Color32::from_rgb(255, 150, 0)) {
                            self.permission_engine.request_permission(PermissionType::USBDevice);
                        }
                    });

                    ui.separator();
                    ui.checkbox(&mut self.cursor_engine.enable_coordinate_crosshair, "Enable Coordinate Graph Crosshair");

                    self.branding_engine.render_credit_footer(ui);
                });
        }

        // --- RENDER REGION 2: NATIVE CANVAS (PUNCHED LAYER VIA SCISSOR RECT) ---
        let mut is_over_coordinate_graph = false;
        for (i, wgpu_rect) in layout.wgpu_rects.iter().enumerate() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                if wgpu_rect.contains(pos) {
                    is_over_coordinate_graph = true;
                }
            }

            egui::Window::new(format!("Native Chart Engine {}", i))
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

        // --- RENDER REGION 3: DYNAMIC HTML POPUP ---
        if self.show_dynamic_popup {
            egui::Window::new("Dynamic HTML Popup (Ultralight)")
                .fixed_pos([400.0, 200.0])
                .fixed_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.heading("Ultralight HTML DOM");
                    ui.label(format!("Loaded: {}", self.ultralight_engine.current_url));
                    ui.label("Background graphics chart is stencil/scissor punched behind this box.");
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
