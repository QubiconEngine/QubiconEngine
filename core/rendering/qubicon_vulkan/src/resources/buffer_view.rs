use std::{ sync::Arc, ops::Range };

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct BufferViewCreateInfo {
//     format: Format,
//     range: Range<u64>
// }
// impl Default for BufferViewCreateInfo {
//     #[inline]
//     fn default() -> Self {
//         Self {
//             format: Default::default(),
//             range: 0..1
//         }
//     }
// }

// pub struct BufferView<A: DeviceMemoryAllocator> {
//     buffer: Arc<BufferInner<A>>,
//     buffer_view: VkBufferView,

//     create_info: BufferViewCreateInfo
// }

// impl<A: DeviceMemoryAllocator> BufferView<A> {
//     /// # Safety
//     /// * Range should be in bounds of buffer, be multiple of format size and countain at least one element
//     /// * Buffer should have one of Texel usage flags
//     pub(crate) unsafe fn create_unchecked(
//         buffer: &Buffer<A>,
//         create_info: &BufferViewCreateInfo
//     ) -> Result<Arc<Self>, Error> {
//         let buffer = Arc::clone(buffer.as_inner());
//         let create_info = create_info.clone();

//         let buffer_view = buffer.device.create_buffer_view(
//             &VkBufferViewCreateInfo {
//                 buffer: buffer.buffer,
//                 format: create_info.format.into(),
//                 offset: create_info.range.start,
//                 range: create_info.range.clone().count() as u64,

//                 ..Default::default()
//             },
//             None
//         ).map_err(| e | VkError::try_from(e).unwrap_unchecked())?;

//         Ok(
//             Arc::new(
//                 Self {
//                     buffer,
//                     buffer_view,

//                     create_info
//                 }
//             )
//         )
//     }

//     pub fn format(&self) -> Format {
//         self.create_info.format
//     }

//     pub fn range(&self) -> Range<u64> {
//         self.create_info.range.clone()
//     }
// }

// impl<A: DeviceMemoryAllocator> Drop for BufferView<A> {
//     fn drop(&mut self) {
//         unsafe {
//             self.buffer.device.destroy_buffer_view(
//                 self.buffer_view,
//                 None
//             )
//         }
//     }
// }