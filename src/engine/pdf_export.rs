use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Native HD Vector PDF Exporter Engine
/// Exports crisp, non-pixelated vector PDFs of charts and reports directly to disk.
pub struct PdfExportEngine;

impl PdfExportEngine {
    /// Generates an HD Vector PDF report directly to disk
    pub fn export_pdf_report(path: &Path, title: &str) -> Result<(), String> {
        let (doc, page1, layer1) = PdfDocument::new(title, Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Load Built-in Helvetica Font
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| format!("PDF Font Error: {:?}", e))?;

        // Write Header Text
        current_layer.use_text(title, 24.0, Mm(20.0), Mm(270.0), &font);
        current_layer.use_text("Smart Brain Engine — Native Vector Report", 14.0, Mm(20.0), Mm(255.0), &font);

        // Save PDF directly to disk
        let file = File::create(path).map_err(|e| format!("File Create Error: {:?}", e))?;
        doc.save(&mut BufWriter::new(file)).map_err(|e| format!("PDF Save Error: {:?}", e))?;

        log::info!("Native Vector PDF saved successfully to {:?}", path);
        Ok(())
    }
}
