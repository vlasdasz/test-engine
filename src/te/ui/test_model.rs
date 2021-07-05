use tools::HasNew;

pub struct TestModel {
    pub data: u32
}

impl HasNew for TestModel {
    fn new() -> Self {
        TestModel { data: 0 }
    }
}