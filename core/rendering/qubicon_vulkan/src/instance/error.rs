#[derive(thiserror::Error, Debug)]
pub enum InstanceError {
    #[error("vulkan library loading error")]
    LoadError(#[from] ash::LoadingError),
    #[error("instance creation error")]
    CreationError(#[from] ash::vk::Result)
}