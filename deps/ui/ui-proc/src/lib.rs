use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    __private::TokenStream2,
    Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, GenericParam, Ident, Meta, Type,
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

        impl #generics test_engine::ui::ViewInternalSetup for #name <#type_params>  {
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
