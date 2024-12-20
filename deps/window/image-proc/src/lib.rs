#![feature(proc_macro_span)]

extern crate proc_macro;

use std::path::PathBuf;

use proc_macro::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
    LitStr, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};
use walkdir::WalkDir;

// Custom struct to hold the parsed input
struct MacroInput {
    ty:  Type,
    lit: LitStr,
}

// Implement the Parse trait for MacroInput
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let lit: LitStr = input.parse()?;
        Ok(MacroInput { ty, lit })
    }
}

#[proc_macro]
pub fn include_images(input: TokenStream) -> TokenStream {
    // Get the span of the input
    let span = match input.clone().into_iter().next() {
        Some(TokenTree::Group(group)) => group.span(),
        Some(token) => token.span(),
        None => Span::call_site(),
    };

    // Get the file name from the span
    let mut file_path = span.source_file().path();

    file_path.pop();

    // Count the number of components in the path
    let component_count = file_path.components().count();

    // Create a new PathBuf with the relative path
    let mut relative_path = PathBuf::new();
    for _ in 0..component_count {
        relative_path.push("..");
    }

    // // Parse the input into a string literal
    // let input = parse_macro_input!(input as LitStr);
    // let folder_path = input.value();

    // Parse the input tokens into a MacroInput struct
    let MacroInput { ty, lit } = parse_macro_input!(input as MacroInput);

    let folder_path = format!("{}/{}", file_path.display(), lit.value());

    // Collect all image files in the specified folder
    let mut image_files = Vec::new();
    for entry in WalkDir::new(&folder_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ["png", "jpg", "jpeg", "gif", "bmp"].contains(&extension) {
                image_files.push(path.to_path_buf());
            }
        }
    }

    let mut methods = Vec::new();

    for image_path in image_files {
        let image_name = image_path.file_stem().and_then(|s| s.to_str()).unwrap();
        let image_name = image_name.replace('-', "_");
        let method_name = syn::Ident::new(&image_name, Span::call_site().into());
        let mut full_image_path = relative_path.clone();
        full_image_path.push(image_path.clone());
        let image_path_str = full_image_path.to_str().unwrap();

        methods.push(quote! {
            pub fn #method_name() -> Weak<Image> {
                Image::load(include_bytes!(#image_path_str), #image_path_str)
            }
        });
    }

    let expanded = quote! {
        impl #ty {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}
