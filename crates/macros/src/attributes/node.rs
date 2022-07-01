use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn apply(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // enum and struct supported types
    let mut attrs_prereqs = vec![quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
    }];

    if matches!(input.data, Data::Struct(..)) {
        attrs_prereqs.push(quote! {
            #[derive(satlight_macros::Getter, derive_more::Constructor)]
        });
    }

    quote! {
        #(#attrs_prereqs)*
        #input
    }
    .into()
}
