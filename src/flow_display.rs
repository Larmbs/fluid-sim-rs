use crate::flow_box::{self, FlowBox};
use glam::ivec2;
use miniquad::*;
use rayon::prelude::*;
#[repr(C)]
struct Vertex {
    in_color: [f32; 3],
}

fn prepare_vertex_data<const C: usize>(
    density: &flow_box::Density<C>,
    dim_sim: (usize, usize),
) -> Vec<Vertex> {
    (0..dim_sim.0 * dim_sim.1)
        .into_par_iter()
        .map(|i| Vertex {
            in_color: [density.r[i], density.g[i], density.b[i]],
        })
        .collect::<Vec<Vertex>>()
}
fn create_index_buffer(dim_sim: (usize, usize)) -> Vec<u16> {
    let mut indices = Vec::new();
    for y in 0..dim_sim.1 - 1 {
        for x in 0..dim_sim.0 - 1 {
            let i = (y * dim_sim.0 + x) as u16;
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + dim_sim.0 as u16);

            indices.push(i + 1);
            indices.push(i + 1 + dim_sim.0 as u16);
            indices.push(i + dim_sim.0 as u16);
        }
    }
    indices
}

pub struct FlowDisplay<const C: usize> {
    flow_box: FlowBox<C>,
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    iter: u128,
    index_buffer_len: usize,
}

impl<const C: usize> FlowDisplay<C> {
    pub fn new(flow_box: FlowBox<C>) -> FlowDisplay<C> {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&prepare_vertex_data(&flow_box.density, flow_box.dim)),
        );
        let index_buffer = create_index_buffer(flow_box.dim);
        let index_buffer_len: usize = index_buffer.len();
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&index_buffer),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::new("in_color", VertexFormat::Float3)],
            shader,
            PipelineParams::default(),
        );

        FlowDisplay::<C> {
            pipeline,
            bindings,
            ctx,
            flow_box,
            iter: 0,
            index_buffer_len,
        }
    }
}

impl<const C: usize> EventHandler for FlowDisplay<C> {
    fn update(&mut self) {
        let pos = (self.flow_box.dim.0 / 2, self.flow_box.dim.1 / 2);
        let angle = self.iter as f32 / 60.;

        self.flow_box
            .add_fluid_velocity_angle_mag(pos.0, pos.1, angle, 90000.0);
        self.flow_box.add_fluid_density(
            pos.0,
            pos.1,
            ((angle * 3.0) % 1.0, angle % 1.0, (angle * 2.0) % 1.0),
        );

        self.flow_box.step(1.0 / 30.0);
        self.iter = self.iter.wrapping_add(1);
        self.ctx.buffer_update(
            self.bindings.vertex_buffers[0],
            BufferSource::slice(&prepare_vertex_data(
                &self.flow_box.density,
                self.flow_box.dim,
            )),
        )
    }

    fn draw(&mut self) {
        let sim_dim = ivec2(self.flow_box.dim.0 as i32, self.flow_box.dim.1 as i32);
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms { sim_dim }));

        self.ctx.draw(0, self.index_buffer_len as i32, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = include_str!("../shaders/vert_shader.glsl");

    pub const FRAGMENT: &str = include_str!("../shaders/frag_shader.glsl");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("sim_dim", UniformType::Int2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub sim_dim: glam::IVec2,
    }
}
