#![allow(dead_code)]

use ui::view;

#[view]
struct TestTemplateView<T: 'static, U: Send + Sync + 'static> {
    data: T,
    dete: U,
}

// /Users/vladas/money/money/test_engine/deps/ui/ui_views
