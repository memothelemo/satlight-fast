use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Visibility};

#[allow(unused_assignments)]
pub fn apply(input: TokenStream) -> TokenStream {
    let item: ItemStruct = syn::parse(input).expect("Getter can be only derived for structs");
    let ItemStruct {
        ident, generics, ..
    } = &item;
    let mut methods = Vec::new();

    for field in &item.fields {
        let mut excluded = false;
        let mut clone = false;
        for attr in &field.attrs {
            let attr_name = match attr.path.get_ident() {
                Some(ident) => ident.to_string(),
                None => continue,
            };
            match attr_name.as_str() {
                "exclude" => excluded = true,
                "clone" => clone = true,
                _ => {}
            }
        }
        if excluded || matches!(field.vis, Visibility::Public(..)) {
            continue;
        }
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        let field_ty = if clone {
            quote! { #field_ty }
        } else {
            quote! { &#field_ty }
        };
        let self_ = if clone {
            quote! {self}
        } else {
            quote! {&self}
        };
        let expr = if clone {
            quote! { #self_.#field_ident.clone() }
        } else {
            quote! { #self_.#field_ident }
        };
        methods.push(quote! {
            #[allow(missing_docs)]
            pub fn #field_ident(&self) -> #field_ty {
                #expr
            }
        });
    }

    quote! {
        impl #generics #ident #generics {
            #(#methods)*
        }
    }
    .into()
}
