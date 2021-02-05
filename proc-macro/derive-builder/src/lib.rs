extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let builder_ident = format_ident!("{}Builder", ident);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let opt_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if get_inner_ty("Option", ty).is_some() || builder_of(f).is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: std::option::Option<#ty> }
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let (arg_ty, value) = if let Some(inner_ty) = get_inner_ty("Option", ty) {
            (inner_ty, quote! { std::option::Option::Some(#name) })
        } else if builder_of(f).is_some() {
            (ty, quote! { #name })
        } else {
            (ty, quote! { std::option::Option::Some(#name) })
        };

        let set_method = quote! {
            pub fn #name(&mut self, #name: #arg_ty) -> &mut Self {
                self.#name = #value;
                self
            }
        };

        match get_each_method(f) {
            None => set_method.into(),
            Some((true, each_method)) => each_method,
            Some((false, each_method)) => {
                let methods = quote! {
                    #set_method
                    #each_method
                };
                methods.into()
            }
        }
    });

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if builder_of(f).is_some() {
            quote! { #name: std::vec::Vec::new() }
        } else {
            quote! { #name: std::option::Option::None }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if get_inner_ty("Option", &f.ty).is_some() || builder_of(f).is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#opt_fields,)*
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_fields,)*
                }
            }
        }

        impl #builder_ident {
            #(#methods)*

            pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#ident {
                    #(#build_fields,)*
                })
            }
        }
    };

    expanded.into()
}

fn builder_of(f: &syn::Field) -> Option<&syn::Attribute> {
    let attr = f.attrs.first()?;
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
        return Some(attr);
    }

    None
}

macro_rules! err {
    ($meta: expr) => {
        syn::Error::new_spanned($meta, r#"expected `builder(each = "...")`"#).to_compile_error()
    };
}

fn get_each_method(f: &syn::Field) -> Option<(bool, proc_macro2::TokenStream)> {
    let name = &f.ident;
    let ty = &f.ty;

    let attr = builder_of(f)?;

    let meta = match attr.parse_meta() {
        Ok(syn::Meta::List(list)) => list,
        Ok(meta) => return Some((false, err!(meta))),
        Err(e) => return Some((false, e.to_compile_error())),
    };

    let nv = match meta.nested.first() {
        Some(syn::NestedMeta::Meta(syn::Meta::NameValue(nv))) => nv,
        _ => {
            return Some((false, err!(meta)));
        }
    };

    if nv.path.get_ident().unwrap() != "each" {
        return Some((false, err!(meta)));
    }

    match &nv.lit {
        syn::Lit::Str(s) => {
            let ident = syn::Ident::new(&s.value(), s.span());
            let inner_ty = get_inner_ty("Vec", ty).unwrap();
            let method = quote! {
                pub fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#name.push(#ident);
                    self
                }
            };
            Some((Some(ident) == f.ident, method))
        }
        _ => Some((false, err!(meta))),
    }
}

fn get_inner_ty<'a>(outer: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != outer {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(inner_ty) = &p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let arg = inner_ty.args.first()?;
            if let syn::GenericArgument::Type(t) = arg {
                return Some(t);
            }
        }
    }
    None
}
