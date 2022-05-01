use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn view(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let struct_data = match &mut stream.data {
        syn::Data::Struct(data) => data,
        _ => panic!("`view` macro has to be used with structs"),
    };

    if let syn::Fields::Named(fields) = &mut struct_data.fields {
        fields
            .named
            .push(syn::Field::parse_named.parse2(quote! { view: ViewBase }).unwrap());
    }

    quote! { #stream }.into()
}
