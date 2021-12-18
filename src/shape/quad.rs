use crate::vertex::Vertex;
use wgpu::util::DeviceExt;

pub struct QuadBuffer {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    pipeline: wgpu::RenderPipeline,
}

impl QuadBuffer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",     // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
        });

        QuadBuffer {
            vertices: Vec::new(),
            indices: Vec::new(),
            pipeline: render_pipeline,
        }
    }

    pub fn push(&mut self, quad: &[Vertex; 4]) {
        self.vertices.extend(quad);
        let count: u32 = (self.vertices.len() / 4 - 1).try_into().unwrap();
        self.indices.extend(&[
            count * 4 + 0,
            count * 4 + 1,
            count * 4 + 2,
            count * 4 + 0,
            count * 4 + 2,
            count * 4 + 3,
        ]);
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        // 1.
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("B Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline); // 2.
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indices.len().try_into().unwrap(), 0, 0..1);
        // 3.
    }
}
