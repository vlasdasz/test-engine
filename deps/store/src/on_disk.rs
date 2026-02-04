use std::{
    fmt::{Debug, Formatter},
    fs::{self, remove_file},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use filesystem::Paths;

use crate::storable::Storable;

fn set_value<T: serde::ser::Serialize>(value: T, path: &Path) {
    let json = serde_json::to_string_pretty(&value).expect("Failed to serialize data");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(dbg!(path), json).expect("Failed to write to file");
}

fn get_value<T: Storable>(path: &Path) -> Option<T> {
    if !path.exists() {
        return None;
    }

    let json = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json).expect("Failet to parse json")
}

fn get_or_init_value<T: Storable + Default>(path: &Path) -> T {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    if !path.exists() {
        let new = T::default();
        set_value(&new, path);
        return new;
    }
    let json = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json).expect("Failet to parse json")
}

pub struct OnDisk<T: Storable> {
    path: &'static str,
    _p:   PhantomData<T>,
}

impl<T: Storable> OnDisk<T> {
    pub const fn new(path: &'static str) -> Self {
        Self {
            path,
            _p: PhantomData,
        }
    }

    pub fn set(&self, val: impl Into<T>) {
        let val = val.into();
        set_value(val, &expand_tilde(self.path));
    }

    pub fn get(&self) -> Option<T> {
        get_value(&expand_tilde(self.path))
    }

    pub fn reset(&self) {
        remove_file(expand_tilde(self.path)).expect("Failed to remove file");
    }
}

impl<T: Storable + Default> OnDisk<T> {
    pub fn get_or_init(&self) -> T {
        get_or_init_value(&expand_tilde(self.path))
    }
}

impl<T: Storable + Debug> Debug for OnDisk<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let p = path.as_ref();
    if !p.starts_with("~") {
        return p.to_path_buf();
    }

    if p == Path::new("~") {
        Paths::home()
    } else {
        Paths::home().join(p.strip_prefix("~").unwrap())
    }
}

#[cfg(test)]
mod test {

    use anyhow::Result;
    use filesystem::Paths;
    use serde::{Deserialize, Serialize};

    use crate::OnDisk;

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
    struct Data {
        number: i32,
        string: String,
    }

    static STORED: OnDisk<i32> = OnDisk::new("~/.test/stored_i32_test.json");
    static STORED_STRUCT: OnDisk<Data> = OnDisk::new("~/.test/stored_struct_test.json");

    fn check_send<T: Send>(_send: &T) {}
    fn check_sync<T: Sync>(_sync: &T) {}

    #[test]
    fn stored() -> Result<()> {
        check_send(&STORED);
        check_sync(&STORED);
        check_send(&STORED_STRUCT);
        check_sync(&STORED_STRUCT);

        STORED.set(10);
        STORED.reset();
        assert_eq!(STORED.get(), None);

        let data = Data {
            number: 555,
            string: "Helloyyyy".to_string(),
        };

        STORED_STRUCT.set(data.clone());

        let loaded_data = STORED_STRUCT.get();

        assert_eq!(data, loaded_data.unwrap());

        Ok(())
    }

    #[test]
    fn paths() {
        assert!(Paths::executable_name().starts_with("store"));
    }
}
