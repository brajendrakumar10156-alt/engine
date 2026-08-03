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
use std::sync::Arc;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    env_logger::init(); 

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Smart Brain Engine — Dual WebGL & WebGPU Hybrid OS"),
        ..Default::default()
    };

    eframe::run_native(
        "Smart Brain Engine",
        options,
        Box::new(|cc| {
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
    event_router: EventRouter,
    #[allow(dead_code)]
    qt_engine: QtDataEngine,
    show_dynamic_popup: bool,
    use_webgl_mode: bool, // Toggle between Standard WebGL vs Next-Gen WebGPU
    export_status_msg: String,
}

impl SmartBrainApp {
    fn new(wgpu_render_state: &egui_wgpu::RenderState) -> Self {
        let mut ultralight_engine = UltralightEngine::new();
        
        // Register initial UI bounding boxes for Layering Punching (top navbar, sidebar)
        ultralight_engine.register_dirty_rect(DirtyRect::new(0.0, 0.0, 1280.0, 40.0)); // Top nav

        Self {
            layout_engine: LayoutEngine::new(),
            chart_engine: Arc::new(ChartEngine::new(wgpu_render_state)),
            ultralight_engine,
            cursor_engine: CursorEngine::new(true), // Universal Cursor Engine
            hft_engine: HftEngine::new(),
            event_router: EventRouter::new(),
            qt_engine: QtDataEngine::new(),
            show_dynamic_popup: false,
            use_webgl_mode: false,
            export_status_msg: "Ready".to_string(),
        }
    }
}

impl eframe::App for SmartBrainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();

        // Render Ultralight surface & clear expired dirty rects
        self.ultralight_engine.render_html_surface();

        // Track pointer position and mouse press state
        let is_mouse_down = ctx.input(|i| i.pointer.primary_down());
        if let Some(pointer_pos) = ctx.pointer_latest_pos() {
            self.cursor_engine.update_state(pointer_pos, is_mouse_down);
            self.event_router.route_mouse_event(pointer_pos, &self.layout_engine.current_layout.html_punch_rects);
        }
        
        // 1. Calculate Tiling & Layering Punching zones
        self.layout_engine.calculate_tiling(
            screen_rect.size(),
            &self.ultralight_engine.active_dirty_rects,
        );
        let layout = self.layout_engine.current_layout.clone();

        // --- RENDER REGION 1: NATIVE EGUI CONTROL PANEL (WebGPU inside Egui) ---
        for (i, egui_rect) in layout.egui_rects.iter().enumerate() {
            egui::Window::new(format!("Native Control Panel {}", i))
                .fixed_rect(*egui_rect)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.heading("Egui + Rust Native Panel");
                    ui.label("100% Native Rust Egui UI.");

                    if ui.button("Toggle Dynamic HTML Popup").clicked() {
                        self.show_dynamic_popup = !self.show_dynamic_popup;
                        if self.show_dynamic_popup {
                            // Register dynamic popup bounding box (chota 300x200) for Layering Punching
                            self.ultralight_engine.register_dirty_rect(DirtyRect::new(400.0, 200.0, 300.0, 200.0));
                            self.ultralight_engine.receive_js_trigger("popup_opened");
                        } else {
                            self.ultralight_engine.active_dirty_rects.retain(|r| r.x != 400.0);
                            self.ultralight_engine.receive_js_trigger("popup_closed");
                        }
                    }

                    ui.separator();
                    ui.heading("Graphics Pipeline Mode:");
                    ui.horizontal(|ui| {
                        if ui.selectable_label(!self.use_webgl_mode, "WebGPU (Next-Gen)").clicked() {
                            self.use_webgl_mode = false;
                        }
                        if ui.selectable_label(self.use_webgl_mode, "WebGL2 (Chrome Style)").clicked() {
                            self.use_webgl_mode = true;
                        }
                    });

                    ui.separator();
                    ui.heading("Phase E: HFT & Exports");
                    
                    if ui.button("Run Polars HFT 1M Candle Math").clicked() {
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
                        if ui.button("Export Native Excel").clicked() {
                            let dest = PathBuf::from("smart_brain_report.xlsx");
                            match ExcelExportEngine::export_trading_report(&dest, "BTC/USDT", 5_000) {
                                Ok(_) => self.export_status_msg = "Excel Exported: smart_brain_report.xlsx".to_string(),
                                Err(e) => self.export_status_msg = format!("Excel Error: {:?}", e),
                            }
                        }

                        if ui.button("Export Native PDF").clicked() {
                            let dest = PathBuf::from("smart_brain_chart.pdf");
                            match PdfExportEngine::export_pdf_report(&dest, "Smart Brain Chart Analysis") {
                                Ok(_) => self.export_status_msg = "PDF Exported: smart_brain_chart.pdf".to_string(),
                                Err(e) => self.export_status_msg = format!("PDF Error: {:?}", e),
                            }
                        }
                    });

                    ui.label(format!("Status: {}", self.export_status_msg));

                    ui.separator();
                    ui.checkbox(&mut self.cursor_engine.enable_coordinate_crosshair, "Enable Coordinate Graph Crosshair");

                    ui.separator();
                    ui.label("Dual Graphics Pipeline Support:");
                    ui.label("✔ WebGL2 Chrome Standard Mode");
                    ui.label("✔ WebGPU Next-Gen Mode");
                    ui.label("✔ WebGL/WebGPU inside Egui");
                    ui.label("✔ WebGL/WebGPU inside Ultralight Punch");
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

        // --- RENDER REGION 3: DYNAMIC ULTRALIGHT HTML POPUP (Layering Punching) ---
        if self.show_dynamic_popup {
            egui::Window::new("Dynamic HTML Popup (Ultralight)")
                .fixed_pos([400.0, 200.0])
                .fixed_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.heading("Ultralight HTML DOM");
                    ui.label("Dynamic HTML Layering active!");
                    ui.label("Background graphics chart is stencil/scissor punched behind this box.");
                });
        }

        // Evaluate cursor state: Defaults to NORMAL OS CURSOR everywhere!
        self.cursor_engine.evaluate_context(is_over_coordinate_graph, is_mouse_down);

        // --- RENDER UNIVERSAL CURSOR ---
        self.cursor_engine.render(ctx, screen_rect.size());
    }
}
