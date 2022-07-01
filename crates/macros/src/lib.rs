use proc_macro::TokenStream;

mod attributes;
mod derive;
mod macros;

#[proc_macro_attribute]
pub fn node(_args: TokenStream, input: TokenStream) -> TokenStream {
    attributes::node::apply(input)
}

#[proc_macro_derive(Getter, attributes(exclude, clone, docs))]
pub fn getter(input: TokenStream) -> TokenStream {
    derive::getter::apply(input)
}

#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
    macros::symbols::apply(input)
}

#[proc_macro]
pub fn operators(input: TokenStream) -> TokenStream {
    macros::operators::apply(input)
}
