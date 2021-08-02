extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AsAny)]
pub fn derive_as_any(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl tools::AsAny for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    return TokenStream::from(expanded);
}
