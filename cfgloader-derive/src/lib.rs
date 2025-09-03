use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(
                name,
                "FromEnv only supports structs",
            )
            .to_compile_error()
            .into()
        }
    };

    let mut inits = Vec::new();

    for field in fields {
        let ident = field.ident.unwrap();
        let ty = field.ty.clone();

        // 預設：沒有 env 標註 → 如果型別也 FromEnv，呼叫其 load()；否則用 Default（若實作）
        // 若有 #[env(...)] 則依規則填值
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
                                    let value_part = default_part[eq_pos+1..].trim();
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
                                    let value_part = split_part[eq_pos+1..].trim();
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
                    // 對於 required 且沒有 default 的 Vec 字段
                    quote! {
                        #ident: {
                            match crate::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => crate::parse_vec::<#ty_item>(#key, raw.clone(), #split)?,
                                _ => return Err(crate::CfgError::MissingEnv(#key))
                            }
                        }
                    }
                } else {
                    let default_branch = if let Some(def) = default_tokens.clone() {
                        quote! {
                            crate::parse_vec::<#ty_item>(#key, #def.to_string(), #split)?
                        }
                    } else {
                        quote! { Vec::<#ty_item>::new() }
                    };
                    quote! {
                        #ident: {
                            match crate::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => crate::parse_vec::<#ty_item>(#key, raw.clone(), #split)?,
                                _ => #default_branch
                            }
                        }
                    }
                }
            } else {
                // scalar
                if required && default_tokens.is_none() {
                    // 對於 required 且沒有 default 的字段，我們需要特殊處理
                    quote! {
                        #ident: {
                            match crate::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => crate::parse_scalar::<#ty>(#key, raw.clone())?,
                                _ => return Err(crate::CfgError::MissingEnv(#key))
                            }
                        }
                    }
                } else {
                    let default_branch = if let Some(def) = default_tokens.clone() {
                        quote! { crate::parse_scalar::<#ty>(#key, #def.to_string())? }
                    } else {
                        quote! { Default::default() }
                    };
                    quote! {
                        #ident: {
                            match crate::get_env(#key) {
                                Some(ref raw) if !raw.trim().is_empty() => crate::parse_scalar::<#ty>(#key, raw.clone())?,
                                _ => #default_branch
                            }
                        }
                    }
                }
            }
        } else {
            // 無 #[env]：直接呼叫 T::load() 進行巢狀載入
            quote! {
                #ident: {
                    #ty::load(env_path)?
                }
            }
        };

        inits.push(init_one);
    }

    let expanded = quote! {
        impl crate::FromEnv for #name {
            fn load(env_path: &std::path::Path) -> Result<Self, crate::CfgError> {
                // Try to load .env file if it exists, but don't fail if it doesn't
                let _ = dotenvy::from_path(env_path);

                Ok(Self {
                    #(#inits),*
                })
            }
        }
    };

    expanded.into()
}

/// 若是 Vec<T> 回傳 Some(T)，否則 None
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
