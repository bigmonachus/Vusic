extern mod gl;

use std::io::{read_whole_file_str};
use std::ptr;
use std::str;
use std::vec;

use gl::types::*;

pub enum ShaderClass {
    Vertex,
    Fragment,
}

pub struct Program {
    gl_id: GLuint,
}

pub struct Shader {
    path: ~str,
    shader_type: ShaderClass,
    shader_obj: u32,
    gl_id: GLuint,
}

pub fn CheckGLError() {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        println(fmt!("GL error detected: %?", err));
    }
}

impl Shader {
    pub fn new (path: &str, st: ShaderClass) -> Shader {
        // Return this on error.
        //let dummy_shader = Shader { path: "", shader_type: Vertex, shader_obj: 0, gl_id: 0 };

        // Load the source.
        match read_whole_file_str(&Path(path)) {
            Ok(src) => {
                let gl_shader_type =
                    match st {
                        Vertex => gl::VERTEX_SHADER,
                        Fragment => gl::FRAGMENT_SHADER,
                    };
                let object = gl::CreateShader(gl_shader_type);
                unsafe {
                    src.with_c_str(|ptr| {gl::ShaderSource(object, 1, &ptr, ptr::null())}) ;
                    gl::CompileShader(object);
                    let mut status = gl::TRUE as GLint;
                    gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut status);
                    if status != (gl::TRUE as GLint) {
                        let mut len = 0;
                        gl::GetShaderiv(object, gl::INFO_LOG_LENGTH, &mut len);
                        let mut buf = vec::from_elem((len as uint) - 1, 0u8);
                        gl::GetShaderInfoLog(object, len, ptr::mut_null(), (vec::raw::to_mut_ptr(buf) as *mut GLbyte));
                        let infolog = str::raw::from_utf8(buf);
                        if infolog.char_len() != 0 {
                            println("==== Shader compilation failed:");
                            println(fmt!("Shader path: %s", path));
                            fail!(infolog);
                        }
                    }
                }
                return Shader { path: path.to_owned(), shader_type: st, shader_obj: object, gl_id: object };

            },
            Err(_) => {
                fail!(fmt!("Could not read path: %s", path));
                //dummy_shader;
            }
        }
        // Create a new shader.
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        println(fmt!("Deleting shader %?", self.gl_id));
        gl::DeleteShader(self.gl_id);
    }
}

impl Program {
    pub fn new (shaders: &[Shader]) -> Program {
        let object = gl::CreateProgram();
        for shader in shaders.iter() {
            gl::AttachShader(object, shader.gl_id);
        }
        gl::LinkProgram(object);
        unsafe {
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(object, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(object, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec::from_elem(len as uint - 1, 0u8); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(object, len, ptr::mut_null(), vec::raw::to_mut_ptr(buf) as *mut GLchar);
                let infolog = str::raw::from_utf8(buf);
                if infolog.char_len() != 0 {
                    println("==== Program linking failed:");
                    fail!(infolog);
                }
            }
        }
        if gl::IsProgram(object) == 0 {
            fail!("Failed to create program.");
        }
        return Program { gl_id: object };
    }

    pub fn enable(&self) {
        gl::UseProgram(self.gl_id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        println(fmt!("Deleting program: %?", self.gl_id));
        gl::DeleteProgram(self.gl_id);
    }
}
