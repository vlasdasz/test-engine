use proc_macro::TokenStream;
use quote::quote;

use crate::view::{VIEW_TESTS, VIEWS};

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

pub fn all_view_tests_impl() -> TokenStream {
    let mut data = quote!();

    for view_name in VIEW_TESTS.lock().iter() {
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
