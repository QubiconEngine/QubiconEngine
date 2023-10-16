#[derive(Default, Debug, Clone, Copy)]
pub struct InstanceCreateInfo {
    #[cfg(feature = "windowing")]
    pub enable_windowing: bool,
}