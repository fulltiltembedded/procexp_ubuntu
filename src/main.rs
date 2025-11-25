use sysinfo::{System};

mod gui;

fn show_process_stub(processes: &std::collections::HashMap<sysinfo::Pid, sysinfo::Process>)
{
    for (pid, process) in processes { 
        if process.cpu_usage() > 0.0 {
            println!("--------------------------------------");
            println!("Process: {}", process.name().to_string_lossy());
            println!("PID: {}", pid);
            println!("CPU usage {}", process.cpu_usage());
            println!("Memory: {}", process.memory());
            println!("Virtual memory: {}", process.virtual_memory());
            println!("Disk usage: {}", process.disk_usage().total_written_bytes);
            println!("Start time: {}", process.start_time());   
        }
    }
}

fn main() {
    // Create new sysinfo::system.
    let mut system = System::new_all();
    system.refresh_all();
    system.refresh_cpu_all();

    // Show process info.
    show_process_stub(system.processes());

    // Show GUI stub.
    gui::show_gui_stub();
}
