
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            println!("{} OpenGL Error with code: {}",
                 format_code_location!(file!(), function!(), line!()), err);
        }
    }}
}

#[macro_export]
macro_rules! GL {
    ($call:ident) => {
        unsafe {
            let ret = gl::$call();
            check_gl_error!();
            ret
        }
    };
    ($call:ident, $($args:expr), *) => {
        unsafe {
            let ret = gl::$call($($args,)*);
            check_gl_error!();
            ret
        }
    };
}