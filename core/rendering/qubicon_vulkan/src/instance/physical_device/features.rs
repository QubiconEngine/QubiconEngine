macro_rules! decl_features_struct {
    {
        $( #[derive( $( $macro_name:ident ),+ )] )?
        $name:ident,
        $( $field_name:ident ),+
    } => {
        $( #[derive( $( $macro_name ),+ )] )?
        pub struct $name {
            $( pub $field_name: bool, )+
        }
    };
}

decl_features_struct!(
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    DeviceFeatures,

    robust_buffer_access,
    full_draw_index_uint32,
    image_cube_array,
    independent_blend,
    geometry_shader,
    tessellation_shader,
    sample_rate_shading,
    dual_src_blend,
    logic_op,
    multi_draw_indirect,
    draw_indirect_first_instance,
    depth_clamp,
    depth_bias_clamp,
    fill_mode_non_solid,
    depth_bounds,
    wide_lines,
    large_points,
    alpha_to_one,
    multi_viewport,
    sampler_anisotropy,
    texture_compression_etc2,
    texture_compression_astc_ldr,
    texture_compression_bc,
    occlusion_query_precise,
    pipeline_statistics_query,
    vertex_pipeline_stores_and_atomics,
    fragment_stores_and_atomics,
    shader_tessellation_and_geometry_point_size,
    shader_image_gather_extended,
    shader_storage_image_extended_formats,
    shader_storage_image_multisample,
    shader_storage_image_read_without_format,
    shader_storage_image_write_without_format,
    shader_uniform_buffer_array_dynamic_indexing,
    shader_sampled_image_array_dynamic_indexing,
    shader_storage_buffer_array_dynamic_indexing,
    shader_storage_image_array_dynamic_indexing,
    shader_clip_distance,
    shader_cull_distance,
    shader_float64,
    shader_int64,
    shader_int16,
    shader_resource_residency,
    shader_resource_min_lod,
    sparse_binding,
    sparse_residency_buffer,
    sparse_residency_image2_d,
    sparse_residency_image3_d,
    sparse_residency2_samples,
    sparse_residency4_samples,
    sparse_residency8_samples,
    sparse_residency16_samples,
    sparse_residency_aliased,
    variable_multisample_rate,
    inherited_queries
);

macro_rules! conv_to_ash {
    ( $value:ident, $( $feature:ident ),+ ) => {
        ash::vk::PhysicalDeviceFeatures {
            $( $feature: $value.$feature as u32 ),+
        }
    };
}

macro_rules! conv_from_ash {
    ( $value:ident, $( $feature:ident ),+ ) => {
        DeviceFeatures {
            $( $feature: $value.$feature == ash::vk::TRUE ),+
        }
    };
}

impl From<DeviceFeatures> for ash::vk::PhysicalDeviceFeatures {
    fn from(value: DeviceFeatures) -> Self {
        conv_to_ash!(
            value,

            robust_buffer_access,
            full_draw_index_uint32,
            image_cube_array,
            independent_blend,
            geometry_shader,
            tessellation_shader,
            sample_rate_shading,
            dual_src_blend,
            logic_op,
            multi_draw_indirect,
            draw_indirect_first_instance,
            depth_clamp,
            depth_bias_clamp,
            fill_mode_non_solid,
            depth_bounds,
            wide_lines,
            large_points,
            alpha_to_one,
            multi_viewport,
            sampler_anisotropy,
            texture_compression_etc2,
            texture_compression_astc_ldr,
            texture_compression_bc,
            occlusion_query_precise,
            pipeline_statistics_query,
            vertex_pipeline_stores_and_atomics,
            fragment_stores_and_atomics,
            shader_tessellation_and_geometry_point_size,
            shader_image_gather_extended,
            shader_storage_image_extended_formats,
            shader_storage_image_multisample,
            shader_storage_image_read_without_format,
            shader_storage_image_write_without_format,
            shader_uniform_buffer_array_dynamic_indexing,
            shader_sampled_image_array_dynamic_indexing,
            shader_storage_buffer_array_dynamic_indexing,
            shader_storage_image_array_dynamic_indexing,
            shader_clip_distance,
            shader_cull_distance,
            shader_float64,
            shader_int64,
            shader_int16,
            shader_resource_residency,
            shader_resource_min_lod,
            sparse_binding,
            sparse_residency_buffer,
            sparse_residency_image2_d,
            sparse_residency_image3_d,
            sparse_residency2_samples,
            sparse_residency4_samples,
            sparse_residency8_samples,
            sparse_residency16_samples,
            sparse_residency_aliased,
            variable_multisample_rate,
            inherited_queries
        )
    }
}

impl From<ash::vk::PhysicalDeviceFeatures> for DeviceFeatures {
    fn from(value: ash::vk::PhysicalDeviceFeatures) -> Self {
        conv_from_ash!(
            value,

            robust_buffer_access,
            full_draw_index_uint32,
            image_cube_array,
            independent_blend,
            geometry_shader,
            tessellation_shader,
            sample_rate_shading,
            dual_src_blend,
            logic_op,
            multi_draw_indirect,
            draw_indirect_first_instance,
            depth_clamp,
            depth_bias_clamp,
            fill_mode_non_solid,
            depth_bounds,
            wide_lines,
            large_points,
            alpha_to_one,
            multi_viewport,
            sampler_anisotropy,
            texture_compression_etc2,
            texture_compression_astc_ldr,
            texture_compression_bc,
            occlusion_query_precise,
            pipeline_statistics_query,
            vertex_pipeline_stores_and_atomics,
            fragment_stores_and_atomics,
            shader_tessellation_and_geometry_point_size,
            shader_image_gather_extended,
            shader_storage_image_extended_formats,
            shader_storage_image_multisample,
            shader_storage_image_read_without_format,
            shader_storage_image_write_without_format,
            shader_uniform_buffer_array_dynamic_indexing,
            shader_sampled_image_array_dynamic_indexing,
            shader_storage_buffer_array_dynamic_indexing,
            shader_storage_image_array_dynamic_indexing,
            shader_clip_distance,
            shader_cull_distance,
            shader_float64,
            shader_int64,
            shader_int16,
            shader_resource_residency,
            shader_resource_min_lod,
            sparse_binding,
            sparse_residency_buffer,
            sparse_residency_image2_d,
            sparse_residency_image3_d,
            sparse_residency2_samples,
            sparse_residency4_samples,
            sparse_residency8_samples,
            sparse_residency16_samples,
            sparse_residency_aliased,
            variable_multisample_rate,
            inherited_queries
        )
    }
}