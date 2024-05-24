use anyhow::Result;
use test_engine::{
    reflected,
    reflected::Reflected,
    refs::Weak,
    ui::{view, Button, FormView, Sub, ViewData, ViewSetup, ViewSubviews, UI},
    ui_test::record_ui_test,
};

#[derive(Default, Debug, Reflected)]
struct Data {
    float:   f32,
    integer: u32,
    boolean: bool,
    string:  String,
}

#[view]
struct FormTestView {
    button: Sub<Button>,
    form:   Sub<FormView<Data>>,
}

impl ViewSetup for FormTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Save").place().lrt(0).h(100);
        self.button.on_tap(move || {
            let mut data = Data::default();
            self.form.get_data(&mut data);
            dbg!(&data);
        });

        self.form.place().lrb(0).t(100);
        self.form.set_data(&Data {
            float:   10.0,
            integer: 20,
            boolean: true,
            string:  "hello".to_string(),
        });
    }
}

pub async fn test_form_view() -> Result<()> {
    let view = UI::init_test_view::<FormTestView>().await;

    dbg!(view.form.dump_subviews());

    record_ui_test().await;

    Ok(())
}
