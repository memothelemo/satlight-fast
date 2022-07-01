use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Brace,
    Attribute, Ident, LitInt, Token, Visibility,
};

pub struct Variant {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fat_arrow: Token![=>],
    pub value: LitInt,
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident: Ident = input.parse()?;
        let fat_arrow = input.parse()?;
        let value = input.parse()?;
        Ok(Self {
            attrs,
            ident,
            fat_arrow,
            value,
        })
    }
}

pub struct OperatorData {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub brace_token: Brace,
    pub variants: Punctuated<Variant, Token![,]>,
}

impl Parse for OperatorData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let enum_token = input.parse::<Token![enum]>()?;
        let ident = input.parse::<Ident>()?;
        let content;
        let brace_token = braced!(content in input);
        let variants = content.parse_terminated(Variant::parse)?;
        Ok(Self {
            attrs,
            vis,
            enum_token,
            ident,
            brace_token,
            variants,
        })
    }
}

pub fn apply(input: TokenStream) -> TokenStream {
    let OperatorData {
        attrs,
        vis,
        ident,
        variants,
        ..
    } = parse_macro_input!(input as OperatorData);

    let mut enum_variants = Vec::new();
    let mut match_variants = Vec::new();

    for variant in variants {
        let Variant {
            attrs,
            ident,
            value,
            ..
        } = variant;
        enum_variants.push(quote! {
            #(#attrs)*
            #ident,
        });
        match_variants.push(quote! {
            Self::#ident => #value,
        });
    }

    quote! {
        #(#attrs)*
        #vis enum #ident {
            #(#enum_variants)*
        }

        impl #ident {
            pub fn order(&self) -> usize {
                match self {
                    #(#match_variants)*
                }
            }
        }
    }
    .into()
}
