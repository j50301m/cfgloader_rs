//! Procedural macros for cfgloader
//!
//! This crate provides the `FromEnv` derive macro that automatically generates
//! configuration loading code from environment variables.

// Proc macro implementation
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(FromEnv, attributes(env))]
pub fn derive_from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(f) => f.named,
            Fields::Unnamed(_) | Fields::Unit => {
                return syn::Error::new_spanned(
                    name,
                    "FromEnv only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "FromEnv only supports structs")
                .to_compile_error()
                .into();
        }
    };

    let mut inits = Vec::new();

    for field in fields {
        let ident = field.ident.unwrap();
        let ty = field.ty.clone();

        // Default: no env annotation → if type also implements FromEnv, call its load(); otherwise use Default (if implemented)
        // If #[env(...)] is present, fill value according to rules
        let mut key_tokens = None::<proc_macro2::TokenStream>;
        let mut default_tokens = None::<proc_macro2::TokenStream>;
        let mut required = false;
        let mut split_tokens = None::<proc_macro2::TokenStream>;

        for attr in field.attrs.iter().filter(|a| a.path().is_ident("env")) {
            // Parse #[env("KEY", default = "value", required, split = ",")]
            match &attr.meta {
                syn::Meta::List(list) => {
                    let tokens = &list.tokens;
                    let content = tokens.to_string();

                    // Parse something like "DB_URL", default = "sqlite://test.db"
                    if content.starts_with('"') {
                        // Find the first string literal as the key
                        if let Some(first_quote_end) = content[1..].find('"') {
                            let key = &content[1..first_quote_end + 1];
                            key_tokens = Some(quote! { #key });

                            // Look for additional arguments after the key
                            let remaining = &content[first_quote_end + 2..];

                            // Parse default = "..."
                            if let Some(default_start) = remaining.find("default") {
                                let default_part = &remaining[default_start..];
                                if let Some(eq_pos) = default_part.find('=') {
                                    let value_part = default_part[eq_pos + 1..].trim();
                                    if value_part.starts_with('"') {
                                        if let Some(end_quote) = value_part[1..].find('"') {
                                            let value = &value_part[1..end_quote + 1];
                                            default_tokens = Some(quote! { #value });
                                        }
                                    }
                                }
                            }

                            // Check for required
                            if remaining.contains("required") {
                                required = true;
                            }

                            // Parse split = "..."
                            if let Some(split_start) = remaining.find("split") {
                                let split_part = &remaining[split_start..];
                                if let Some(eq_pos) = split_part.find('=') {
                                    let value_part = split_part[eq_pos + 1..].trim();
                                    if value_part.starts_with('"') {
                                        if let Some(end_quote) = value_part[1..].find('"') {
                                            let value = &value_part[1..end_quote + 1];
                                            split_tokens = Some(quote! { #value });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    // Skip other attribute formats for now
                }
            }
        }

        let init_one = if let Some(key) = key_tokens {
            let (ty_item_opt, _is_vec) = element_type(&ty);
            if let Some(ty_item) = ty_item_opt {
                // Vec<T>
                let split = split_tokens.clone().unwrap_or_else(|| quote! { "," });
                if required && default_tokens.is_none() {
                    // For required Vec fields without default
                    quote! {
                        #ident: {
                            match ::cfgloader_core::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => ::cfgloader_core::parse_vec::<#ty_item>(#key, raw.clone(), #split)?,
                                _ => return Err(::cfgloader_core::CfgError::MissingEnv(#key))
                            }
                        }
                    }
                } else {
                    let default_branch = if let Some(def) = default_tokens.clone() {
                        quote! {
                            ::cfgloader_core::parse_vec::<#ty_item>(#key, #def.to_string(), #split)?
                        }
                    } else {
                        quote! { Vec::<#ty_item>::new() }
                    };
                    quote! {
                        #ident: {
                            match ::cfgloader_core::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => ::cfgloader_core::parse_vec::<#ty_item>(#key, raw.clone(), #split)?,
                                _ => #default_branch
                            }
                        }
                    }
                }
            } else {
                // scalar
                if required && default_tokens.is_none() {
                    // For required fields without default, we need special handling
                    quote! {
                        #ident: {
                            match ::cfgloader_core::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => ::cfgloader_core::parse_scalar::<#ty>(#key, raw.clone())?,
                                _ => return Err(::cfgloader_core::CfgError::MissingEnv(#key))
                            }
                        }
                    }
                } else {
                    let default_branch = if let Some(def) = default_tokens.clone() {
                        quote! { ::cfgloader_core::parse_scalar::<#ty>(#key, #def.to_string())? }
                    } else {
                        quote! { Default::default() }
                    };
                    quote! {
                        #ident: {
                            match ::cfgloader_core::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => ::cfgloader_core::parse_scalar::<#ty>(#key, raw.clone())?,
                                _ => #default_branch
                            }
                        }
                    }
                }
            }
        } else {
            // No #[env]: directly call T::load() for nested loading
            quote! {
                #ident: {
                    #ty::load(env_path)?
                }
            }
        };

        inits.push(init_one);
    }

    let expanded = quote! {
        impl ::cfgloader_core::FromEnv for #name {
            fn load(env_path: &std::path::Path) -> Result<Self, ::cfgloader_core::CfgError> {
                // Load .env file if it exists
                ::cfgloader_core::load_env_file(env_path)?;

                Ok(Self {
                    #(#inits),*
                })
            }
        }
    };

    expanded.into()
}

/// Return Some(T) if Vec<T>, otherwise None
fn element_type(ty: &syn::Type) -> (Option<syn::Type>, bool) {
    if let syn::Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                        return (Some(inner.clone()), true);
                    }
                }
            }
        }
    }
    (None, false)
}
