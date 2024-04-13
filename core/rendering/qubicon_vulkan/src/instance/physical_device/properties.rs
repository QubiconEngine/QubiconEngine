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

    pub limits: DeviceLimits
    // TODO: sparse properties
}

impl From<ash::vk::PhysicalDeviceProperties> for DeviceProperties {
    fn from(value: ash::vk::PhysicalDeviceProperties) -> Self {    
        Self {
            driver_version: value.driver_version,
            vendor_id: value.vendor_id,
            device_id: value.device_id,
            device_type: value.device_type.into(),
            device_name: ArrayString::from_byte_string(unsafe { core::mem::transmute(&value.device_name) }).unwrap(),
            pipeline_chache_uuid: value.pipeline_cache_uuid,

            limits: value.limits.into()
        }
    }
}

impl From<ash::vk::PhysicalDeviceType> for DeviceType {
    fn from(value: ash::vk::PhysicalDeviceType) -> Self {
        unsafe {core::mem::transmute(value.as_raw() as u8)}
    }
}



// fuck, its practicaly the same as just normal declaration
macro_rules! decl_device_limits_struct {
    {
        $( #[derive( $( $trait_name:ident ),* )] )?
        $name:ident {
            $( $limit_name:ident: $ty:tt ),*
        }
    } => {
        $( #[derive( $( $trait_name ),* )] )?
        pub struct $name {
            $( pub $limit_name: $ty ),*
        }
    };
}

decl_device_limits_struct!{
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    DeviceLimits {
        max_image_dimension1_d: u32,
        max_image_dimension2_d: u32,
        max_image_dimension3_d: u32,
        max_image_dimension_cube: u32,
        max_image_array_layers: u32,
        max_texel_buffer_elements: u32,
        max_uniform_buffer_range: u32,
        max_storage_buffer_range: u32,
        max_push_constants_size: u32,
        max_memory_allocation_count: u32,
        max_sampler_allocation_count: u32,
        //buffer_image_granularity: DeviceSize,
        //sparse_address_space_size: DeviceSize,
        max_bound_descriptor_sets: u32,
        max_per_stage_descriptor_samplers: u32,
        max_per_stage_descriptor_uniform_buffers: u32,
        max_per_stage_descriptor_storage_buffers: u32,
        max_per_stage_descriptor_sampled_images: u32,
        max_per_stage_descriptor_storage_images: u32,
        max_per_stage_descriptor_input_attachments: u32,
        max_per_stage_resources: u32,
        max_descriptor_set_samplers: u32,
        max_descriptor_set_uniform_buffers: u32,
        max_descriptor_set_uniform_buffers_dynamic: u32,
        max_descriptor_set_storage_buffers: u32,
        max_descriptor_set_storage_buffers_dynamic: u32,
        max_descriptor_set_sampled_images: u32,
        max_descriptor_set_storage_images: u32,
        max_descriptor_set_input_attachments: u32,
        max_vertex_input_attributes: u32,
        max_vertex_input_bindings: u32,
        max_vertex_input_attribute_offset: u32,
        max_vertex_input_binding_stride: u32,
        max_vertex_output_components: u32,
        max_tessellation_generation_level: u32,
        max_tessellation_patch_size: u32,
        max_tessellation_control_per_vertex_input_components: u32,
        max_tessellation_control_per_vertex_output_components: u32,
        max_tessellation_control_per_patch_output_components: u32,
        max_tessellation_control_total_output_components: u32,
        max_tessellation_evaluation_input_components: u32,
        max_tessellation_evaluation_output_components: u32,
        max_geometry_shader_invocations: u32,
        max_geometry_input_components: u32,
        max_geometry_output_components: u32,
        max_geometry_output_vertices: u32,
        max_geometry_total_output_components: u32,
        max_fragment_input_components: u32,
        max_fragment_output_attachments: u32,
        max_fragment_dual_src_attachments: u32,
        max_fragment_combined_output_resources: u32,
        max_compute_shared_memory_size: u32,
        max_compute_work_group_count: [u32; 3],
        max_compute_work_group_invocations: u32,
        max_compute_work_group_size: [u32; 3],
        sub_pixel_precision_bits: u32,
        sub_texel_precision_bits: u32,
        mipmap_precision_bits: u32,
        max_draw_indexed_index_value: u32,
        max_draw_indirect_count: u32,
        max_sampler_lod_bias: f32,
        max_sampler_anisotropy: f32,
        max_viewports: u32,
        max_viewport_dimensions: [u32; 2],
        viewport_bounds_range: [f32; 2],
        viewport_sub_pixel_bits: u32,
        min_memory_map_alignment: usize,
        //min_texel_buffer_offset_alignment: DeviceSize,
        //min_uniform_buffer_offset_alignment: DeviceSize,
        //min_storage_buffer_offset_alignment: DeviceSize,
        min_texel_offset: i32,
        max_texel_offset: u32,
        min_texel_gather_offset: i32,
        max_texel_gather_offset: u32,
        min_interpolation_offset: f32,
        max_interpolation_offset: f32,
        sub_pixel_interpolation_offset_bits: u32,
        max_framebuffer_width: u32,
        max_framebuffer_height: u32,
        max_framebuffer_layers: u32,
        //framebuffer_color_sample_counts: SampleCountFlags,
        //framebuffer_depth_sample_counts: SampleCountFlags,
        //framebuffer_stencil_sample_counts: SampleCountFlags,
        //framebuffer_no_attachments_sample_counts: SampleCountFlags,
        max_color_attachments: u32,
        //sampled_image_color_sample_counts: SampleCountFlags,
        //sampled_image_integer_sample_counts: SampleCountFlags,
        //sampled_image_depth_sample_counts: SampleCountFlags,
        //sampled_image_stencil_sample_counts: SampleCountFlags,
        //storage_image_sample_counts: SampleCountFlags,
        max_sample_mask_words: u32,
        timestamp_compute_and_graphics: bool,
        timestamp_period: f32,
        max_clip_distances: u32,
        max_cull_distances: u32,
        max_combined_clip_and_cull_distances: u32,
        discrete_queue_priorities: u32,
        point_size_range: [f32; 2],
        line_width_range: [f32; 2],
        point_size_granularity: f32,
        line_width_granularity: f32,
        strict_lines: bool,
        standard_sample_locations: bool//,
        //optimal_buffer_copy_offset_alignment: DeviceSize,
        //optimal_buffer_copy_row_pitch_alignment: DeviceSize,
        //non_coherent_atom_size: DeviceSize
    }
}

macro_rules! convert_field {
    ( $self:ident, $value:ident, $field_name:ident: ( bool, Bool32 ) ) => {
        $self.$field_name = $value.$field_name as bool;
    };
    ( $self:ident, $value:ident, $field_name:ident ) => {
        $self.$field_name = $value.$field_name;
    };
}

macro_rules! convert_fields {
    ( $self:ident, $value:ident, $( $field_name:ident $(: $conversion_ty:tt )? ),* ) => {
        $( convert_field!($self, $value, $field_name $(: $conversion_ty)?); )*
    };
}

impl From<ash::vk::PhysicalDeviceLimits> for DeviceLimits {
    fn from(value: ash::vk::PhysicalDeviceLimits) -> Self {
        let mut out = Self::default();

        convert_fields!(
            out,
            value,

            max_image_dimension1_d,
            max_image_dimension2_d,
            max_image_dimension3_d,
            max_image_dimension_cube,
            max_image_array_layers,
            max_texel_buffer_elements,
            max_uniform_buffer_range,
            max_storage_buffer_range,
            max_push_constants_size,
            max_memory_allocation_count,
            max_sampler_allocation_count,
            //buffer_image_granularity: DeviceSize,
            //sparse_address_space_size: DeviceSize,
            max_bound_descriptor_sets,
            max_per_stage_descriptor_samplers,
            max_per_stage_descriptor_uniform_buffers,
            max_per_stage_descriptor_storage_buffers,
            max_per_stage_descriptor_sampled_images,
            max_per_stage_descriptor_storage_images,
            max_per_stage_descriptor_input_attachments,
            max_per_stage_resources,
            max_descriptor_set_samplers,
            max_descriptor_set_uniform_buffers,
            max_descriptor_set_uniform_buffers_dynamic,
            max_descriptor_set_storage_buffers,
            max_descriptor_set_storage_buffers_dynamic,
            max_descriptor_set_sampled_images,
            max_descriptor_set_storage_images,
            max_descriptor_set_input_attachments,
            max_vertex_input_attributes,
            max_vertex_input_bindings,
            max_vertex_input_attribute_offset,
            max_vertex_input_binding_stride,
            max_vertex_output_components,
            max_tessellation_generation_level,
            max_tessellation_patch_size,
            max_tessellation_control_per_vertex_input_components,
            max_tessellation_control_per_vertex_output_components,
            max_tessellation_control_per_patch_output_components,
            max_tessellation_control_total_output_components,
            max_tessellation_evaluation_input_components,
            max_tessellation_evaluation_output_components,
            max_geometry_shader_invocations,
            max_geometry_input_components,
            max_geometry_output_components,
            max_geometry_output_vertices,
            max_geometry_total_output_components,
            max_fragment_input_components,
            max_fragment_output_attachments,
            max_fragment_dual_src_attachments,
            max_fragment_combined_output_resources,
            max_compute_shared_memory_size,
            max_compute_work_group_count,
            max_compute_work_group_invocations,
            max_compute_work_group_size,
            sub_pixel_precision_bits,
            sub_texel_precision_bits,
            mipmap_precision_bits,
            max_draw_indexed_index_value,
            max_draw_indirect_count,
            max_sampler_lod_bias,
            max_sampler_anisotropy,
            max_viewports,
            max_viewport_dimensions,
            viewport_bounds_range,
            viewport_sub_pixel_bits,
            min_memory_map_alignment,
            //min_texel_buffer_offset_alignment: DeviceSize,
            //min_uniform_buffer_offset_alignment: DeviceSize,
            //min_storage_buffer_offset_alignment: DeviceSize,
            min_texel_offset,
            max_texel_offset,
            min_texel_gather_offset,
            max_texel_gather_offset,
            min_interpolation_offset,
            max_interpolation_offset,
            sub_pixel_interpolation_offset_bits,
            max_framebuffer_width,
            max_framebuffer_height,
            max_framebuffer_layers,
            //framebuffer_color_sample_counts: SampleCountFlags,
            //framebuffer_depth_sample_counts: SampleCountFlags,
            //framebuffer_stencil_sample_counts: SampleCountFlags,
            //framebuffer_no_attachments_sample_counts: SampleCountFlags,
            max_color_attachments,
            //sampled_image_color_sample_counts: SampleCountFlags,
            //sampled_image_integer_sample_counts: SampleCountFlags,
            //sampled_image_depth_sample_counts: SampleCountFlags,
            //sampled_image_stencil_sample_counts: SampleCountFlags,
            //storage_image_sample_counts: SampleCountFlags,
            max_sample_mask_words,
            timestamp_compute_and_graphics: (bool, Bool32),
            timestamp_period,
            max_clip_distances,
            max_cull_distances,
            max_combined_clip_and_cull_distances,
            discrete_queue_priorities,
            point_size_range,
            line_width_range,
            point_size_granularity,
            line_width_granularity,
            strict_lines: (bool, Bool32),
            standard_sample_locations: (bool, Bool32)//,
            //optimal_buffer_copy_offset_alignment: DeviceSize,
            //optimal_buffer_copy_row_pitch_alignment: DeviceSize,
            //non_coherent_atom_size: DeviceSize
        );

        out
    }
}