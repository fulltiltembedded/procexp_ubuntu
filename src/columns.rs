#[derive(Clone)]
pub struct VisibleColumns {
    pub virtual_memory: bool,
    pub parent_pid: bool,
    pub start_time: bool,
    pub executable_path: bool,
    pub working_directory: bool,
}

impl Default for VisibleColumns {
    fn default() -> Self {
        Self {
            virtual_memory: false,
            parent_pid: false,
            start_time: false,
            executable_path: false,
            working_directory: false,
        }
    }
}
