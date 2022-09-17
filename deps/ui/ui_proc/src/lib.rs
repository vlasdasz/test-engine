use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser,
    parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Type,
    __private::{Span, TokenStream2},
};

#[proc_macro_attribute]
pub fn view(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let data = match &mut stream.data {
        Data::Struct(data) => data,
        _ => panic!("`view` macro has to be used with structs"),
    };

    let mut inits = quote!();

    if let Fields::Named(fields) = &mut data.fields {
        inits = add_inits(&fields);

        fields.named.push(Field::parse_named.parse2(quote! { view: ViewBase }).unwrap());
    }

    let name = &stream.ident;

    quote! {
        #stream
        impl View for #name {
            fn rglica(&self) -> Rglica<dyn View> {
                (self as &dyn View).to_rglica()
            }
            fn init_views(&mut self) {
                #inits
            }
        }
        impl std::ops::Deref for #name {
            type Target = ViewBase;
            fn deref(&self) -> &ViewBase {
                &self.view
            }
        }
        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut ViewBase {
                &mut self.view
            }
        }
    }
    .into()
}

fn add_inits(fields: &FieldsNamed) -> TokenStream2 {
    let subview = Ident::new("SubView", Span::call_site());

    // dbg!(&subview);

    let mut res = quote!();

    for field in &fields.named {
        // dbg!(&field.ident);

        let name = field.ident.as_ref().unwrap();

        if let Type::Path(path) = &field.ty {
            for segment in &path.path.segments {
                if segment.ident == subview {
                    res = quote! {
                        #res
                        self.#name = self.initialize_view();
                    }
                }
            }
        }
    }

    res
}
