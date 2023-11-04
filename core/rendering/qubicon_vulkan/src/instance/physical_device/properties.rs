use arrayvec::ArrayString;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Cpu = 4,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Other = 0
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceProperties {
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: DeviceType,
    pub device_name: ArrayString<256>,
    pub pipeline_chache_uuid: [u8; ash::vk::UUID_SIZE],

    // TODO: limits
    // TODO: sparse properties
}

/// For internal use only
impl From<ash::vk::PhysicalDeviceProperties> for DeviceProperties {
    fn from(value: ash::vk::PhysicalDeviceProperties) -> Self {
        Self {
            driver_version: value.driver_version,
            vendor_id: value.vendor_id,
            device_id: value.device_id,
            device_type: value.device_type.into(),
            device_name: ArrayString::from_byte_string(unsafe { core::mem::transmute(&value.device_name) }).unwrap(),
            pipeline_chache_uuid: value.pipeline_cache_uuid
        }
    }
}

/// For internal use only
impl From<ash::vk::PhysicalDeviceType> for DeviceType {
    fn from(value: ash::vk::PhysicalDeviceType) -> Self {
        unsafe {core::mem::transmute(value.as_raw() as u8)}
    }
}