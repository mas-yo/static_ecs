use itertools::Itertools;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Token, Type, TypeTuple};

fn permutations(types: Vec<syn::Type>) -> Vec<Vec<syn::Type>> {
    let mut permutations = Vec::new();
    let len = types.len();
    for l in 1..=len {
        let mut perm: Vec<Vec<syn::Type>> = types
            .iter()
            .permutations(l)
            .map(|x| x.iter().map(|y| (*y).clone()).collect())
            .collect();
        permutations.append(&mut perm);
    }
    permutations
}

// struct ImplTypeRef {
//     ident: syn::Ident,
//     tuple: syn::TypeTuple,
// }

// impl Parse for ImplTypeRef {
//     fn parse(input: ParseStream) -> Result<Self> {
//         input.parse::<Token![struct]>()?;
//         let id = input.parse::<syn::Ident>()?;
//         let tpl = input.parse::<syn::TypeTuple>()?;
//         input.parse::<Token![;]>();
//         Ok(ImplTypeRef {
//             ident: id,
//             tuple: tpl,
//         })
//     }
// }

struct TypeAndIndex {
    typ: syn::Type,
    index: syn::Index,
}
impl TypeAndIndex {
    fn new(typ: syn::Type, index: syn::Index) -> Self {
        Self {
            typ: typ,
            index: index,
        }
    }
}

#[proc_macro_derive(World, attributes(components))]
pub fn derive_world(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::ItemStruct); //ImplTypeRef);

    // let parsed = dbg!(parsed);

    let target = &parsed.ident;

    let mut tuple = None::<syn::TypeTuple>;
    let mut tuple_ident = None::<syn::Ident>;
    if let syn::Fields::Named(fields) = parsed.fields {
        for f in fields.named {
            if f.attrs
                .iter()
                .any(|a| a.path.segments.iter().any(|s| s.ident == "components"))
            {
                match f.ty {
                    syn::Type::Tuple(tpl) => {
                        tuple_ident = f.ident;
                        tuple = Some(tpl);
                    }
                    _ => {
                        panic!("components must be a Tuple")
                    }
                }
                break;
            }
        }
    }

    if tuple.is_none() {
        return TokenStream::new();
    }
    let tuple = tuple.unwrap();
    let tuple_ident = tuple_ident.unwrap();

    let type_and_index: Vec<TypeAndIndex> = tuple
        .elems
        .iter()
        .enumerate()
        .map(|x| TypeAndIndex::new(x.1.clone(), x.0.into()))
        .collect();
    let get_index = |typ: &syn::Type| {
        type_and_index
            .iter()
            .find(|x| x.typ == *typ)
            .unwrap()
            .index
            .clone()
    };

    let mut ts = quote::__private::TokenStream::new();
    let types: Vec<syn::Type> = tuple.elems.iter().map(|x| (*x).clone()).collect();
    let perms = permutations(types);
    for perm in &perms {
        let typ = perm.iter();
        let idx = perm.iter().map(|x| get_index(x));
        let ref_types = quote! { ( #( &'a #typ ),* ) };
        let ref_index = quote! { ( #( & self. #tuple_ident . #idx ),* ) };

        let ref_mut1 = perm.iter().enumerate().map(|(idx, x)| {
            if idx == 0 {
                quote! {&'a mut}
            } else {
                quote! {&'a}
            }
        });
        let ref_mut2 = perm.iter().enumerate().map(|(idx, x)| {
            if idx == 0 {
                quote! {&mut}
            } else {
                quote! {&}
            }
        });
        let typ = perm.iter();
        let idx = perm.iter().map(|x| get_index(x));
        let ref_mut_types = quote! { ( #( #ref_mut1 #typ ),* ) };
        let ref_mut_index = quote! { ( #( #ref_mut2 self. #tuple_ident . #idx ),* ) };

        let impls = quote! {
            impl<'a> GetComponent<'a, #ref_types> for #target {
                fn get_component(&'a self) -> #ref_types {
                    #ref_index
                }
            }
            impl<'a> GetComponentMut<'a, #ref_mut_types> for #target {
                fn get_component_mut(&'a mut self) -> #ref_mut_types {
                    #ref_mut_index
                }
            }
        };
        ts.extend(impls);
    }

    // println!("-------¥n {}", ts.to_string());
    ts.into()
}
