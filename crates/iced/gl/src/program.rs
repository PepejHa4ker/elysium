/// The [`Version`] of a `Program`.
pub struct Version {
    vertex: String,
    fragment: String,
}

impl Version {
    pub fn new(_gl: &elysium_gl::Context) -> Version {
        let vertex = "#version 410".into();
        let fragment = "#version 410".into();

        Version { vertex, fragment }
    }
}

pub struct Shader(u32);

impl Shader {
    fn compile(gl: &elysium_gl::Context, stage: u32, content: &str) -> Shader {
        let shader = gl.create_shader(stage);

        gl.shader_source(shader, &content);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }

        Shader(shader)
    }

    /// Creates a vertex [`Shader`].
    pub fn vertex(gl: &elysium_gl::Context, version: &Version, content: &'static str) -> Self {
        let content = format!("{}\n{}", version.vertex, content);

        Shader::compile(gl, elysium_gl::VERTEX_SHADER, &content)
    }

    /// Creates a fragment [`Shader`].
    pub fn fragment(gl: &elysium_gl::Context, version: &Version, content: &'static str) -> Self {
        let content = format!("{}\n{}", version.fragment, content);

        Shader::compile(gl, elysium_gl::FRAGMENT_SHADER, &content)
    }
}

pub unsafe fn create(
    gl: &elysium_gl::Context,
    shaders: &[Shader],
    attributes: &[(u32, &str)],
) -> u32 {
    let program = gl.create_program();

    for shader in shaders {
        gl.attach_shader(program, shader.0);
    }

    for (i, name) in attributes {
        gl.bind_attrib_location(program, *i, name);
    }

    gl.link_program(program);

    if !gl.get_program_link_status(program) {
        //panic!("{}", gl.get_program_info_log(program));
        panic!("link failure");
    }

    for shader in shaders {
        gl.detach_shader(program, shader.0);
        gl.delete_shader(shader.0);
    }

    program
}
