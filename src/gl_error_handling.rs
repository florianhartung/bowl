pub fn gl_clear_errors() {
    unsafe {
        while gl::GetError() != gl::NO_ERROR {}
    }
}

pub fn gl_log_errors(gl_call_file: &str, gl_call_line: u32) {
    unsafe {
        loop {
            let error_code = gl::GetError();
            if error_code == gl::NO_ERROR { break; }

            eprintln!("[Bowl:OpenGL] Error code {} caused by OpenGL call --> {}:{}", error_code, gl_call_file, gl_call_line);
            eprintln!("{:?}", backtrace::Backtrace::new());
        }
    }
}

#[macro_export]
macro_rules! gl_call {
    ( $x:expr ) => {
        {
            $crate::gl_error_handling::gl_clear_errors();
            let result = unsafe { $x };
            $crate::gl_error_handling::gl_log_errors(file!(), line!());
            result
        }
    };

    ( $( $x:expr ),+ $(,)?) => {
        ( $( $crate::gl_call!($x) ),+ ,)
    };
}