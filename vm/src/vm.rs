pub struct VirtualMachine {
    // Maximum memory in MB that the VM will allocate
    // If loaded classpath is bigger than this, the VM will fail to load
    pub max_memory_size: i32,
}

impl VirtualMachine {
    pub fn new(max_memory_size: Option<i32>) -> Self {
        VirtualMachine {
            max_memory_size: max_memory_size.unwrap_or(4096), // Default to 4GB
        }
    }
}