use tools::New;

pub struct TestModel {
    pub data: u32,
}

impl New for TestModel {
    fn new() -> Self {
        TestModel { data: 0 }
    }
}
