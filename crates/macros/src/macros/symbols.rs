use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parse, Attribute, LitStr, Token};

/*
$main_name
$( #[attrs] )*
$name => $expr,
*/

struct SymbolVariant {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub literal: LitStr,
}

impl Parse for SymbolVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;
        let literal = input.parse::<LitStr>()?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        };
        Ok(Self {
            attrs,
            ident,
            literal,
        })
    }
}

struct SymbolStruct {
    name: Ident,
    variants: Vec<SymbolVariant>,
}

impl Parse for SymbolStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        let mut variants = Vec::new();
        while !input.is_empty() {
            variants.push(input.parse::<SymbolVariant>()?);
        }

        Ok(Self { name, variants })
    }
}

pub fn apply(input: TokenStream) -> TokenStream {
    let SymbolStruct { name, variants } =
        syn::parse::<SymbolStruct>(input).expect("expected symbol part");

    let mut enum_members = Vec::new();
    let mut display_fmt_pat = Vec::new();
    let mut from_str_pat = Vec::new();
    let mut symbol_members = Vec::new();

    // let total_variants = variants.len();
    // let total_variants = syn::LitInt::new(
    //     &format!("{}usize", total_variants),
    //     quote::__private::Span::call_site(),
    // );

    let mut cmps = Vec::new();

    for variant in variants {
        let ident = variant.ident;
        let attrs = variant.attrs;
        let value = variant.literal;
        let lit = syn::LitByteStr::new(
            value.value().as_bytes(),
            quote::__private::Span::call_site(),
        );
        cmps.push(quote! {
            if #lit == s {
                return Ok(#name::#ident);
            }
        });
        enum_members.push(quote! {
            #(#attrs)*
            #ident,
        });
        display_fmt_pat.push(quote! {
            Self::#ident => #value,
        });
        from_str_pat.push(quote! {
            #value => #name::#ident,
        });
        symbol_members.push(quote! {
            #name::#ident
        });
    }

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub enum #name {
            #(#enum_members)*
        }

        impl #name {
            fn to_str(&self) -> &'static str {
                match self {
                    #(#display_fmt_pat)*
                    _ => unreachable!("unknown symbol type"),
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    #(#display_fmt_pat)*
                    _ => unreachable!("unknown symbol type"),
                })
            }
        }

        impl std::str::FromStr for #name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.as_bytes();
                #(#cmps)*
                Err("unknown lua symbol")
            }
        }
    }
    .into()
}
