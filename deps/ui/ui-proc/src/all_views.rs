use proc_macro::TokenStream;
use quote::quote;

use crate::view::VIEWS;

pub fn all_views_impl() -> TokenStream {
    let mut data = quote!();

    for view_name in VIEWS.lock().iter() {
        data = quote! {
            #data
            #view_name,
        };
    }

    quote! {
        [#data]
    }
    .into()
}
