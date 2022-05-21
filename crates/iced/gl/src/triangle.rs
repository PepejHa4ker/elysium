//! Draw meshes of triangles.
use crate::program::{self, Shader};
use crate::Transformation;
use iced_graphics::layer;
use std::marker::PhantomData;

pub use iced_graphics::triangle::{Mesh2D, Vertex2D};

const VERTEX_BUFFER_SIZE: usize = 10_000;
const INDEX_BUFFER_SIZE: usize = 10_000;

#[derive(Debug)]
pub(crate) struct Pipeline {
    program: u32,
    vertex_array: u32,
    vertices: Buffer<Vertex2D>,
    indices: Buffer<u32>,
    transform_location: u32,
    current_transform: Transformation,
}

impl Pipeline {
    pub fn new(gl: &elysium_gl::Context, shader_version: &program::Version) -> Pipeline {
        let program = {
            let vertex_shader = Shader::vertex(
                gl,
                shader_version,
                include_str!("shader/common/triangle.vert"),
            );

            let fragment_shader = Shader::fragment(
                gl,
                shader_version,
                include_str!("shader/common/triangle.frag"),
            );

            unsafe {
                program::create(
                    gl,
                    &[vertex_shader, fragment_shader],
                    &[(0, "i_Position"), (1, "i_Color")],
                )
            }
        };

        let transform_location = gl
            .get_uniform_location(program, "u_Transform")
            .expect("Get transform location");

        gl.use_program(program);

        let transform: [f32; 16] = Transformation::identity().into();

        gl.uniform_matrix_4_f32_slice(transform_location, false, &transform);
        gl.use_program(0);

        let vertex_array = gl.create_vertex_array();

        gl.bind_vertex_array(vertex_array);

        let vertices = unsafe {
            Buffer::new(
                gl,
                elysium_gl::ARRAY_BUFFER,
                elysium_gl::DYNAMIC_DRAW,
                VERTEX_BUFFER_SIZE,
            )
        };

        let indices = unsafe {
            Buffer::new(
                gl,
                elysium_gl::ELEMENT_ARRAY_BUFFER,
                elysium_gl::DYNAMIC_DRAW,
                INDEX_BUFFER_SIZE,
            )
        };

        let stride = std::mem::size_of::<Vertex2D>() as i32;

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, elysium_gl::FLOAT, false, stride, 0);

        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 4, elysium_gl::FLOAT, false, stride, 4 * 2);

        gl.bind_vertex_array(0);

        Pipeline {
            program,
            vertex_array,
            vertices,
            indices,
            transform_location,
            current_transform: Transformation::identity(),
        }
    }

    pub fn draw(
        &mut self,
        gl: &elysium_gl::Context,
        target_height: u32,
        transformation: Transformation,
        scale_factor: f32,
        meshes: &[layer::Mesh<'_>],
    ) {
        gl.enable(elysium_gl::MULTISAMPLE);
        gl.enable(elysium_gl::SCISSOR_TEST);
        gl.use_program(self.program);
        gl.bind_vertex_array(self.vertex_array);

        // This looks a bit crazy, but we are just counting how many vertices
        // and indices we will need to handle.
        // TODO: Improve readability
        let (total_vertices, total_indices) = meshes
            .iter()
            .map(|layer::Mesh { buffers, .. }| (buffers.vertices.len(), buffers.indices.len()))
            .fold((0, 0), |(total_v, total_i), (v, i)| {
                (total_v + v, total_i + i)
            });

        // Then we ensure the current buffers are big enough, resizing if
        // necessary
        unsafe {
            self.vertices.bind(gl, total_vertices);
            self.indices.bind(gl, total_indices);
        }

        // We upload all the vertices and indices upfront
        let mut last_vertex = 0;
        let mut last_index = 0;

        for layer::Mesh { buffers, .. } in meshes {
            gl.buffer_sub_data_u8_slice(
                elysium_gl::ARRAY_BUFFER,
                (last_vertex * std::mem::size_of::<Vertex2D>()) as i32,
                bytemuck::cast_slice(&buffers.vertices),
            );

            gl.buffer_sub_data_u8_slice(
                elysium_gl::ELEMENT_ARRAY_BUFFER,
                (last_index * std::mem::size_of::<u32>()) as i32,
                bytemuck::cast_slice(&buffers.indices),
            );

            last_vertex += buffers.vertices.len();
            last_index += buffers.indices.len();
        }

        // Then we draw each mesh using offsets
        let mut last_vertex = 0;
        let mut last_index = 0;

        for layer::Mesh {
            buffers,
            origin,
            clip_bounds,
        } in meshes
        {
            let transform = transformation * Transformation::translate(origin.x, origin.y);

            let clip_bounds = (*clip_bounds * scale_factor).snap();

            if self.current_transform != transform {
                let matrix: [f32; 16] = transform.into();

                gl.uniform_matrix_4_f32_slice(self.transform_location, false, &matrix);

                self.current_transform = transform;
            }

            gl.scissor(
                clip_bounds.x as i32,
                (target_height - (clip_bounds.y + clip_bounds.height)) as i32,
                clip_bounds.width as i32,
                clip_bounds.height as i32,
            );

            gl.draw_elements_base_vertex(
                elysium_gl::TRIANGLES,
                buffers.indices.len() as i32,
                elysium_gl::UNSIGNED_INT,
                (last_index * std::mem::size_of::<u32>()) as i32,
                last_vertex as i32,
            );

            last_vertex += buffers.vertices.len();
            last_index += buffers.indices.len();
        }

        gl.bind_vertex_array(0);
        gl.use_program(0);
        gl.disable(elysium_gl::SCISSOR_TEST);
        gl.disable(elysium_gl::MULTISAMPLE);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Uniforms {
    transform: [f32; 16],
}

unsafe impl bytemuck::Zeroable for Uniforms {}
unsafe impl bytemuck::Pod for Uniforms {}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: *Transformation::identity().as_ref(),
        }
    }
}

impl From<Transformation> for Uniforms {
    fn from(transformation: Transformation) -> Uniforms {
        Self {
            transform: transformation.into(),
        }
    }
}

#[derive(Debug)]
struct Buffer<T> {
    raw: u32,
    target: u32,
    usage: u32,
    size: usize,
    phantom: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub unsafe fn new(gl: &elysium_gl::Context, target: u32, usage: u32, size: usize) -> Self {
        let raw = gl.create_buffer();

        let mut buffer = Buffer {
            raw,
            target,
            usage,
            size: 0,
            phantom: PhantomData,
        };

        buffer.bind(gl, size);
        buffer
    }

    pub unsafe fn bind(&mut self, gl: &elysium_gl::Context, size: usize) {
        gl.bind_buffer(self.target, self.raw);

        if self.size < size {
            gl.buffer_data_size(
                self.target,
                (size * std::mem::size_of::<T>()) as i32,
                self.usage,
            );

            self.size = size;
        }
    }
}
