use std::str::FromStr;

use parking_lot::Mutex;
use proc_macro::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    __private::TokenStream2,
    Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, GenericParam, Ident, Meta, Type,
    parse::Parser,
    parse_macro_input, parse_quote,
    spanned::Spanned,
    token::{Bracket, Pound},
};

pub(crate) static VIEWS: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub(crate) static VIEW_TESTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[allow(clippy::too_many_lines)]
pub fn view_impl(stream: TokenStream, test: bool) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        panic!("`view` macro has to be used with structs")
    };

    let name = &stream.ident;

    let name_str =
        TokenStream2::from_str(&format!("\"{name}\"")).expect("Failed to extract view struct name");

    VIEWS.lock().push(name.to_string());

    if test {
        VIEW_TESTS.lock().push(format!("{} {:#?}", name, Span::call_site().file()));
    }

    let generics = &stream.generics;

    let type_param_names: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some(type_param.ident.clone()),
            GenericParam::Const(const_param) => Some(const_param.ident.clone()),
            GenericParam::Lifetime(_) => None,
        })
        .collect();

    let type_params = quote_spanned! {stream.generics.span()=>
        #(#type_param_names),*
    };

    let Fields::Named(fields) = &mut data.fields else {
        panic!("No named fields");
    };

    let inits = add_inits(name, fields);

    fields.named.insert(
        0,
        Field::parse_named
            .parse2(quote! { __view_base: test_engine::ui::ViewBase })
            .expect("parse2(quote! { __view_base: test_engine::ui::ViewBase })"),
    );

    let ui_test_related_stuff = if test {
        quote! {
            #[test_engine::__internal_macro_deps::ctor::ctor(crate_path = test_engine::__internal_macro_deps::ctor)]
            fn store_test() {
                use futures::FutureExt;

                crate::UI_TESTS
                    .lock()
                    .insert(#name_str.to_string(), || run_ui_test().boxed());
            }

            #[test]
            fn ui_test() -> anyhow::Result<()> {
                let mut child = std::process::Command::new("cargo")
                    .args([
                        "run",
                        "-p",
                        "ui-test",
                        "--target-dir",
                        "../target/ui_tests",
                        "--",
                        "--test-name",
                        #name_str,
                    ])
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .spawn()?;

                let status = child.wait()?;

                if !status.success() {
                    std::process::exit(status.code().unwrap_or(1));
                }

                Ok(())
            }

            #[allow(clippy::unused_async)]
            pub async fn run_ui_test() -> Result<()> {
                #name::perform_test(test_engine::ui_test::UITest::start::<#name>())
            }
        }
    } else {
        quote!()
    };

    quote! {


        #[derive(test_engine::educe::Educe)]
        #[educe(Default)]
        #stream

        impl #generics test_engine::ui::View for #name <#type_params> {
            fn weak_view(&self) -> test_engine::refs::Weak<dyn test_engine::ui::View> {
                test_engine::refs::weak_from_ref(self as &dyn test_engine::ui::View)
            }
            fn __base_view(&self) -> &mut test_engine::ui::ViewBase {
                #![allow(clippy::transmute_ptr_to_ptr)]
                unsafe { std::mem::transmute(&self.__view_base) }
            }
            fn __init_views(&mut self) {
                use test_engine::ui::ViewSubviews;
                #inits
            }

        }

        impl #generics test_engine::refs::AsAny for #name <#type_params> {
            fn as_any(&self) -> &dyn std::any::Any {
               self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
               self
            }
        }

        impl #generics test_engine::ui::__ViewInternalSetup for #name <#type_params>  {
            fn __internal_before_setup(&mut self) {
                use test_engine::ui::Setup;
                let mut weak = test_engine::refs::weak_from_ref(self);
                weak.before_setup();
            }

            fn __internal_setup(&mut self) {
                use test_engine::ui::Setup;
                use test_engine::ui::WithHeader;
                use test_engine::ui::ViewData;
                self.__view_base.view_label += &#name_str.to_string();
                self.layout_header();
                let mut weak = test_engine::refs::weak_from_ref(self);
                weak.setup();
                self.__after_setup_event().trigger(());
            }

            fn __internal_inspect(&mut self) {
                use test_engine::ui::Setup;
                let mut weak = test_engine::refs::weak_from_ref(self);
                weak.inspect();
            }

            fn __internal_on_selection_changed(&mut self, selected: bool) {
                use test_engine::ui::Setup;
                let mut weak = test_engine::refs::weak_from_ref(self);
                weak.on_selection_changed(selected);
            }
        }

        impl #generics test_engine::ui::__ViewInternalTableData for #name <#type_params>  {
            fn __cell_height(&self) -> f32 {
                use test_engine::ui::TableData;
                let weak = test_engine::refs::weak_from_ref(self);
                weak.cell_height()
            }
            fn __number_of_cells(&self) -> usize {
                use test_engine::ui::TableData;
                let weak = test_engine::refs::weak_from_ref(self);
                weak.number_of_cells()
            }
            fn __make_cell(&self, index: usize) -> test_engine::refs::Own<dyn test_engine::ui::View> {
                use test_engine::ui::TableData;
                let weak = test_engine::refs::weak_from_ref(self);
                weak.make_cell(index)
            }
            fn __setup_cell(&self, cell: &mut dyn std::any::Any, index: usize) {
                use test_engine::ui::TableData;
                let weak = test_engine::refs::weak_from_ref(self);
                weak.setup_cell(cell, index)
            }
            fn __cell_selected(&mut self, index: usize) {
                use test_engine::ui::TableData;
                let weak = test_engine::refs::weak_from_ref(self);
                weak.cell_selected(index)
            }
        }

        #ui_test_related_stuff

    }
    .into()
}

fn add_inits(root_name: &Ident, fields: &mut FieldsNamed) -> TokenStream2 {
    let mut res = quote!();

    let init_attr = Attribute {
        pound_token:   Pound::default(),
        style:         syn::AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta:          Meta::Path(parse_quote!(init)),
    };

    let mut inits_started = false;

    for field in &mut fields.named {
        if let Some(idx) = field.attrs.iter().position(|a| *a == init_attr) {
            field.attrs.remove(idx);
            inits_started = true;
        }

        if !inits_started {
            continue;
        }

        let name = field.ident.as_ref().expect("let name = field.ident.as_ref()");

        let ty = &field.ty;

        let weak_wrapped_type = Type::without_plus
            .parse2(quote! { test_engine::refs::Weak<#ty> })
            .expect("Type::without_plus.parse2(quote! { Weak<#ty> })");

        field.ty = weak_wrapped_type;

        let label = TokenStream2::from_str(&format!("\"{root_name}.{name}\""))
            .expect("let label = TokenStream2::from_str()");

        res = quote! {
            #res
            self.#name = self.add_view();
            self.#name.__base_view().view_label = format!("{}: {}", #label, self.#name.__base_view().view_label);
        }
    }

    res
}
