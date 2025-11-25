use sysinfo::{System};

fn get_process_list()
{
    let mut system = System::new_all();
    system.refresh_all();
    system.refresh_cpu_all();

    // Iterate through the list.
    let processes = system.processes();
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
    get_process_list();
}
