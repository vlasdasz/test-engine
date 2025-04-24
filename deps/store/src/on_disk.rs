use std::{
    fmt::{Debug, Formatter},
    fs,
    fs::remove_file,
    marker::PhantomData,
};

use crate::{Paths, storable::Storable};

fn set_value<T: serde::ser::Serialize>(value: T, key: &str) {
    let json = serde_json::to_string_pretty(&value).expect("Failed to serialize data");
    let dir = Paths::storage();
    _ = fs::create_dir_all(&dir);
    fs::write(dir.join(key), json).expect("Failed to write to file");
}

fn get_value<T: Storable>(key: &str) -> Option<T> {
    let dir = Paths::storage();
    let path = dir.join(key);

    if !path.exists() {
        return None;
    }

    let json = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json).expect("Failet to parse json")
}

fn get_or_init_value<T: Storable + Default>(key: &str) -> T {
    let dir = Paths::storage();
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
        set_value(val, self.name);
    }

    pub fn get(&self) -> Option<T> {
        get_value(self.name)
    }

    pub fn reset(&self) {
        let dir = Paths::storage();
        let path = dir.join(self.name);

        remove_file(path).expect("Failed to remove file");
    }
}

impl<T: Storable + Default> OnDisk<T> {
    pub fn get_or_init(&self) -> T {
        get_or_init_value(self.name)
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

    use crate::{OnDisk, Paths};

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
        assert_eq!(STORED.get(), None);

        for _ in 0..10 {
            let rand: i32 = Faker.fake();

            spawn(async move {
                STORED.set(rand);
            })
            .await?;

            spawn(async move {
                assert_eq!(STORED.get(), Some(rand));
                assert_eq!(format!("Some({rand})"), format!("{STORED:?}"));
            })
            .await?;
        }

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
