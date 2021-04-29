use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Token, Type, TypeTuple};

struct DeclareMemberOfType {
    tuple: syn::TypeTuple,
}

impl Parse for DeclareMemberOfType {
    fn parse(input: ParseStream) -> Result<Self> {
        let tuple = input.parse()?;
        Ok(DeclareMemberOfType { tuple: tuple })
    }
}
#[proc_macro]
pub fn declare_member_of_type(input: TokenStream) -> TokenStream {
    let DeclareMemberOfType { tuple } = parse_macro_input!(input as DeclareMemberOfType);

    let typ = tuple.elems.iter();
    let index = (0..tuple.elems.len()).map(syn::Index::from);
    let expanded = quote! {
        macro_rules! member_of_type {
            #(
                ($e:expr, #typ) => {
                    $e. #index
                };
            )*
        }
    };
    // println!("expanded: {}", expanded.to_string());

    expanded.into()
}
