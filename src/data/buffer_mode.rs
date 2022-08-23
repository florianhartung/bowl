use gl::types::GLenum;

pub trait BufferMode {
    fn to_gl_enum(&self) -> GLenum;
}

pub enum DrawMode {
    STREAM,
    STATIC,
    DYNAMIC,
}

impl BufferMode for DrawMode {
    fn to_gl_enum(&self) -> GLenum {
        match self {
            DrawMode::STREAM => gl::STREAM_DRAW,
            DrawMode::STATIC => gl::STATIC_DRAW,
            DrawMode::DYNAMIC => gl::DYNAMIC_DRAW,
        }
    }
}
