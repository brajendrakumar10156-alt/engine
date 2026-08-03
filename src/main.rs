#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod engine;

use eframe::egui;
use eframe::egui_wgpu;
use ui::layout_engine::LayoutEngine;
use ui::event_router::EventRouter;
use engine::chart_engine::ChartEngine;
use engine::ultralight_engine::{UltralightEngine, DirtyRect};
use engine::qt_engine::QtDataEngine;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    env_logger::init(); 

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Smart Brain Engine — Dual WebGPU (Egui + Ultralight) Hybrid OS"),
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
    event_router: EventRouter,
    #[allow(dead_code)]
    qt_engine: QtDataEngine,
    show_dynamic_popup: bool,
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
            event_router: EventRouter::new(),
            qt_engine: QtDataEngine::new(),
            show_dynamic_popup: false,
        }
    }
}

impl eframe::App for SmartBrainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();

        // Render Ultralight surface & clear expired dirty rects
        self.ultralight_engine.render_html_surface();

        // Route mouse events using EventRouter
        if let Some(pointer_pos) = ctx.pointer_latest_pos() {
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
                    ui.label("Dual WebGPU Support:");
                    ui.label("✔ WebGPU inside Egui (Rust)");
                    ui.label("✔ WebGPU inside Ultralight Punch");
                    ui.label("✔ Scissor GPU Punching Active");
                });
        }

        // --- RENDER REGION 2: WEBGPU NATIVE CANVAS (PUNCHED LAYER VIA SCISSOR RECT) ---
        for (i, wgpu_rect) in layout.wgpu_rects.iter().enumerate() {
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
                    ui.label("Background WebGPU chart is stencil/scissor punched behind this box.");
                });
        }
    }
}
