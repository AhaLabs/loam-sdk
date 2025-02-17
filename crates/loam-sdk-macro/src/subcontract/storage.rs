use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Fields, FieldsNamed, Ident, Item, ItemStruct, Result, Type};

pub(crate) fn from_item(item: Item) -> Result<TokenStream> {
    match item {
        Item::Struct(item_struct) => generate_storage(&item_struct),
        _ => Err(Error::new_spanned(
            item,
            "loamstorage can only be applied to structs",
        )),
    }
}

fn generate_storage(item_struct: &ItemStruct) -> Result<TokenStream> {
    let struct_name = &item_struct.ident;
    let Fields::Named(FieldsNamed { named: fields, .. }) = &item_struct.fields else {
        return Err(Error::new_spanned(
            item_struct,
            "Only named fields are supported",
        ));
    };
    let module_name = format_ident!("{}_keys__", struct_name.to_string().to_snake_case());

    let (struct_fields, additional_items): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref();
            let field_type = &field.ty;
            let Type::Path(type_path) = field_type else {
                return Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TemporaryMap, PersistentItem, InstanceItem, or TemporaryItem"));
            };

            let last_segment = type_path.path.segments.last().unwrap();
            let key_wrapper = format_ident!("{}{}Key", struct_name, field_name.as_ref().unwrap().to_string().to_upper_camel_case());

            match last_segment.ident.to_string().as_str() {
                ident @ ("PersistentMap" | "InstanceMap" | "TemporaryMap") => {
                    generate_map_field(field_name, field_type, &key_wrapper, ident, &module_name, struct_name)
                },
                ident @ ("PersistentItem" | "InstanceItem" | "TemporaryItem") => {
                    generate_store_field(field_name, field_type, &key_wrapper, ident, &module_name, struct_name)
                },
                _ => Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TemporaryMap, PersistentItem, InstanceItem, or TemporaryItem")),
            }
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip();

    let main_struct = quote! {
        #[derive(Clone, Default)]
        pub struct #struct_name {
            #(#struct_fields,)*
        }

        impl soroban_sdk::Lazy for #struct_name {
            fn get_lazy() -> Option<Self> {
                Some(#struct_name::default())
            }

            fn set_lazy(self) {}
        }
    };

    let data_key_variants = generate_data_key_variants(fields, struct_name)?;
    let data_key = format_ident!("{struct_name}Key");

    let additional_items = quote! {
        #[derive(Clone)]
        #[soroban_sdk::contracttype]
        pub enum #data_key {
            #(#data_key_variants,)*
        }

        #(#additional_items)*
    };

    Ok(quote! {
        #main_struct
        mod #module_name {
            use super::*;
            #additional_items
        }
    })
}

fn generate_map_field(
    field_name: Option<&syn::Ident>,
    field_type: &Type,
    key_wrapper: &syn::Ident,
    map_type: &str,
    module_name: &Ident,
    struct_name: &Ident,
) -> Result<(TokenStream, TokenStream)> {
    let Type::Path(type_path) = field_type else {
        return Err(Error::new_spanned(
            field_type,
            format!("{map_type} must be a path type"),
        ));
    };
    let last_segment = type_path.path.segments.last().unwrap();
    if last_segment.ident != map_type {
        return Err(Error::new_spanned(
            field_type,
            format!("Expected {}, found {}", map_type, last_segment.ident),
        ));
    }
    let syn::PathArguments::AngleBracketed(generic_args) = &last_segment.arguments else {
        return Err(Error::new_spanned(
            field_type,
            format!("{map_type} must contain key and value types"),
        ));
    };
    if generic_args.args.len() != 2 {
        return Err(Error::new_spanned(
            field_type,
            format!("{map_type} must contain key and value types"),
        ));
    }
    let enum_case_name = field_to_enum_case(field_name, struct_name);
    let key_type = &generic_args.args[0];
    let value_type = &generic_args.args[1];
    let data_key = format_ident!("{struct_name}Key");

    let additional_item = quote! {
        #[derive(Clone)]
        pub struct #key_wrapper(#key_type);

        impl From<#key_type> for #key_wrapper {
            fn from(key: #key_type) -> Self {
                Self(key)
            }
        }

        impl soroban_sdk::LoamKey for #key_wrapper {
            fn to_key(&self) -> soroban_sdk::Val {
                soroban_sdk::IntoVal::into_val(&#data_key::#enum_case_name(self.0.clone()),soroban_sdk::env())
            }
        }
    };
    let map_type_ident = format_ident!("{}", map_type);
    let struct_field =
        quote! { #field_name: #map_type_ident<#key_type, #value_type, #module_name::#key_wrapper> };
    Ok((struct_field, additional_item))
}

fn generate_store_field(
    field_name: Option<&syn::Ident>,
    field_type: &Type,
    key_wrapper: &syn::Ident,
    store_type: &str,
    module_name: &Ident,
    struct_name: &Ident,
) -> Result<(TokenStream, TokenStream)> {
    let Type::Path(type_path) = field_type else {
        return Err(Error::new_spanned(
            field_type,
            format!("{store_type} must be a path type"),
        ));
    };
    let last_segment = type_path.path.segments.last().unwrap();
    if last_segment.ident != store_type {
        return Err(Error::new_spanned(
            field_type,
            format!("Expected {}, found {}", store_type, last_segment.ident),
        ));
    }
    let syn::PathArguments::AngleBracketed(generic_args) = &last_segment.arguments else {
        return Err(Error::new_spanned(
            field_type,
            format!("{store_type} must contain value type"),
        ));
    };

    let enum_case_name = field_to_enum_case(field_name, struct_name);
    let value_type = &generic_args.args[0];
    let store_type_ident = format_ident!("{}", store_type);
    let struct_field =
        quote! { #field_name: #store_type_ident<#value_type, #module_name::#key_wrapper> };
    let data_key = format_ident!("{struct_name}Key");
    let additional_item = quote! {
        #[derive(Clone, Default)]
        pub struct #key_wrapper;

        impl soroban_sdk::LoamKey for #key_wrapper {
            fn to_key(&self) -> soroban_sdk::Val {
                soroban_sdk::IntoVal::into_val(&#data_key::#enum_case_name, soroban_sdk::env())
            }
        }
    };
    Ok((struct_field, additional_item))
}

fn field_to_enum_case(field_name: Option<&Ident>, struct_name: &Ident) -> Option<Ident> {
    field_name.map(|name| {
        let enum_case = format!("{}{}", struct_name, name.to_string().to_upper_camel_case());
        syn::Ident::new(&enum_case, name.span())
    })
}

fn generate_data_key_variants(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    struct_name: &Ident,
) -> Result<Vec<TokenStream>> {
    fields.iter().map(|field| {
        let field_name = field.ident.as_ref();
        let field_name = field_to_enum_case(field_name, struct_name);
        let field_type = &field.ty;

        let Type::Path(type_path) = field_type else {
            return Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TemporaryMap, PersistentItem, InstanceItem, or TemporaryItem"));
        };
        let last_segment = type_path.path.segments.last().unwrap();
        match last_segment.ident.to_string().as_str() {
            "PersistentMap" | "InstanceMap" | "TemporaryMap" => {
                let args = &last_segment.arguments;
                if let syn::PathArguments::AngleBracketed(generic_args) = args {
                    if generic_args.args.len() == 2 {
                        let key_type = &generic_args.args[0];
                        Ok(quote! { #field_name(#key_type) })
                    } else {
                        Err(Error::new_spanned(field_type, "Map must contain key and value types"))
                    }
                } else {
                    Err(Error::new_spanned(field_type, "Map must contain key and value types"))
                }
            },
            "PersistentItem" | "InstanceItem" | "TemporaryItem" => {
                Ok(quote! { #field_name })
            },
            _ => Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TemporaryMap, PersistentItem, InstanceItem, or TemporaryItem")),
        }
    }).collect()
}

#[cfg(test)]
mod test {
    use crate::util::equal_tokens;

    use super::*;

    #[test]
    fn test_generate_storage() {
        let input: Item = syn::parse_quote! {
            struct Foo {
                bar: PersistentMap<String, u64>,
                baz: TemporaryItem<u64>,
            }
        };
        let generated = from_item(input).unwrap();
        let expected = quote! {
        #[derive(Clone, Default)]
        pub struct Foo {
            bar: PersistentMap<String, u64, foo_keys__::FooBarKey>,
            baz: TemporaryItem<u64, foo_keys__::FooBazKey>,
        }
        impl soroban_sdk::Lazy for Foo {
            fn get_lazy() -> Option<Self> {
                Some(Foo::default())
            }
            fn set_lazy(self) {}
        }
        mod foo_keys__ {
            use super::*;
            #[derive(Clone)]
            #[soroban_sdk::contracttype]
            pub enum FooKey {
                FooBar(String),
                FooBaz,
            }
            #[derive(Clone)]
            pub struct FooBarKey(String);
            impl From<String> for FooBarKey {
                fn from(key: String) -> Self {
                    Self(key)
                }
            }
            impl soroban_sdk::LoamKey for FooBarKey {
                fn to_key(&self) -> soroban_sdk::Val {
                    soroban_sdk::IntoVal::into_val(&FooKey::FooBar(self.0.clone()), soroban_sdk::env())
                }
            }
            #[derive(Clone, Default)]
            pub struct FooBazKey;
            impl soroban_sdk::LoamKey for FooBazKey {
                fn to_key(&self) -> soroban_sdk::Val {
                    soroban_sdk::IntoVal::into_val(&FooKey::FooBaz, soroban_sdk::env())
                }
            }
        }

                };
        equal_tokens(&expected, &generated);
    }
}
