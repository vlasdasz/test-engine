use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::Parser,
    parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, GenericParam, Ident, Type,
    __private::{Span, TokenStream2},
    spanned::Spanned,
};

#[proc_macro_attribute]
pub fn view(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        panic!("`view` macro has to be used with structs")
    };

    let name = &stream.ident;

    let name_str = TokenStream2::from_str(&format!("\"{name}\"")).unwrap();

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

    let mut inits = quote!();

    if let Fields::Named(fields) = &mut data.fields {
        inits = add_inits(name, fields);

        fields
            .named
            .push(Field::parse_named.parse2(quote! { view: ui::ViewBase }).unwrap());
    }

    quote! {
        #[derive(derivative::Derivative, Default)]
        #[derivative(Debug)]
        #stream

        impl #generics ui::View for #name <#type_params> {
            fn weak_view(&self) -> ui::refs::Weak<dyn ui::View> {
                (self as &dyn ui::View).weak()
            }
            fn base(&self) -> &ui::ViewBase {
                &self.view
            }
            fn init_views(&mut self) {
                use ui::ViewSubviews;
                #inits
            }
            fn as_any(&self) -> &dyn std::any::Any {
               self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
               self
            }
        }

        impl #generics ui::ViewInternalSetup for #name <#type_params>  {
            fn __internal_setup(&mut self) {
                use ui::ViewSetup;
                use ui::WithHeader;
                self.view.label = #name_str.to_string();
                self.layout_header();
                self.weak().setup();
            }
        }

        impl #generics std::ops::Deref for #name <#type_params> {
            type Target = ui::ViewBase;
            fn deref(&self) -> &ui::ViewBase {
                &self.view
            }
        }
        impl #generics std::ops::DerefMut for #name <#type_params>  {
            fn deref_mut(&mut self) -> &mut ui::ViewBase {
                &mut self.view
            }
        }
    }
    .into()
}

fn add_inits(root_name: &Ident, fields: &FieldsNamed) -> TokenStream2 {
    let subview = Ident::new("SubView", Span::call_site());

    let mut res = quote!();

    for field in &fields.named {
        let name = field.ident.as_ref().unwrap();

        if let Type::Path(path) = &field.ty {
            for segment in &path.path.segments {
                if segment.ident == subview {
                    let label = TokenStream2::from_str(&format!("\"{root_name}.{name}\"")).unwrap();

                    res = quote! {
                        #res
                        self.#name = self.__internal_add_view();
                        self.#name.label += #label;
                    }
                }
            }
        }
    }

    res
}
