

#[macro_export]
macro_rules! _file_name {
    ($file:expr) => {{
            use std::path::PathBuf;
            let mut str = String::from(
                PathBuf::from(&file!()).file_name().unwrap().to_str().unwrap()
            );
            str.truncate(str.len() - 3);
            str
    }}
}

#[macro_export]
macro_rules! log {
    ($message:expr) => {
        {
            println!("[{} - {}] {:#?}", _file_name!(file!()), line!(), $message);
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
        &name[..name.len() - 3]
    }}
}
