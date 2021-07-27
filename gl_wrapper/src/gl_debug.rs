#[cfg(not(any(target_os = "ios", target_os = "android")))]
#[macro_export]
macro_rules! GLT {
    ($type:ident) => {
        gl::types::$type
    };
}

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_export]
macro_rules! GLT {
    ($type:ident) => {
        $type
    };
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
#[macro_export]
macro_rules! GLC {
    ($constant:ident) => {
        gl::$constant
    };
}

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_export]
macro_rules! GLC {
    ($constant:ident) => {{
        mashup! {
            glc["GLC"] = GL_ $constant;
        }
        glc! {
            "GLC"
        }
    }};
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            println!(
                "{} OpenGL Error with code: {}",
                format_code_location!(file!(), function!(), line!()),
                err
            );
        }
    }};
}

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let err = glGetError();
        if err != GL_NO_ERROR {
            println!(
                "{} OpenGL Error with code: {}",
                format_code_location!(file!(), function!(), line!()),
                err
            );
        }
    }};
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
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

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_export]
macro_rules! GL {
    ($call:ident) => {
        unsafe {
            mashup! {
                gl["GL"] = gl $call;
            }
            let function = gl! {
                "GL"
            };
            let ret = function();
            check_gl_error!();
            ret
        }
    };
    ($call:ident, $($args:expr), *) => {
        unsafe {
            mashup! {
                gl2["GL2"] = gl $call;
            }
            let function = gl2! {
                "GL2"
            };
            let ret = function($($args,)*);
            check_gl_error!();
            ret
        }
    };
}
