#[cfg(desktop)]
#[macro_export]
macro_rules! GLT {
    ($type:ident) => {
        gl::types::$type
    };
}

#[cfg(mobile)]
#[macro_export]
macro_rules! GLT {
    ($type:ident) => {
        $type
    };
}

#[cfg(desktop)]
#[macro_export]
macro_rules! GLC {
    ($constant:ident) => {
        gl::$constant
    };
}

#[cfg(mobile)]
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

#[cfg(desktop)]
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let gl_error = gl::GetError();
        if gl_error != gl::NO_ERROR {
            error!("{}", gl_error);
        }
    }};
}

#[cfg(mobile)]
#[macro_export]
macro_rules! check_gl_error {
    () => {{
        let gl_error = glGetError();
        if gl_error != GL_NO_ERROR {
            error!("OpenGL Error with code: {}", gl_error);
        }
    }};
}

//TODO: poll errors
#[cfg(desktop)]
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

#[cfg(mobile)]
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

#[cfg(desktop)]
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

#[cfg(mobile)]
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
