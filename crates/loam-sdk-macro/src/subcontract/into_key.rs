use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Item};

pub(crate) fn from_item(item: Item) -> Result<TokenStream, syn::Error> {
    let mut is_unit = false;
    let (name, generics) = match item {
        Item::Union(union_) => (union_.ident, union_.generics),
        Item::Enum(item) => (item.ident, item.generics),
        Item::Struct(item) => {
            if let Fields::Unit = item.fields {
                is_unit = true;
            }
            (item.ident, item.generics)
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "IntoKey can only be used as a derive on enums or structs.",
            ))
        }
    };
    let name_str = name.to_string();
    let string = quote! { loam_sdk::soroban_sdk::String};
    let body = if is_unit {
        quote! {}
    } else {
        quote! { #string::from_slice(loam_sdk::soroban_sdk::env(), #name_str)}
    };

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics loam_sdk::soroban_sdk::IntoKey for #name #ty_generics {
            type Key = #string;
            fn into_key() -> Self::Key {
                #body
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_key() {
        let input: Item = syn::parse_quote! {
           struct Foo;
        };
        let result = from_item(input).unwrap().to_string();
        println!("{result}");
        let impl_ = syn::parse_str::<syn::ItemImpl>(result.as_str()).unwrap();
        println!("{impl_:#?}");
    }
}
