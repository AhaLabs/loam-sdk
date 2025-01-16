use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Item, ItemStruct};

pub(crate) fn from_item(item: Item) -> Result<TokenStream, syn::Error> {
    let Item::Struct(ItemStruct {
        fields: Fields::Unit,
        ident,
        generics,
        ..
    }) = item
    else {
        return Err(syn::Error::new(
            Span::call_site(),
            "Lazy can only be derived on empty structs.",
        ));
    };

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics loam_sdk::soroban_sdk::Lazy for #ident #ty_generics {
            fn get_lazy() -> Option<Self> {
                Some(Self)
            }

            fn set_lazy(self) {}
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
