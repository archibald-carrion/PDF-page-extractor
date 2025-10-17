use eframe::egui;
use lopdf::Document;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "PDF Page Extractor",
        options,
        Box::new(|_cc| Ok(Box::new(PdfExtractorApp::default()))),
    )
}

#[derive(Default)]
struct PdfExtractorApp {
    input_path: String,
    output_path: String,
    page_range: String,
    status_message: String,
}

impl eframe::App for PdfExtractorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PDF Page Extractor");
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Input PDF:");
                ui.text_edit_singleline(&mut self.input_path);
                if ui.button("Browse").clicked() {
                    match rfd::FileDialog::new()
                        .add_filter("PDF", &["pdf"])
                        .pick_file()
                    {
                        Some(path) => {
                            self.input_path = path.display().to_string();
                            self.status_message.clear();
                        }
                        None => {
                            self.status_message = "File selection cancelled or unavailable. You can type the path manually.".to_string();
                        }
                    }
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Output PDF:");
                ui.text_edit_singleline(&mut self.output_path);
                if ui.button("Browse").clicked() {
                    match rfd::FileDialog::new()
                        .add_filter("PDF", &["pdf"])
                        .set_file_name("output.pdf")
                        .save_file()
                    {
                        Some(path) => {
                            self.output_path = path.display().to_string();
                            self.status_message.clear();
                        }
                        None => {
                            self.status_message = "File selection cancelled or unavailable. You can type the path manually.".to_string();
                        }
                    }
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Pages (e.g., 1-3,5,7-9):");
                ui.text_edit_singleline(&mut self.page_range);
            });

            ui.add_space(20.0);

            if ui.button("Extract Pages").clicked() {
                // Validate inputs first
                if self.input_path.is_empty() {
                    self.status_message = "✗ Error: Please specify an input PDF file".to_string();
                } else if self.output_path.is_empty() {
                    self.status_message = "✗ Error: Please specify an output PDF file".to_string();
                } else if self.page_range.is_empty() {
                    self.status_message = "✗ Error: Please specify page range".to_string();
                } else if !std::path::Path::new(&self.input_path).exists() {
                    self.status_message = format!("✗ Error: Input file not found: {}", self.input_path);
                } else {
                    self.status_message = match extract_pages(
                        &self.input_path,
                        &self.output_path,
                        &self.page_range,
                    ) {
                        Ok(_) => "✓ Pages extracted successfully!".to_string(),
                        Err(e) => format!("✗ Error: {}", e),
                    };
                }
            }

            ui.add_space(20.0);

            if !self.status_message.is_empty() {
                ui.label(&self.status_message);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.label("Examples: '1-5' extracts pages 1 to 5");
            ui.label("'1,3,5' extracts pages 1, 3, and 5");
            ui.label("'1-3,7-9' extracts pages 1-3 and 7-9");
        });
    }
}

fn extract_pages(input: &str, output: &str, range: &str) -> Result<(), String> {
    let mut doc = Document::load(input).map_err(|e| format!("Failed to load PDF: {}", e))?;
    let pages_to_extract = parse_page_range(range)?;
    
    let page_map = doc.get_pages();
    let total_pages = page_map.len();
    
    for page in &pages_to_extract {
        if *page < 1 || *page > total_pages {
            return Err(format!("Page {} is out of range (1-{})", page, total_pages));
        }
    }
    
    // Get all page numbers
    let all_pages: Vec<u32> = page_map.keys().copied().collect();
    
    // Convert page numbers to keep (1-indexed input to actual page numbers in PDF)
    let mut sorted_page_nums: Vec<u32> = all_pages.clone();
    sorted_page_nums.sort();
    
    let pages_to_keep: Vec<u32> = pages_to_extract
        .iter()
        .filter_map(|&page_idx| sorted_page_nums.get(page_idx - 1).copied())
        .collect();
    
    // Calculate which pages to delete (all pages except the ones we want)
    let pages_to_delete: Vec<u32> = all_pages
        .into_iter()
        .filter(|p| !pages_to_keep.contains(p))
        .collect();
    
    // Delete unwanted pages
    doc.delete_pages(&pages_to_delete);
    doc.compress();
    
    doc.save(output).map_err(|e| format!("Failed to save PDF: {}", e))?;
    Ok(())
}

fn parse_page_range(range: &str) -> Result<Vec<usize>, String> {
    let mut pages = Vec::new();
    
    for part in range.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() != 2 {
                return Err(format!("Invalid range: {}", part));
            }
            let start: usize = bounds[0].trim().parse()
                .map_err(|_| format!("Invalid number: {}", bounds[0]))?;
            let end: usize = bounds[1].trim().parse()
                .map_err(|_| format!("Invalid number: {}", bounds[1]))?;
            
            if start > end {
                return Err(format!("Invalid range: {} > {}", start, end));
            }
            
            for i in start..=end {
                pages.push(i);
            }
        } else {
            let page: usize = part.parse()
                .map_err(|_| format!("Invalid page number: {}", part))?;
            pages.push(page);
        }
    }
    
    Ok(pages)
}