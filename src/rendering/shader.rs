extern crate gl;

use gl::types::*;
use std::{ffi::CString, fs::File, io::Read, path::Path, ptr, str};

/// Represents an OpenGL shader program.
pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    /// Creates a new `Shader` from vertex and fragment shader source files.
    ///
    /// # Arguments
    ///
    /// * `vertex_path` - The path to the vertex shader source file.
    /// * `fragment_path` - The path to the fragment shader source file.
    ///
    /// # Returns
    ///
    /// A new `Shader` instance.
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_code = Shader::read_shader_source(vertex_path);
        let fragment_code = Shader::read_shader_source(fragment_path);

        let vertex_shader = Shader::compile_shader(&vertex_code, gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(&fragment_code, gl::FRAGMENT_SHADER);

        let shader_program = Shader::link_program(vertex_shader, fragment_shader);

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id: shader_program }
    }

    fn read_shader_source(path: &str) -> String {
        let path = Path::new(path);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => s,
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        }
    }

    /// Compiles a shader from source code.
    ///
    /// # Arguments
    ///
    /// * `source` - The shader source code.
    /// * `shader_type` - The type of shader (e.g., `gl::VERTEX_SHADER`).
    ///
    /// # Returns
    ///
    /// The compiled shader ID.
    fn compile_shader(src: &str, ty: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(ty);
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
                );
            }
            shader
        }
    }

    /// Links vertex and fragment shaders into a shader program.
    ///
    /// # Arguments
    ///
    /// * `vertex_shader` - The compiled vertex shader ID.
    /// * `fragment_shader` - The compiled fragment shader ID.
    ///
    /// # Returns
    ///
    /// The linked shader program ID.
    fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
                );
            }
            program
        }
    }

    /// Activates the shader program.
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
