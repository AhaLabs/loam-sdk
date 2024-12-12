#![recursion_limit = "128"]
extern crate proc_macro;
use proc_macro::TokenStream;
use heck::ToUpperCamelCase;
use std::env;
use subcontract::derive_contract_impl;

use quote::quote;
use syn::Item;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

mod contract;
mod subcontract;
mod util;

/// Generates a companion Trait which has a default type `Impl`, which implements this trait.
///
/// # Panics
///
/// This macro will panic if:
/// - The input `TokenStream` cannot be parsed into a valid Rust item.
/// - The `subcontract::generate` function fails to generate the companion trait.
///
#[proc_macro_attribute]
pub fn subcontract(_: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: Item = syn::parse(item).unwrap();
    subcontract::generate(&parsed).into()
}

#[proc_macro_derive(IntoKey)]
pub fn into_key(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(subcontract::into_key::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

#[proc_macro_derive(Lazy)]
pub fn lazy(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(subcontract::lazy::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

pub(crate) fn manifest() -> std::path::PathBuf {
    std::path::PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("failed to finde cargo manifest"),
    )
    .join("Cargo.toml")
}

/// Generates a contract Client for a given contract.
/// It is expected that the name should be the same as the published contract or a contract in your current workspace.
///
/// # Panics
///
/// This function may panic in the following situations:
/// - If `loam_build::get_target_dir()` fails to retrieve the target directory
/// - If the input tokens cannot be parsed as a valid identifier
/// - If the directory path cannot be canonicalized
/// - If the canonical path cannot be converted to a string
#[proc_macro]
pub fn import_contract(tokens: TokenStream) -> TokenStream {
    let cargo_file = manifest();
    let mut dir = loam_build::get_target_dir(&cargo_file)
        .unwrap()
        .join(tokens.to_string());
    let name = syn::parse::<syn::Ident>(tokens).expect("The input must be a valid identifier");
    dir.set_extension("wasm");
    let binding = dir.canonicalize().unwrap();
    let file = binding.to_str().unwrap();
    quote! {
        mod #name {
            use loam_sdk::soroban_sdk;
            loam_sdk::soroban_sdk::contractimport!(file = #file);
        }
    }
    .into()
}

/// Generates a contract made up of subcontracts
/// ```ignore
/// #[derive_contract(Core(Admin), Postable(StatusMessage))]
/// pub struct Contract;
/// ```
/// Generates
///
/// ```ignore
/// pub struct Contract;
/// impl Postable for Contract {
///     type Impl = StatusMessage;
/// }
/// impl Core for Contract {
///     type Impl = Admin;
/// }
/// #[loam_sdk::soroban_sdk::contract]
/// struct SorobanContract__;
///
/// #[loam_sdk::soroban_sdk::contract]
/// impl SorobanContract__ {
///  // Postable and Core methods exposed
/// }
///
/// ```
///
/// # Panics
/// This function may panic if the input tokens cannot be parsed as a valid Rust item.
///
#[proc_macro_attribute]
pub fn derive_contract(args: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: Item = syn::parse(item.clone()).expect("failed to parse Item");
    derive_contract_impl(proc_macro2::TokenStream::from(args), parsed).into()
}

/// Generates a contract Client for a given asset.
/// It is expected that the name of an asset, e.g. "native" or "USDC:G1...."
///
/// # Panics
///
#[proc_macro]
pub fn stellar_asset(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal
    let input_str = syn::parse_macro_input!(input as syn::LitStr);
    let network = std::env::var("STELLAR_NETWORK").unwrap_or_else(|_| "local".to_owned());
    let asset = util::parse_asset_literal(&input_str, &network);

    // Return the generated code as a TokenStream
    asset.into()
}


#[proc_macro_attribute]
pub fn loamstorage(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Only named fields are supported"),
            }
        },
        _ => panic!("Only structs are supported"),
    };

    let (struct_fields, additional_items): (Vec<_>, Vec<_>) = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        if let Type::Path(type_path) = field_type {
            let last_segment = type_path.path.segments.last().unwrap();
            let key_wrapper = quote::format_ident!("{}Key", field_name.as_ref().unwrap().to_string().to_upper_camel_case());

            match last_segment.ident.to_string().as_str() {
                "PersistentMap" => {
                    let args = &last_segment.arguments;
                    if let syn::PathArguments::AngleBracketed(generic_args) = args {
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
                            let struct_field = quote! { #field_name: PersistentMap<#key_type, #value_type, #key_wrapper> };
                            (struct_field, additional_item)
                        } else {
                            panic!("PersistentMap must contain value and type");
                        }
                    } else {
                        panic!("PersistentMap must contain value and type");
                    }
                },
                "PersistentStore" => {
                    let args = &last_segment.arguments;
                    if let syn::PathArguments::AngleBracketed(generic_args) = args {
                        let value_type = &generic_args.args[0];
    
                        let struct_field = quote! { #field_name: PersistentStore<#value_type, #key_wrapper> };
                        let additional_item = quote! {
                            #[derive(Clone, Default)]
                            pub struct #key_wrapper;
    
                            impl LoamKey for #key_wrapper {
                                fn to_key(&self) -> Val {
                                    DataKey::#field_name.into_val(env())
                                }
                            }

                        };
                        (struct_field, additional_item)
                    } else {
                        panic!("PersistentStore must contain type");
                    }
                },
                _ => panic!("Must use one of PersistentStore or PersistentMap"),
            }
        } else {
            panic!("Must use one of PersistentStore or PersistentMap");
        }
    }).unzip();

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

            fn set_lazy(self) {
            }
        }
    };

    let data_key_variants = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        if let Type::Path(type_path) = field_type {
            let last_segment = type_path.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "PersistentMap" => {
                    let args = &last_segment.arguments;
                    if let syn::PathArguments::AngleBracketed(generic_args) = args {
                        if generic_args.args.len() == 2 {
                            let key_type = &generic_args.args[0];
                            quote! { #field_name(#key_type) }
                        } else {
                            quote! { #field_name }
                        }
                    } else {
                        quote! { #field_name }
                    }
                },
                "PersistentStore" => {
                    quote! { #field_name }
                },
                _ => quote! { #field_name },
            }
        } else {
            quote! { #field_name }
        }
    });

    let additional_items = quote! {
        #[derive(Clone)]
        #[contracttype]
        pub enum DataKey {
            #(#data_key_variants,)*
        }

        #(#additional_items)*
    };

    let result = quote! {
        #main_struct

        #additional_items
    };

    result.into()
}
