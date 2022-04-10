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
        let gl_error = gl::GetError();
        if gl_error != gl::NO_ERROR {
            dbg!(gl_error);
        }
    }};
}

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let gl_error = glGetError();
        if gl_error != GL_NO_ERROR {
            error!("OpenGL Error with code: {}", gl_error);
            dbg!(gl_error);
        }
    }};
}

//TODO: poll errors
#[cfg(not(any(target_os = "ios", target_os = "android")))]
#[macro_export]
macro_rules! GL_SILENT {
    ($call:ident) => {
        unsafe {
            gl::$call()
        }
    };
    ($call:ident, $($args:expr), *) => {
        unsafe {
            gl::$call($($args,)*)
        }
    };
}

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_export]
macro_rules! GL_SILENT {
    ($call:ident) => {
        unsafe {
            mashup! {
                gl["GL"] = gl $call;
            }
            let function = gl! {
                "GL"
            };
            function()
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
            function($($args,)*)
        }
    };
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
