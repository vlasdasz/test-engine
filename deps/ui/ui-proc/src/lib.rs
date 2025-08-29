use std::str::FromStr;

use proc_macro::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    __private::TokenStream2,
    Attribute, Data, DeriveInput, Expr, Field, Fields, FieldsNamed, GenericParam, Ident, Meta, Token, Type,
    parse::Parser,
    parse_macro_input, parse_quote,
    spanned::Spanned,
    token::{Bracket, Pound},
};

#[proc_macro_attribute]
#[allow(clippy::too_many_lines)]
pub fn view(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        panic!("`view` macro has to be used with structs")
    };

    let name = &stream.ident;

    let name_str =
        TokenStream2::from_str(&format!("\"{name}\"")).expect("Failed to extract view struct name");

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

    quote! {
        #[derive(test_engine::educe::Educe)]
        #[educe(Default)]
        #stream

        impl #generics test_engine::ui::View for #name <#type_params> {
            fn weak_view(&self) -> test_engine::refs::Weak<dyn test_engine::ui::View> {
                test_engine::refs::weak_from_ref(self as &dyn test_engine::ui::View)
            }
            fn base_view(&self) -> &test_engine::ui::ViewBase {
                &self.__view_base
            }
            fn base_view_mut(&mut self) -> &mut test_engine::ui::ViewBase {
                &mut self.__view_base
            }
            fn init_views(&mut self) {
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
                // self.__after_setup_event().trigger(());
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
            self.#name.base_view_mut().view_label = format!("{}: {}", #label, self.#name.base_view().view_label);
        }
    }

    res
}

#[proc_macro]
pub fn launch_app(_input: TokenStream) -> TokenStream {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let use_statement = crate_name.replace('-', "_");

    let crate_ident = Ident::new(&use_statement, Span::call_site().into());

    let output = quote! {
        pub use #crate_ident::test_engine_create_app;
        test_engine::test_engine_start_app();
    };

    output.into()
}

#[proc_macro]
pub fn async_link_button(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as ButtonArgs);

    let button_path = args.button;
    let method = args.method;

    let expanded = if let Some(arg) = args.arg {
        quote! {
            #button_path.on_tap(move || {
                tokio::spawn(async move {
                    use test_engine::ui::AlertErr;
                    self.#method(#arg).await.alert_err();
                });
            });
        }
    } else {
        quote! {
            #button_path.on_tap(move || {
                tokio::spawn(async move {
                    use test_engine::ui::AlertErr;
                    self.#method().await.alert_err();
                });
            });
        }
    };

    TokenStream::from(expanded)
}

use syn::parse::{Parse, ParseStream, Result};

struct ButtonArgs {
    button: Expr,
    method: Ident,
    arg:    Option<Expr>,
}

impl Parse for ButtonArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let button: Expr = input.parse()?;
        let _comma1: Token![,] = input.parse()?;
        let method: Ident = input.parse()?;

        let arg = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ButtonArgs { button, method, arg })
    }
}

#[proc_macro]
pub fn cast_cell(input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input as Ident);

    let expanded = quote! {
        cell.downcast_mut::<#ty>().unwrap()
    };

    TokenStream::from(expanded)
}
