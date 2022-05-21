use iced_elysium_gl::elysium_gl;
use iced_elysium_gl::Color;

pub struct Scene {
    program: u32,
    vertex_array: u32,
}

impl Scene {
    #[inline]
    pub fn new(gl: &elysium_gl::Context, shader_version: &str) -> Self {
        let vertex_array = gl.create_vertex_array();

        gl.bind_vertex_array(vertex_array);

        let program = gl.create_program();

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                    vec2(0.5f, 1.0f),
                    vec2(0.0f, 0.0f),
                    vec2(1.0f, 0.0f)
                );
                out vec2 vert;
                void main() {
                    vert = verts[gl_VertexID];
                    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
                }"#,
            r#"precision highp float;
                in vec2 vert;
                out vec4 color;
                void main() {
                    color = vec4(vert, 0.5, 1.0);
                }"#,
        );

        let shader_sources = [
            (elysium_gl::VERTEX_SHADER, vertex_shader_source),
            (elysium_gl::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl.create_shader(*shader_type);

            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));

            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }

            gl.attach_shader(program, shader);

            shaders.push(shader);
        }

        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            //panic!("{}", gl.get_program_info_log(program));
            panic!("link");
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(program);
        Self {
            program,
            vertex_array,
        }
    }

    #[inline]
    pub fn clear(&self, gl: &elysium_gl::Context, background_color: Color) {
        let [r, g, b, a] = background_color.into_linear();

        gl.clear_color(r, g, b, a);
        gl.clear(elysium_gl::COLOR_BUFFER_BIT);
    }

    #[inline]
    pub fn draw(&self, gl: &elysium_gl::Context) {
        gl.bind_vertex_array(self.vertex_array);
        gl.use_program(self.program);
        gl.draw_arrays(elysium_gl::TRIANGLES, 0, 3);
    }

    #[inline]
    pub fn cleanup(&self, gl: &elysium_gl::Context) {
        gl.delete_program(self.program);
        gl.delete_vertex_array(self.vertex_array);
    }
}
