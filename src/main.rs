mod process_explorer_app;
mod columns;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Process Explorer"),
        ..Default::default()
    };
    eframe::run_native(
        "Process Explorer",
        options,
        Box::new(|cc| Box::new(process_explorer_app::ProcessExplorerApp::new(cc))),
    )
}