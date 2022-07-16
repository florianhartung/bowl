use std::ffi::CString;
use std::process::exit;

use gl::types::{GLenum, GLsizei, GLuint};

pub fn compile_shader(src: &str, t: GLenum) -> GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(t);

        gl::ShaderSource(vertex_shader, 1, &(src.as_bytes().as_ptr().cast()), &src.len().try_into().unwrap());

        gl::CompileShader(vertex_shader);

        let mut success: i32 = 0;


        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log: [i8; 1024] = [0; 1024];
            gl::GetShaderInfoLog(vertex_shader, 1024, 0 as *mut GLsizei, log.as_mut_ptr());
            let s: String = CString::from_raw(log.as_mut_ptr()).into_string().unwrap();
            println!("[Bowl] Could not compile shader of type {}:\n{}", t, s);
            exit(1);
        }

        return vertex_shader;
    }
}

pub fn create_shader_program(mut shaders: Vec<Shader>) -> GLuint {
    let shader_program;
    unsafe { shader_program = gl::CreateProgram(); }

    for shader in &mut shaders {

        let compiled_shader = compile_shader(shader.src, map_shader_type_to_glfw(&shader.r#type));
        shader.glfw_shader = Some(compiled_shader);
        unsafe { gl::AttachShader(shader_program, compiled_shader); }
    }

    unsafe {
        gl::LinkProgram(shader_program);
    }

    for shader in shaders {
        unsafe {
            let glfw_shader = shader.glfw_shader.unwrap();
            gl::DetachShader(shader_program, glfw_shader);
            gl::DeleteShader(glfw_shader);
        }
    }

    return shader_program;
}

fn map_shader_type_to_glfw(r#type: &ShaderType) -> GLenum {
    match r#type {
        ShaderType::VERTEX => gl::VERTEX_SHADER,
        ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
    }
}

pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

pub struct Shader<'a> {
    pub src: &'a str,
    pub r#type: ShaderType,
    glfw_shader: Option<GLuint>,
}

impl Shader<'static> {
    pub fn from(src: &str, r#type: ShaderType) -> Shader {
        Shader {
            src,
            r#type,
            glfw_shader: None,
        }
    }
}
