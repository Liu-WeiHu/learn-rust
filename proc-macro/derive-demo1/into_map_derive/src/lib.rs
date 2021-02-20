use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote};

#[proc_macro_derive(IntoMap)]
pub fn into_map(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct{fields: syn::Fields::Named(fields),..}) = ast.data {
        fields
    }else {
        unimplemented!()
    };
    let fields = fields.named.iter().map(|field| {
        let field_name = &field.ident;
     
        quote!{
            map.insert(
                stringify!(#field_name).to_string(),
                self.#field_name.to_string(),
            );
        }
    });

    let expand = quote! {
        use std::collections::hash_map::HashMap;

        impl #struct_name {
            pub fn into_map(&self) -> HashMap<String, String> {
                let mut map = HashMap::new();
                #(#fields)*
                map
            }
        }
    };
    expand.into()
}
