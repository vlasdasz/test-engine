
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let mut err = gl::GetError();
        while err != gl::NO_ERROR {
            println!("{} OpenGL Error with code: {}",
                 format_code_location!(file!(), function!(), line!()), err);
            err = gl::GetError();
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