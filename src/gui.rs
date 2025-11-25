use eframe::egui;
use sysinfo::{System};
use crate::visible_columns::VisibleColumns;

pub struct ProcessExplorerApp {
    system: System,
    visible_columns: VisibleColumns,
}


impl ProcessExplorerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        system.refresh_cpu_all();
        Self {
            system,
            visible_columns: VisibleColumns::default(),
        }
    }

    fn format_bytes(&self, bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;
        
        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }
        
        if unit_idx == 0 {
            format!("{} {}", bytes, UNITS[unit_idx])
        } else {
            format!("{:.2} {}", size, UNITS[unit_idx])
        }
    }

}

impl eframe::App for ProcessExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { 
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Refresh").clicked() {
                        self.system.refresh_all();
                    }
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.label("Extra Columns:");
                    ui.separator();
                    ui.checkbox(&mut self.visible_columns.virtual_memory, "Virtual Memory");
                    ui.checkbox(&mut self.visible_columns.parent_pid, "Parent PID");
                    ui.checkbox(&mut self.visible_columns.start_time, "Start Time");
                    ui.checkbox(&mut self.visible_columns.executable_path, "Executable Path");
                    ui.checkbox(&mut self.visible_columns.working_directory, "Working Directory");
                    ui.separator();
                    if ui.button("System Information...").clicked() {}
                });
                ui.menu_button("Process", |ui| { });
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let cpu_usage = self.system.global_cpu_usage();
                let total_memory = self.system.total_memory();
                let used_memory = self.system.used_memory();
                let total_swap = self.system.total_swap();
                let used_swap = self.system.used_swap();
                let process_count = self.system.processes().len();
                let memory_percent = if total_memory > 0 {
                    (used_memory as f64 / total_memory as f64) * 100.0
                } else {
                    0.0
                };

                // CPU mini graph
                ui.label(format!("CPU: {:.2}%", cpu_usage));
                ui.separator();
                ui.label(format!("Memory: {:.2}% ({})", 
                    memory_percent,
                    self.format_bytes(used_memory)));
                ui.separator();
                if total_swap > 0 {
                    ui.label(format!("Swap: {:.2}% ({})",
                        (used_swap as f64 / total_swap as f64) * 100.0,
                        self.format_bytes(used_swap)));
                    ui.separator();
                }
                ui.label(format!("Processes: {}", process_count));
            });
        });
    }
}

