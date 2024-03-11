use std::{
    fmt::{Debug, Formatter},
    fs,
    marker::PhantomData,
    path::PathBuf,
};

use gm::Platform;

use crate::storable::Storable;

pub fn executable_name() -> String {
    std::env::current_exe()
        .expect("Failed to get std::env::current_exe()")
        .file_name()
        .expect("Failed to get executable name")
        .to_string_lossy()
        .into()
}

pub(crate) fn storage_dir() -> PathBuf {
    let home = if Platform::MOBILE {
        dirs::document_dir()
    } else {
        dirs::home_dir()
    }
    .expect("Failed to get home directory");

    format!("{}/.{}", home.display(), executable_name()).into()
}

fn set_value<T: serde::ser::Serialize>(value: T, key: &str) {
    let json = serde_json::to_string_pretty(&value).expect("Failed to serialize data");
    let dir = storage_dir();
    _ = fs::create_dir_all(&dir);
    fs::write(dir.join(key), json).expect("Failed to write to file");
}

fn get_value<T: Storable>(key: &str) -> T {
    let dir = storage_dir();
    let path = dir.join(key);

    fs::create_dir_all(&dir).unwrap();

    if !path.exists() {
        let new = T::default();
        set_value(&new, key);
        return new;
    }
    let json = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json).expect("Failet to parse json")
}

pub struct OnDisk<T: Storable> {
    name: &'static str,
    _p:   PhantomData<T>,
}

impl<T: Storable> OnDisk<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _p: PhantomData,
        }
    }

    pub fn set(&self, val: impl Into<T>) {
        let val = val.into();
        set_value(val, self.name)
    }

    pub fn get(&self) -> T {
        get_value(self.name)
    }

    pub fn reset(&self) {
        self.set(T::default())
    }
}

impl<T: Storable + Debug> Debug for OnDisk<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod test {

    use anyhow::Result;
    use fake::{Fake, Faker};
    use serde::{Deserialize, Serialize};
    use tokio::spawn;

    use crate::{on_disk::executable_name, OnDisk};

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
    struct Data {
        number: i32,
        string: String,
    }

    static STORED: OnDisk<i32> = OnDisk::new("stored_i32_test");
    static STORED_STRUCT: OnDisk<Data> = OnDisk::new("stored_struct_test");

    fn check_send<T: Send>(_send: &T) {}
    fn check_sync<T: Sync>(_sync: &T) {}

    #[tokio::test]
    async fn stored() -> Result<()> {
        check_send(&STORED);
        check_sync(&STORED);
        check_send(&STORED_STRUCT);
        check_sync(&STORED_STRUCT);

        STORED.set(10);
        STORED.reset();
        assert_eq!(STORED.get(), i32::default());

        for _ in 0..10 {
            let rand: i32 = Faker.fake();

            spawn(async move {
                STORED.set(rand);
            })
            .await?;

            spawn(async move {
                assert_eq!(STORED.get(), rand);
                assert_eq!(format!("{rand}"), format!("{STORED:?}"));
            })
            .await?;
        }

        let data = Data {
            number: 555,
            string: "Helloyyyy".to_string(),
        };

        STORED_STRUCT.set(data.clone());

        let loaded_data = STORED_STRUCT.get();

        assert_eq!(data, loaded_data);

        Ok(())
    }

    #[test]
    fn paths() {
        assert!(executable_name().starts_with("store"));
    }
}
