use super::MemoryTypeProperties;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryKind {
    /// Memory is local to GPU and cant be accessed from CPU.
    /// GPU can access this memory way faster than any other.
    /// Preffered format to textures, models and all that stuff
    Local,
    /// Memory can be accessed both from CPU and GPU.
    /// Effective for transfering data to GPU
    Upload,
    /// Memory also can be accessed both from CPU and GPU.
    /// Effective for reading data from GPU
    Download,
    
    /// Memory for some special needs. Will be rarely used.
    Custom {
        /// Properties what memory type should have
        allowed_properties: MemoryTypeProperties,
        /// Properties what memory type shouldnt have
        denied_properties: MemoryTypeProperties
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationLifetime {
    /// Usable for staging buffers, temporal images and etc.
    /// Basicaly for resources what needed for a short period.
    Short,
    /// Usable for textures and models, because they are
    /// required for a pretty long period.
    Long
}