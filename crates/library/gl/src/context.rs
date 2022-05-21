use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::ptr;

use crate::cstr_cow_from_bytes;
use crate::PixelUnpackData;
use crate::{def_proc, get_proc};
use crate::{COMPILE_STATUS, INFO_LOG_LENGTH, LINK_STATUS};

/// GL functions.
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    active_texture: Option<unsafe extern "C" fn(unit: u32)>,
    attach_shader: Option<unsafe extern "C" fn(program: u32, shader: u32)>,
    bind_buffer: Option<unsafe extern "C" fn(target: u32, buffer: u32)>,
    bind_texture: Option<unsafe extern "C" fn(target: u32, texture: u32)>,
    bind_vertex_array: Option<unsafe extern "C" fn(array: u32)>,
    _bind_attrib_location: Option<unsafe extern "C" fn(program: u32, index: u32, name: *const u8)>,
    blend_func: Option<unsafe extern "C" fn(src: u32, dst: u32)>,
    blend_func_separate:
        Option<unsafe extern "C" fn(src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32)>,
    buffer_data:
        Option<unsafe extern "C" fn(target: u32, data_len: isize, data: *const (), usage: u32)>,
    buffer_sub_data: Option<
        unsafe extern "C" fn(target: u32, offset: isize, src_data_len: isize, src_data: *const ()),
    >,
    clear: Option<unsafe extern "C" fn(mask: u32)>,
    clear_color: Option<unsafe extern "C" fn(r: f32, g: f32, b: f32, a: f32)>,
    compile_shader: Option<unsafe extern "C" fn(shader: u32)>,
    create_program: Option<unsafe extern "C" fn() -> u32>,
    create_shader: Option<unsafe extern "C" fn(shader_type: u32) -> u32>,
    delete_buffers: Option<unsafe extern "C" fn(len: i32, buffers: *const u32)>,
    delete_program: Option<unsafe extern "C" fn(program: u32)>,
    delete_shader: Option<unsafe extern "C" fn(shader: u32)>,
    delete_textures: Option<unsafe extern "C" fn(len: i32, textures: *const u32)>,
    delete_vertex_arrays: Option<unsafe extern "C" fn(len: i32, vertex_arrays: *const u32)>,
    detach_shader: Option<unsafe extern "C" fn(program: u32, shader: u32)>,
    disable: Option<unsafe extern "C" fn(parameter: u32)>,
    draw_arrays: Option<unsafe extern "C" fn(mode: u32, first: i32, count: i32)>,
    draw_arrays_instanced:
        Option<unsafe extern "C" fn(mode: u32, first: i32, count: i32, instance_count: i32)>,
    _draw_elements:
        Option<unsafe extern "C" fn(mode: u32, count: i32, element_type: u32, offset: *const ())>,
    _draw_elements_base_vertex: Option<
        unsafe extern "C" fn(
            mode: u32,
            count: i32,
            kind: u32,
            indices: *const (),
            base_vertex: i32,
        ),
    >,
    enable: Option<unsafe extern "C" fn(parameter: u32)>,
    enable_vertex_attrib_array: Option<unsafe extern "C" fn(index: u32)>,
    gen_buffers: Option<unsafe extern "C" fn(len: i32, buffers: *mut u32)>,
    gen_textures: Option<unsafe extern "C" fn(len: i32, textures: *mut u32)>,
    gen_vertex_arrays: Option<unsafe extern "C" fn(len: i32, vertex_array: *mut u32)>,
    get_integerv: Option<unsafe extern "C" fn(pamarameter: u32, data: *mut i32)>,
    get_programiv: Option<unsafe extern "C" fn(program: u32, name: u32, params: *mut i32)>,
    get_shaderiv: Option<unsafe extern "C" fn(shader: u32, name: u32, params: *mut i32)>,
    _get_shader_info_log: Option<
        unsafe extern "C" fn(shader: u32, max_length: i32, length: *mut i32, info_log: *mut u8),
    >,
    _get_uniform_location: Option<unsafe extern "C" fn(program: u32, name: *const u8) -> i32>,
    link_program: Option<unsafe extern "C" fn(program: u32)>,
    pixel_storei: Option<unsafe extern "C" fn(parameter: u32, value: i32)>,
    scissor: Option<unsafe extern "C" fn(x: i32, y: i32, width: i32, height: i32)>,
    _shader_source: Option<
        unsafe extern "C" fn(shader: u32, count: i32, string: *const *const u8, len: *const i32),
    >,
    _tex_image_2d: Option<
        unsafe extern "C" fn(
            target: u32,
            level: i32,
            internal_format: i32,
            width: i32,
            height: i32,
            border: i32,
            format: u32,
            ty: u32,
            pixels: *const (),
        ),
    >,
    tex_parameteri: Option<unsafe extern "C" fn(target: u32, parameter: u32, value: i32)>,
    tex_parameteriv: Option<unsafe extern "C" fn(target: u32, parameter: u32, values: *const i32)>,
    _tex_sub_image_2d: Option<
        unsafe extern "C" fn(
            target: u32,
            level: i32,
            internal_format: i32,
            width: i32,
            height: i32,
            border: i32,
            format: u32,
            ty: u32,
            pixels: *const (),
        ),
    >,
    uniform1f: Option<unsafe extern "C" fn(location: i32, data: f32)>,
    uniform1i: Option<unsafe extern "C" fn(location: i32, data: i32)>,
    uniform_matrix4fv:
        Option<unsafe extern "C" fn(location: i32, len: i32, transpose: u8, data: *const f32)>,
    use_program: Option<unsafe extern "C" fn(program: u32)>,
    viewport: Option<unsafe extern "C" fn(x: i32, y: i32, width: i32, height: i32)>,
    vertex_attrib_divisor: Option<unsafe extern "C" fn(index: u32, divisor: u32)>,
    vertex_attrib_pointer: Option<
        unsafe extern "C" fn(
            index: u32,
            size: i32,
            data_type: u32,
            normalized: u8,
            stride: i32,
            offset: *const (),
        ),
    >,
    vertex_attrib_ipointer: Option<
        unsafe extern "C" fn(index: u32, size: i32, data_type: u32, stride: i32, offset: *const ()),
    >,
    vertex_attrib_lpointer: Option<
        unsafe extern "C" fn(index: u32, size: i32, data_type: u32, stride: i32, offset: *const ()),
    >,
    vertex_attrib_format: Option<
        unsafe extern "C" fn(
            index: u32,
            size: i32,
            data_type: u32,
            normalized: u8,
            relative_offset: u32,
        ),
    >,
    vertex_attrib_iformat:
        Option<unsafe extern "C" fn(index: u32, size: i32, data_type: u32, relative_offset: u32)>,
}

impl Context {
    #[doc(hidden)]
    pub const NONE: Context = Context {
        active_texture: None,
        attach_shader: None,
        bind_buffer: None,
        bind_texture: None,
        bind_vertex_array: None,
        _bind_attrib_location: None,
        blend_func: None,
        blend_func_separate: None,
        buffer_data: None,
        buffer_sub_data: None,
        clear: None,
        clear_color: None,
        compile_shader: None,
        create_program: None,
        create_shader: None,
        delete_buffers: None,
        delete_program: None,
        delete_shader: None,
        delete_textures: None,
        delete_vertex_arrays: None,
        detach_shader: None,
        disable: None,
        draw_arrays: None,
        draw_arrays_instanced: None,
        _draw_elements: None,
        _draw_elements_base_vertex: None,
        enable_vertex_attrib_array: None,
        enable: None,
        gen_buffers: None,
        gen_textures: None,
        gen_vertex_arrays: None,
        get_integerv: None,
        get_programiv: None,
        get_shaderiv: None,
        _get_shader_info_log: None,
        _get_uniform_location: None,
        link_program: None,
        pixel_storei: None,
        scissor: None,
        _shader_source: None,
        _tex_image_2d: None,
        tex_parameteri: None,
        tex_parameteriv: None,
        _tex_sub_image_2d: None,
        uniform1f: None,
        uniform1i: None,
        uniform_matrix4fv: None,
        use_program: None,
        viewport: None,
        vertex_attrib_divisor: None,
        vertex_attrib_pointer: None,
        vertex_attrib_ipointer: None,
        vertex_attrib_lpointer: None,
        vertex_attrib_format: None,
        vertex_attrib_iformat: None,
    };

    /// Load GL, specifically `libGL.so.1`.
    #[inline]
    pub fn new<F>(mut get_proc_address: F) -> Self
    where
        F: FnMut(&str) -> *const (),
    {
        let mut this = Self::NONE;

        get_proc!(this.active_texture = get_proc_address("glActiveTexture"));
        get_proc!(this.attach_shader = get_proc_address("glAttachShader"));
        get_proc!(this.bind_buffer = get_proc_address("glBindBuffer"));
        get_proc!(this.bind_texture = get_proc_address("glBindTexture"));
        get_proc!(this.bind_vertex_array = get_proc_address("glBindVertexArray"));
        get_proc!(this._bind_attrib_location = get_proc_address("glBindAttribLocation"));
        get_proc!(this.blend_func = get_proc_address("glBlendFunc"));
        get_proc!(this.blend_func_separate = get_proc_address("glBlendFuncSeparate"));
        get_proc!(this.buffer_data = get_proc_address("glBufferData"));
        get_proc!(this.buffer_sub_data = get_proc_address("glBufferSubData"));
        get_proc!(this.clear = get_proc_address("glClear"));
        get_proc!(this.clear_color = get_proc_address("glClearColor"));
        get_proc!(this.compile_shader = get_proc_address("glCompileShader"));
        get_proc!(this.create_program = get_proc_address("glCreateProgram"));
        get_proc!(this.create_shader = get_proc_address("glCreateShader"));
        get_proc!(this.disable = get_proc_address("glDisable"));
        get_proc!(this.delete_buffers = get_proc_address("glDeleteBuffers"));
        get_proc!(this.delete_program = get_proc_address("glDeleteProgram"));
        get_proc!(this.delete_shader = get_proc_address("glDeleteShader"));
        get_proc!(this.delete_textures = get_proc_address("glDeleteTextures"));
        get_proc!(this.delete_vertex_arrays = get_proc_address("glDeleteVertexArrays"));
        get_proc!(this.detach_shader = get_proc_address("glDetachShader"));
        get_proc!(this.draw_arrays = get_proc_address("glDrawArrays"));
        get_proc!(this.draw_arrays_instanced = get_proc_address("glDrawArraysInstanced"));
        get_proc!(this._draw_elements = get_proc_address("glDrawElements"));
        get_proc!(this._draw_elements_base_vertex = get_proc_address("glDrawElementsBaseVertex"));
        get_proc!(this.enable = get_proc_address("glEnable"));
        get_proc!(this.enable_vertex_attrib_array = get_proc_address("glEnableVertexAttribArray"));
        get_proc!(this.gen_buffers = get_proc_address("glGenBuffers"));
        get_proc!(this.gen_textures = get_proc_address("glGenTextures"));
        get_proc!(this.gen_vertex_arrays = get_proc_address("glGenVertexArrays"));
        get_proc!(this.get_integerv = get_proc_address("glGetIntegerv"));
        get_proc!(this.get_programiv = get_proc_address("glGetProgramiv"));
        get_proc!(this.get_shaderiv = get_proc_address("glGetShaderiv"));
        get_proc!(this._get_shader_info_log = get_proc_address("glGetShaderInfoLog"));
        get_proc!(this._get_uniform_location = get_proc_address("glGetUniformLocation"));
        get_proc!(this.link_program = get_proc_address("glLinkProgram"));
        get_proc!(this.pixel_storei = get_proc_address("glPixelStorei"));
        get_proc!(this.scissor = get_proc_address("glScissor"));
        get_proc!(this._shader_source = get_proc_address("glShaderSource"));
        get_proc!(this._tex_image_2d = get_proc_address("glTexImage2D"));
        get_proc!(this.tex_parameteri = get_proc_address("glTexParameteri"));
        get_proc!(this.tex_parameteriv = get_proc_address("glTexParameteriv"));
        get_proc!(this._tex_sub_image_2d = get_proc_address("glTexSubImage2D"));
        get_proc!(this.uniform1f = get_proc_address("glUniform1f"));
        get_proc!(this.uniform1i = get_proc_address("glUniform1i"));
        get_proc!(this.uniform_matrix4fv = get_proc_address("glUniformMatrix4fv"));
        get_proc!(this.use_program = get_proc_address("glUseProgram"));
        get_proc!(this.viewport = get_proc_address("glViewport"));
        get_proc!(this.vertex_attrib_divisor = get_proc_address("glVertexAttribDivisor"));
        get_proc!(this.vertex_attrib_pointer = get_proc_address("glVertexAttribPointer"));
        get_proc!(this.vertex_attrib_ipointer = get_proc_address("glVertexAttribIPointer"));
        get_proc!(this.vertex_attrib_lpointer = get_proc_address("glVertexAttribLPointer"));
        get_proc!(this.vertex_attrib_format = get_proc_address("glVertexAttribFormat"));
        get_proc!(this.vertex_attrib_iformat = get_proc_address("glVertexAttribIFormat"));

        this
    }

    // unsafe private interfaces
    def_proc!(fn _bind_attrib_location(&self, program: u32, index: u32, name: *const u8));
    def_proc!(fn buffer_data(&self, target: u32, data_len: isize, data: *const (), usage: u32));
    def_proc!(fn buffer_sub_data(&self, target: u32, offset: isize, src_data_len: isize, src_data: *const ()));
    def_proc!(fn _draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: *const ()));
    def_proc!(fn _draw_elements_base_vertex(&self, mode: u32, count: i32, kind: u32, indices: *const (), base_vertex: i32));
    def_proc!(fn delete_buffers(&self, len: i32, buffers: *const u32));
    def_proc!(fn delete_textures(&self, len: i32, textures: *const u32));
    def_proc!(fn delete_vertex_arrays(&self, len: i32, vertex_arrays: *const u32));
    def_proc!(fn gen_buffers(&self, len: i32, buffers: *mut u32));
    def_proc!(fn gen_textures(&self, len: i32, textures: *mut u32));
    def_proc!(fn gen_vertex_arrays(&self, len: i32, vertex_arrays: *mut u32));
    def_proc!(fn get_integerv(&self, parameter: u32, data: *mut i32));
    def_proc!(fn get_programiv(&self, program: u32, name: u32, status: *mut i32));
    def_proc!(fn get_shaderiv(&self, shader: u32, name: u32, status: *mut i32));
    def_proc!(fn _get_shader_info_log(&self, shader: u32, max_length: i32, length: *mut i32, info_log: *mut u8));
    def_proc!(fn _get_uniform_location(&self, program: u32, name: *const u8) -> i32);
    def_proc!(fn pixel_storei(&self, paramater: u32, value: i32));
    def_proc!(fn _shader_source(&self, shader: u32, count: i32, string: *const *const u8, len: *const i32));
    def_proc!(fn _tex_image_2d(&self, target: u32, level: i32, internal_format: i32, width: i32, height: i32, border: i32, format: u32, ty: u32, pixels: *const ()));
    def_proc!(fn tex_parameteri(&self, target: u32, parameter: u32, value: i32));
    def_proc!(fn tex_parameteriv(&self, target: u32, parameter: u32, values: *const i32));
    def_proc!(fn _tex_sub_image_2d(&self, target: u32, level: i32, internal_format: i32, width: i32, height: i32, border: i32, format: u32, ty: u32, pixels: *const ()));
    def_proc!(fn uniform1f(&self, location: i32, data: f32));
    def_proc!(fn uniform1i(&self, location: i32, data: i32));
    def_proc!(fn uniform_matrix4fv(&self, location: i32, len: i32, transpose: u8, data: *const f32));
    def_proc!(fn vertex_attrib_pointer(&self, index: u32, size: i32, data_type: u32, normalized: u8, stride: i32, offset: *const ()));
    def_proc!(fn vertex_attrib_ipointer(&self, index: u32, size: i32, data_type: u32, stride: i32, offset: *const ()));
    def_proc!(fn vertex_attrib_lpointer(&self, index: u32, size: i32, data_type: u32, stride: i32, offset: *const ()));
    def_proc!(fn vertex_attrib_format(&self, index: u32, size: i32, data_type: u32, normalized: u8, relative_offset: u32));
    def_proc!(fn vertex_attrib_iformat(&self, index: u32, size: i32, data_type: u32, relative_offset: u32));

    // safe public interfaces
    def_proc!(pub fn active_texture(&self, unit: u32));
    def_proc!(pub fn attach_shader(&self, program: u32, shader: u32));
    def_proc!(pub fn bind_buffer(&self, target: u32, buffer: u32));
    def_proc!(pub fn bind_texture(&self, target: u32, texture: u32));
    def_proc!(pub fn bind_vertex_array(&self, vertex_array: u32));
    def_proc!(pub fn blend_func(&self, src: u32, dst: u32));
    def_proc!(pub fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32));
    def_proc!(pub fn clear(&self, mask: u32));
    def_proc!(pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32));
    def_proc!(pub fn compile_shader(&self, shader: u32));
    def_proc!(pub fn create_program(&self) -> u32);
    def_proc!(pub fn create_shader(&self, shader_type: u32) -> u32);
    def_proc!(pub fn delete_program(&self, program: u32));
    def_proc!(pub fn delete_shader(&self, shader: u32));
    def_proc!(pub fn detach_shader(&self, program: u32, shader: u32));
    def_proc!(pub fn disable(&self, parameter: u32));
    def_proc!(pub fn draw_arrays(&self, mode: u32, first: i32, count: i32));
    def_proc!(pub fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32));
    def_proc!(pub fn enable(&self, parameter: u32));
    def_proc!(pub fn enable_vertex_attrib_array(&self, index: u32));
    def_proc!(pub fn link_program(&self, program: u32));
    def_proc!(pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32));
    def_proc!(pub fn use_program(&self, program: u32));
    def_proc!(pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32));
    def_proc!(pub fn vertex_attrib_divisor(&self, index: u32, divisor: u32));

    // safe public interfaces which wrap the private unsafe interfaces
    #[inline]
    pub fn bind_attrib_location<S>(&self, program: u32, index: u32, name: S)
    where
        S: AsRef<OsStr>,
    {
        let name = match cstr_cow_from_bytes(name.as_ref().as_bytes()) {
            Some(name) => name,
            None => return,
        };

        let name = name.to_bytes_with_nul();

        self._bind_attrib_location(program, index, name.as_ptr());
    }

    #[inline]
    pub fn buffer_data_size(&self, target: u32, size: i32, usage: u32) {
        self.buffer_data(target, size as isize, ptr::null(), usage);
    }

    #[inline]
    pub fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        self.buffer_data(target, data.len() as isize, data.as_ptr().cast(), usage);
    }

    #[inline]
    pub fn buffer_sub_data_u8_slice(&self, target: u32, offset: i32, src_data: &[u8]) {
        self.buffer_sub_data(
            target,
            offset as isize,
            src_data.len() as isize,
            src_data.as_ptr().cast(),
        );
    }

    #[inline]
    pub fn create_buffer(&self) -> u32 {
        let mut buffer = 0;

        self.gen_buffers(1, &mut buffer);

        buffer
    }

    #[inline]
    pub fn create_texture(&self) -> u32 {
        let mut texture = 0;

        self.gen_textures(1, &mut texture);

        texture
    }

    #[inline]
    pub fn create_vertex_array(&self) -> u32 {
        let mut vertex_array = 0;

        self.gen_vertex_arrays(1, &mut vertex_array);

        vertex_array
    }

    #[inline]
    pub fn delete_buffer(&self, buffer: u32) {
        self.delete_buffers(1, &buffer);
    }

    #[inline]
    pub fn delete_texture(&self, texture: u32) {
        self.delete_textures(1, &texture);
    }

    #[inline]
    pub fn delete_vertex_array(&self, vertex_array: u32) {
        self.delete_vertex_arrays(1, &vertex_array);
    }

    #[inline]
    pub fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        self._draw_elements(mode, count, element_type, offset as *const ());
    }

    #[inline]
    pub fn draw_elements_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        base_vertex: i32,
    ) {
        self._draw_elements_base_vertex(mode, count, element_type, offset as *const (), base_vertex)
    }

    #[inline]
    pub fn get_parameter_i32(&self, parameter: u32) -> i32 {
        let mut value = 0;

        self.get_integerv(parameter, &mut value);

        value
    }

    #[inline]
    pub fn get_program_link_status(&self, program: u32) -> bool {
        let mut status = 0;

        self.get_programiv(program, LINK_STATUS, &mut status);

        status == 1
    }

    #[inline]
    pub fn get_shader_compile_status(&self, shader: u32) -> bool {
        let mut status = 0;

        self.get_shaderiv(shader, COMPILE_STATUS, &mut status);

        status == 1
    }

    #[inline]
    pub fn get_shader_info_log(&self, shader: u32) -> String {
        let mut length = 0;

        self.get_shaderiv(shader, INFO_LOG_LENGTH, &mut length);

        if length > 0 {
            let mut log = vec![0; length as usize];

            self._get_shader_info_log(shader, length, &mut length, log.as_mut_ptr());

            log.truncate(length as usize);

            unsafe { String::from_utf8_unchecked(log) }
        } else {
            String::new()
        }
    }

    #[inline]
    pub fn get_uniform_location<S>(&self, program: u32, name: S) -> Option<u32>
    where
        S: AsRef<OsStr>,
    {
        let name = cstr_cow_from_bytes(name.as_ref().as_bytes())?;
        let name = name.to_bytes_with_nul();

        let uniform_location = self._get_uniform_location(program, name.as_ptr());

        if uniform_location < 0 {
            None
        } else {
            Some(uniform_location as u32)
        }
    }

    #[inline]
    pub fn pixel_store_i32(&self, parameter: u32, value: i32) {
        self.pixel_storei(parameter, value);
    }

    #[inline]
    pub fn shader_source(&self, shader: u32, source: &str) {
        self._shader_source(shader, 1, &source.as_ptr(), &(source.len() as i32));
    }

    #[inline]
    pub fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        self._tex_image_2d(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            pixels
                .map(|pixels| pixels.as_ptr().cast())
                .unwrap_or(ptr::null()),
        )
    }

    #[inline]
    pub fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32) {
        self.tex_parameteri(target, parameter, value);
    }

    #[inline]
    pub fn tex_parameter_i32_slice(&self, target: u32, parameter: u32, values: &[i32]) {
        self.tex_parameteriv(target, parameter, values.as_ptr());
    }

    #[inline]
    pub fn tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        self._tex_sub_image_2d(
            target,
            level,
            x_offset,
            y_offset,
            width,
            height,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const (),
                PixelUnpackData::Slice(data) => data.as_ptr().cast(),
            },
        );
    }

    #[inline]
    pub fn uniform_1_f32(&self, location: u32, x: f32) {
        self.uniform1f(location as i32, x);
    }

    #[inline]
    pub fn uniform_1_i32(&self, location: u32, x: i32) {
        self.uniform1i(location as i32, x);
    }

    #[inline]
    pub fn uniform_matrix_4_f32_slice(&self, location: u32, transpose: bool, values: &[f32]) {
        self.uniform_matrix4fv(
            location as i32,
            values.len() as i32 / 16,
            transpose as u8,
            values.as_ptr(),
        );
    }

    #[inline]
    pub fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.vertex_attrib_pointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const (),
        );
    }

    #[inline]
    pub fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        self.vertex_attrib_ipointer(index, size, data_type, stride, offset as *const ());
    }

    #[inline]
    pub fn vertex_attrib_pointer_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        self.vertex_attrib_lpointer(index, size, data_type, stride, offset as *const ());
    }

    #[inline]
    pub fn vertex_attrib_format_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        relative_offset: u32,
    ) {
        self.vertex_attrib_format(index, size, data_type, normalized as u8, relative_offset);
    }

    #[inline]
    pub fn vertex_attrib_format_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        self.vertex_attrib_iformat(index, size, data_type, relative_offset);
    }
}
