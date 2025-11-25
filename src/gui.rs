use eframe::egui;

struct ProcessExplorerApp {
    
}

impl ProcessExplorerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for ProcessExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { }
}

pub fn show_gui_stub() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Process Explorer"),
        ..Default::default()
    };
    eframe::run_native(
        "Process Explorer",
        options,
        Box::new(|cc| Box::new(ProcessExplorerApp::new(cc))),
    )
}