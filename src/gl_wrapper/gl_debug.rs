
#[cfg(not(target_os="ios"))]
#[macro_export]
macro_rules! GLT {
    ($type:ident) => {
        gl::types::$type
    }
}

#[cfg(target_os="ios")]
#[macro_export]
macro_rules! GLT {
    ($type:ident) => {
        $type
    }
}

#[cfg(not(target_os="ios"))]
#[macro_export]
macro_rules! GLC {
    ($constant:ident) => {
        gl::$constant
    }
}

#[cfg(target_os="ios")]
#[macro_export]
macro_rules! GLC {
    ($constant:ident) => {
        concat_idents!(GL_, $constant)
    }
}

#[cfg(not(target_os="ios"))]
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

#[cfg(target_os="ios")]
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let err = glGetError();
        if err != GL_NO_ERROR {
            println!("{} OpenGL Error with code: {}",
                 format_code_location!(file!(), function!(), line!()), err);
        }
    }}
}

#[cfg(not(target_os="ios"))]
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

#[cfg(target_os="ios")]
#[macro_export]
macro_rules! GL {
    ($call:ident) => {
        unsafe {
            let function = concat_idents!(gl, $call);
            let ret = function();
            check_gl_error!();
            ret
        }
    };
    ($call:ident, $($args:expr), *) => {
        unsafe {
            let function = concat_idents!(gl, $call);
            let ret = function($($args,)*);
            check_gl_error!();
            ret
        }
    };
}

// let function = concat_idents!(gl, $call);
// let ret = function($($args,)*);