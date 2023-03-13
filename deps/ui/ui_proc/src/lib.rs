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

    let data = match &mut stream.data {
        Data::Struct(data) => data,
        _ => panic!("`view` macro has to be used with structs"),
    };

    let name = &stream.ident;

    let generics = &stream.generics;

    let type_param_names: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some(type_param.ident.clone()),
            _ => None,
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
        #[derive(Default)]
        #stream

        impl #generics ui::View for #name <#type_params> {
            fn weak_view(&self) -> ui::refs::Weak<dyn ui::View> {
                use ui::refs::ToWeak;
                (self as &dyn ui::View).weak()
            }
            fn init_views(&mut self) {
                use ui::ViewSubviews;
                #inits
            }
            fn as_any(&self) -> &dyn std::any::Any {
               self
            }
        }

        impl #generics ui::ViewInternalSetup for #name <#type_params>  {
            fn internal_setup(&mut self) {
                use ui::ViewSetup;
                use ui::refs::ToWeak;
                self.weak().setup()
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
                        self.#name = self.add_view();
                        self.#name.label = String::from(#label);
                    }
                }
            }
        }
    }

    res
}
