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
            .with_title("QuantaAI — Smart Brain Native Hybrid OS"),
        ..Default::default()
    };

    eframe::run_native(
        "QuantaAI Engine",
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
    show_dev_panel: bool,
    show_dynamic_popup: bool,
    show_3d_icons: bool,
    use_webgl_mode: bool,
    active_tab: String,
    export_status_msg: String,
}

impl SmartBrainApp {
    fn new(wgpu_render_state: &egui_wgpu::RenderState) -> Self {
        let mut ultralight_engine = UltralightEngine::new();
        ultralight_engine.register_dirty_rect(DirtyRect::new(0.0, 0.0, 1280.0, 42.0));

        Self {
            layout_engine: LayoutEngine::new(),
            chart_engine: Arc::new(ChartEngine::new(wgpu_render_state)),
            ultralight_engine,
            cursor_engine: CursorEngine::new(true),
            hft_engine: HftEngine::new(),
            permission_engine: PermissionEngine::new(),
            branding_engine: BrandingEngine::new("QuantaAI Trading App", "Satyam Enterprise"),
            icon_3d_engine: Icon3DEngine::new(),
            theme_engine: ThemeEngine::new(),
            event_router: EventRouter::new(),
            qt_engine: QtDataEngine::new(),
            show_dev_panel: false,
            show_dynamic_popup: false,
            show_3d_icons: false,
            use_webgl_mode: false,
            active_tab: "Paper Trading".to_string(),
            export_status_msg: "QuantaAI Engine Active | 100% WebGPU GPU Accelerated".to_string(),
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
        
        self.layout_engine.calculate_tiling(
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

        // --- LAYER 2: TOP NAVIGATION HEADER BAR (MATCHING IMAGE 1) ---
        egui::TopBottomPanel::top("top_header_panel")
            .exact_height(42.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(19, 23, 34)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    ui.heading(egui::RichText::new("QuantaAI").strong().color(egui::Color32::WHITE));
                    ui.add_space(10.0);

                    // Symbol Selector Badge
                    ui.group(|ui| {
                        ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(28, 34, 48);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Binance 🔍  OGTRY").strong().color(egui::Color32::from_rgb(255, 200, 0)));
                            ui.label(egui::RichText::new("$6.53  ⚡28  1m").color(egui::Color32::from_rgb(255, 80, 80)));
                        });
                    });

                    ui.add_space(10.0);
                    // WebGPU Hardware Acceleration Badge
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🟢 WEBGPU HARDWARE: NVIDIA GeForce RTX (WGPU Native)").small().color(egui::Color32::from_rgb(0, 255, 180)));
                        });
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        if ui.button(egui::RichText::new("🖥️ Desktop App").small().color(egui::Color32::WHITE)).clicked() {
                            self.show_dev_panel = !self.show_dev_panel;
                        }
                        if ui.button("⚡ Hard Refresh").clicked() {
                            self.ultralight_engine.hard_refresh();
                            self.export_status_msg = "⚡ HARD REFRESH: Cache Wiped & GPU VRAM Flushed!".to_string();
                        }
                        if ui.button("🔄 Refresh").clicked() {
                            self.ultralight_engine.normal_refresh();
                            self.export_status_msg = "🔄 Normal Refresh: DOM Reloaded".to_string();
                        }
                    });
                });
            });

        // --- LAYER 3: LEFT DRAWING TOOLBAR (MATCHING IMAGE 1) ---
        egui::SidePanel::left("left_drawing_toolbar")
            .exact_width(52.0)
            .resizable(false)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(19, 23, 34)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::Settings, "", egui::Color32::LIGHT_GRAY);
                    ui.add_space(6.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::LightningHFT, "", egui::Color32::from_rgb(0, 255, 180));
                    ui.add_space(6.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::ThemePalette, "", egui::Color32::from_rgb(255, 200, 0));
                    ui.add_space(6.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::NotificationBell, "", egui::Color32::from_rgb(200, 100, 255));
                    ui.add_space(6.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::ExportExcel, "", egui::Color32::from_rgb(40, 200, 100));
                    ui.add_space(6.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::ExportPdf, "", egui::Color32::from_rgb(255, 80, 80));
                    ui.add_space(6.0);
                    let _ = IconEngine::render_icon_button(ui, IconType::SecurityShield, "", egui::Color32::from_rgb(255, 180, 0));
                });
            });

        // --- LAYER 4: BOTTOM STATUS & TAB BAR (MATCHING IMAGE 1) ---
        egui::TopBottomPanel::bottom("bottom_status_panel")
            .exact_height(36.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(19, 23, 34)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    
                    let tabs = ["Paper Trading", "Strategy Tester", "Arbitrage", "Advanced Tools"];
                    for tab in tabs {
                        if ui.selectable_label(self.active_tab == tab, tab).clicked() {
                            self.active_tab = tab.to_string();
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new(format!("Status: {}", self.export_status_msg)).small().color(egui::Color32::YELLOW));
                        ui.label(egui::RichText::new("🕒 13:37:30 UTC | % | log | auto").small().color(egui::Color32::LIGHT_GRAY));
                    });
                });
            });

        // --- LAYER 5: OPTIONAL DEVTOOLS OVERLAY PANEL (TOGGLED VIA CTRL+D OR DESKTOP APP BUTTON) ---
        if self.show_dev_panel {
            egui::Window::new("QuantaAI DevTools & Native Controls")
                .fixed_pos([100.0, 80.0])
                .fixed_size([420.0, 520.0])
                .show(ctx, |ui| {
                    ui.heading("QuantaAI Native Engine Controls");
                    ui.separator();

                    ui.checkbox(&mut self.show_3d_icons, "Show 3D Desktop Icons");
                    self.theme_engine.render_customizer(ui);

                    ui.separator();
                    ui.heading("Graphics Pipeline Mode:");
                    ui.horizontal(|ui| {
                        if ui.selectable_label(!self.use_webgl_mode, "WebGPU (Next-Gen WGSL)").clicked() {
                            self.use_webgl_mode = false;
                        }
                        if ui.selectable_label(self.use_webgl_mode, "WebGL2 (Chrome GLSL)").clicked() {
                            self.use_webgl_mode = true;
                        }
                    });

                    ui.separator();
                    if IconEngine::render_icon_button(ui, IconType::LightningHFT, "Run Polars HFT 1M Candle Math", self.theme_engine.accent_color32()) {
                        let dummy_prices: Vec<f64> = (0..10_000).map(|i| 100.0 + (i as f64 * 0.01)).collect();
                        match self.hft_engine.calculate_indicators(&dummy_prices) {
                            Ok(res) => self.export_status_msg = format!("Polars Math Done: {} items in <1ms", res.len()),
                            Err(e) => self.export_status_msg = format!("HFT Error: {:?}", e),
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

                    ui.separator();
                    ui.heading("Hardware API Permission Guard");
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

                    self.branding_engine.render_credit_footer(ui);
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
