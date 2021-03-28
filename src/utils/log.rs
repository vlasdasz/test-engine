
macro_rules! get_last_method_path {
    ($path:expr) => {
        if let Some(index) = $path.rfind(":") {
            $path.chars().skip(index + 1).take($path.len() - index).collect()
        }
        else {
            $path.to_string()
        }
    }
}

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        get_last_method_path!(&name[..name.len() - 3])
    }}
}

#[macro_export]
macro_rules! format_code_location {
    ($file:expr, $func:expr, $line:expr) => {{
        use std::path::PathBuf;
        let mut file = String::from(
            PathBuf::from($file).file_name().unwrap().to_str().unwrap()
        );
        file.truncate(file.len() - 3);
        format!("[{}::{} : {}]", file, $func, $line)
    }}
}

#[macro_export]
macro_rules! code_location {
    () => {
        format_code_location!(file!(), function!(), line!())
    }
}

#[macro_export]
macro_rules! log {
    ($message:expr) => {
        println!("{} {:?}", code_location!(), $message);
    }
}

#[macro_export]
macro_rules! assert_null {
    ($prt:expr) => {
        assert_eq!($prt.is_null(), false);
    }
}