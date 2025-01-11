use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Fields, FieldsNamed, Item, ItemStruct, Result, Type};

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

    let (struct_fields, additional_items): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref();
            let field_type = &field.ty;
            if let Type::Path(type_path) = field_type {
                let last_segment = type_path.path.segments.last().unwrap();
                let key_wrapper = format_ident!("{}Key", field_name.as_ref().unwrap().to_string().to_upper_camel_case());

                match last_segment.ident.to_string().as_str() {
                    ident @ ("PersistentMap" | "InstanceMap" | "TempMap") => {
                        generate_map_field(field_name, field_type, &key_wrapper, ident)
                    },
                    ident @ ("PersistentStore" | "InstanceStore" | "TempStore") => {
                        generate_store_field(field_name, field_type, &key_wrapper, ident)
                    },
                    _ => Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TempMap, PersistentStore, InstanceStore, or TempStore")),
                }
            } else {
                Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TempMap, PersistentStore, InstanceStore, or TempStore"))
            }
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip();

    let field_names = fields.iter().map(|f| &f.ident);

    let main_struct = quote! {
        #[derive(Clone, Default)]
        pub struct #struct_name {
            #(#struct_fields,)*
        }

        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(#field_names: Default::default(),)*
                }
            }
        }

        impl Lazy for #struct_name {
            fn get_lazy() -> Option<Self> {
                Some(#struct_name::default())
            }

            fn set_lazy(self) {}
        }
    };

    let data_key_variants = generate_data_key_variants(fields)?;

    let additional_items = quote! {
        #[derive(Clone)]
        #[contracttype]
        pub enum DataKey {
            #(#data_key_variants,)*
        }

        #(#additional_items)*
    };

    Ok(quote! {
        #main_struct

        #additional_items
    })
}

fn generate_map_field(
    field_name: Option<&syn::Ident>,
    field_type: &Type,
    key_wrapper: &syn::Ident,
    map_type: &str,
) -> Result<(TokenStream, TokenStream)> {
    if let Type::Path(type_path) = field_type {
        let last_segment = type_path.path.segments.last().unwrap();
        if last_segment.ident == map_type {
            if let syn::PathArguments::AngleBracketed(generic_args) = &last_segment.arguments {
                if generic_args.args.len() == 2 {
                    let key_type = &generic_args.args[0];
                    let value_type = &generic_args.args[1];

                    let additional_item = quote! {
                        #[derive(Clone)]
                        pub struct #key_wrapper(#key_type);

                        impl From<#key_type> for #key_wrapper {
                            fn from(key: #key_type) -> Self {
                                Self(key)
                            }
                        }

                        impl LoamKey for #key_wrapper {
                            fn to_key(&self) -> Val {
                                DataKey::#field_name(self.0.clone()).into_val(env())
                            }
                        }
                    };
                    let map_type_ident = format_ident!("{}", map_type);
                    let struct_field = quote! { #field_name: #map_type_ident<#key_type, #value_type, #key_wrapper> };
                    Ok((struct_field, additional_item))
                } else {
                    Err(Error::new_spanned(
                        field_type,
                        format!("{map_type} must contain key and value types"),
                    ))
                }
            } else {
                Err(Error::new_spanned(
                    field_type,
                    format!("{map_type} must contain key and value types"),
                ))
            }
        } else {
            Err(Error::new_spanned(
                field_type,
                format!("Expected {}, found {}", map_type, last_segment.ident),
            ))
        }
    } else {
        Err(Error::new_spanned(
            field_type,
            format!("{map_type} must be a path type"),
        ))
    }
}

fn generate_store_field(
    field_name: Option<&syn::Ident>,
    field_type: &Type,
    key_wrapper: &syn::Ident,
    store_type: &str,
) -> Result<(TokenStream, TokenStream)> {
    if let Type::Path(type_path) = field_type {
        let last_segment = type_path.path.segments.last().unwrap();
        if last_segment.ident == store_type {
            if let syn::PathArguments::AngleBracketed(generic_args) = &last_segment.arguments {
                let value_type = &generic_args.args[0];
                let store_type_ident = format_ident!("{}", store_type);
                let struct_field =
                    quote! { #field_name: #store_type_ident<#value_type, #key_wrapper> };
                let additional_item = quote! {
                    #[derive(Clone, Default)]
                    pub struct #key_wrapper;

                    impl LoamKey for #key_wrapper {
                        fn to_key(&self) -> Val {
                            DataKey::#field_name.into_val(env())
                        }
                    }
                };
                Ok((struct_field, additional_item))
            } else {
                Err(Error::new_spanned(
                    field_type,
                    format!("{store_type} must contain value type"),
                ))
            }
        } else {
            Err(Error::new_spanned(
                field_type,
                format!("Expected {}, found {}", store_type, last_segment.ident),
            ))
        }
    } else {
        Err(Error::new_spanned(
            field_type,
            format!("{store_type} must be a path type"),
        ))
    }
}

fn generate_data_key_variants(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> Result<Vec<TokenStream>> {
    fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        if let Type::Path(type_path) = field_type {
            let last_segment = type_path.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "PersistentMap" | "InstanceMap" | "TempMap" => {
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
                "PersistentStore" | "InstanceStore" | "TempStore" => {
                    Ok(quote! { #field_name })
                },
                _ => Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TempMap, PersistentStore, InstanceStore, or TempStore")),
            }
        } else {
            Err(Error::new_spanned(field_type, "Must use one of PersistentMap, InstanceMap, TempMap, PersistentStore, InstanceStore, or TempStore"))
        }
    }).collect()
}
