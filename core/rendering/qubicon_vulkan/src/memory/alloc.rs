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
    Download
}