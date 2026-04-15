use super::atlas::NUM_TIERS;

pub(super) fn create_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("WoW UI Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    })
}

pub(super) fn create_glyph_atlas(
    device: &wgpu::Device,
    size: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Glyph Atlas"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn create_atlas_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let texture_entry = |binding: u32| wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("WoW UI Texture Bind Group Layout"),
        entries: &[
            texture_entry(0),
            texture_entry(1),
            texture_entry(2),
            texture_entry(3),
            texture_entry(4),
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            texture_entry(6),
            texture_entry(7),
            texture_entry(8),
        ],
    })
}

pub(super) fn create_atlas_bind_groups(
    device: &wgpu::Device,
    tier_views: [&wgpu::TextureView; NUM_TIERS],
    glyph_view: &wgpu::TextureView,
    bc1_view: &wgpu::TextureView,
    bc3_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let layout = create_atlas_bind_group_layout(device);
    let views: [&wgpu::TextureView; 8] = [
        tier_views[0],
        tier_views[1],
        tier_views[2],
        tier_views[3],
        tier_views[4],
        glyph_view,
        bc1_view,
        bc3_view,
    ];
    let mut entries: Vec<wgpu::BindGroupEntry<'_>> = views
        .iter()
        .enumerate()
        .map(|(i, view)| wgpu::BindGroupEntry {
            binding: i as u32,
            resource: wgpu::BindingResource::TextureView(view),
        })
        .collect();
    entries.insert(
        5,
        wgpu::BindGroupEntry {
            binding: 5,
            resource: wgpu::BindingResource::Sampler(sampler),
        },
    );
    for (i, entry) in entries.iter_mut().enumerate() {
        entry.binding = i as u32;
    }
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("WoW UI Texture Bind Group"),
        layout: &layout,
        entries: &entries,
    });
    (layout, bind_group)
}
