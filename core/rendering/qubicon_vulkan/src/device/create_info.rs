pub struct DeviceCreateInfo {
    pub queue_families: Vec<QueueFamilyUsage>,
    pub extensions
}


pub struct QueueFamilyUsage {
    pub family_idx: u32,
    pub queues: Vec<f32>
}