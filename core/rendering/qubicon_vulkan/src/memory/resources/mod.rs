pub enum SharingMode<'a> {
    Exclusive,
    Concurent { queue_families: &'a [u32] }
}

pub mod image;
pub mod buffer;