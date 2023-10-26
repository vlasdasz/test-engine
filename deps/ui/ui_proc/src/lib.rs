use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
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

    let Fields::Named(fields) = &mut data.fields else {
        panic!("No named fields");
    };

    let inits = add_inits(name, fields);
    let links = add_links(fields);

    fields
        .named
        .push(Field::parse_named.parse2(quote! { view: ui::ViewBase }).unwrap());

    quote! {
        #[derive(derivative::Derivative, Default)]
        #[derivative(Debug)]
        #stream

        impl #generics ui::View for #name <#type_params> {
            fn weak_view(&self) -> ui::refs::Weak<dyn ui::View> {
                ui::refs::weak_from_ref(self as &dyn ui::View)
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
                ui::refs::weak_from_ref(self).__link();
                ui::refs::weak_from_ref(self).setup();
            }
        }

        impl #generics  #name <#type_params> {
            fn __link(self: ui::refs::Weak<Self>) {
                #links
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

fn add_inits(root_name: &Ident, fields: &mut FieldsNamed) -> TokenStream2 {
    let subview = Ident::new("SubView", Span::call_site());

    let mut res = quote!();

    for field in &mut fields.named {
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

fn add_links(fields: &mut FieldsNamed) -> TokenStream2 {
    let mut res = quote!();

    for field in &mut fields.named {
        let field_name = field.ident.as_ref().unwrap();

        let attrs = field.attrs.clone();

        let attr = attrs.first();
        field.attrs = vec![];

        let Some(attr) = attr else {
            continue;
        };

        let attribute_name = attr.path.to_token_stream().to_string();
        let tokens = attr.tokens.to_token_stream().to_string();
        let method = tokens.strip_prefix("= ").unwrap();

        let method = Ident::new(method, Span::call_site());

        match attribute_name.as_str() {
            "link" => {
                res = quote! {
                    #res
                    {
                        use ui_views::AlertErr;
                        self.#field_name.on_tap.sub(move || self.#method().alert_err());
                    }
                };
            }
            "link_async" => {
                res = quote! {
                    #res
                    self.#field_name.on_tap.sub(move || {
                        tokio::spawn(async move {
                            use ui_views::AlertErr;
                            self.#method().await.alert_err();
                        });
                    });
                };
            }
            _ => panic!("Invalid attribute. Only `link` and 'link_async' are supported."),
        }
    }

    res
}
