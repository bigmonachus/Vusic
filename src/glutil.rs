extern mod gl;

use std::cast;
use std::io::{read_whole_file_str};
use std::ptr;
use std::str;
use std::sys;
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
    gl_id: GLuint,
}

pub struct Mesh {
    vbo: GLuint,  // Vertex buffer object
    vib: GLuint,  // Element array object
    num_vertices: u16
}

pub fn CheckGLError() {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        println!("GL error detected: {}", err);
    }
}

// program must have vec3 attribute "position".
pub fn render_meshes(program: &Program, meshes: &~[Mesh]) {
    for mesh in meshes.iter() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program.gl_id, ptr));
            gl::EnableVertexAttribArray(
                pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
            gl::BindBuffer(
                gl::ELEMENT_ARRAY_BUFFER, mesh.vib);
            gl::DrawElements(
                    gl::TRIANGLES, 3 * (mesh.num_vertices as i32), gl::UNSIGNED_SHORT, ptr::null());
        }
    }
    CheckGLError();
}

impl Mesh {
    pub fn new(num_vertices: u16, vertex_data: &~[GLfloat], indices: &~[GLushort]) -> Mesh {
        assert!((num_vertices as uint) == indices.len());
        unsafe {
            let mut vbo: GLuint = 0;
            let mut vib: GLuint = 0;

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertex_data.len() * sys::size_of::<GLfloat>()) as GLsizeiptr,
                    cast::transmute(&vertex_data[0]),
                    gl::STATIC_DRAW);
            CheckGLError();

            let indices: [GLushort, ..6] = [0, 1, 2, 3, 4, 5];

            gl::GenBuffers(1, &mut vib);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vib);
            gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * sys::size_of::<GLushort>()) as GLsizeiptr,
                    cast::transmute(&indices[0]),
                    gl::STATIC_DRAW);
            CheckGLError();
            Mesh {vbo: vbo, vib: vib, num_vertices: num_vertices}
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            println!("Deleting mesh: ({}, {})", self.vbo, self.vib);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.vib);
        }
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
                        gl::GetShaderInfoLog(
                            object, len, ptr::mut_null(), (vec::raw::to_mut_ptr(buf) as *mut GLbyte));
                        let infolog = str::raw::from_utf8(buf);
                        if infolog.char_len() != 0 {
                            println("==== Shader compilation failed:");
                            println!("Shader path: {}", path);
                            fail!(infolog);
                        }
                    }
                }
                Shader { gl_id: object }

            },
            Err(_) => {
                fail!(fmt!("Could not read path: %s", path));
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        println!("Deleting shader {}", self.gl_id);
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
                let mut buf = vec::from_elem((len as uint) - 1, 0u8);
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
        Program { gl_id: object }
    }

    pub fn enable(&self) {
        gl::UseProgram(self.gl_id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        println!("Deleting program: {}", self.gl_id);
        gl::DeleteProgram(self.gl_id);
    }
}
