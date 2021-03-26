use proc_macro::TokenStream;
use quote::{quote, format_ident};

#[proc_macro]
pub fn hello(input: TokenStream) -> TokenStream {
    let name = format_ident!("fn_{}", input.to_string());
    let expanded = quote! {
        fn #name<T: std::fmt::Debug>(t: T) {
            println!("{:?}",t);
        }
    };
    expanded.into()
}