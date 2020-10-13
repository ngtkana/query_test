use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Token};

#[proc_macro_attribute]
pub fn query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &item_struct.ident;
    let generic_params = &item_struct.generics.params;
    let generic_params_without_bounds = item_struct
        .generics
        .params
        .iter()
        .map(|param| {
            use syn::GenericParam;
            match param {
                GenericParam::Type(type_param) => GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: type_param.ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                    eq_token: None,
                    default: None,
                }),
                GenericParam::Lifetime(lifetime) => GenericParam::Lifetime(syn::LifetimeDef {
                    attrs: Vec::new(),
                    lifetime: lifetime.lifetime.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                }),
                GenericParam::Const(_const_param) => unimplemented!(),
            }
        })
        .collect::<Punctuated<_, Token![,]>>();

    let type_bare_fn = syn::parse_macro_input!(attr as syn::TypeBareFn);
    let params = type_bare_fn.inputs;
    let output: Box<syn::Type> = match type_bare_fn.output {
        syn::ReturnType::Default => {
            let token_stream: TokenStream = quote! {()}.into();
            Box::new(syn::parse_macro_input!(token_stream as syn::Type))
        }
        syn::ReturnType::Type(_, boxed_type) => boxed_type,
    };

    (quote! {
        #item_struct
        impl<#generic_params> Query for #struct_name<#generic_params_without_bounds> {
            type Param = (#params);
            type Output = #output;
            const NAME: &'static str = stringify!(#struct_name);
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn help_material(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &item_struct.ident;
    let generic_params = &item_struct.generics.params;
    let generic_params_without_bounds = item_struct
        .generics
        .params
        .iter()
        .map(|param| {
            use syn::GenericParam;
            match param {
                GenericParam::Type(type_param) => GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: type_param.ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                    eq_token: None,
                    default: None,
                }),
                GenericParam::Lifetime(lifetime) => GenericParam::Lifetime(syn::LifetimeDef {
                    attrs: Vec::new(),
                    lifetime: lifetime.lifetime.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                }),
                GenericParam::Const(_const_param) => unimplemented!(),
            }
        })
        .collect::<Punctuated<_, Token![,]>>();
    let value = syn::parse_macro_input!(attr as syn::Type);

    (quote! {
        #item_struct
        impl<#generic_params> HelpMaterial for #struct_name<#generic_params_without_bounds> {
            type Value = #value;
        }
    })
    .into()
}
