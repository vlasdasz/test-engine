use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Data, DeriveInput, Field, Fields};

#[proc_macro_attribute]
pub fn view(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let data = match &mut stream.data {
        Data::Struct(data) => data,
        _ => panic!("`view` macro has to be used with structs"),
    };

    if let Fields::Named(fields) = &mut data.fields {
        fields
            .named
            .push(Field::parse_named.parse2(quote! { view: ViewBase }).unwrap());
    }

    let name = &stream.ident;

    quote! {
        #stream
        impl View for #name {
            fn rglica(&self) -> Rglica<dyn View> { (self as &dyn View).to_rglica() }
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
