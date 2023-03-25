use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_derive(TypeHash, attributes(type_hash))]
/// Implement [`tysh::TypeHash`] trait for the struct.<br>
/// Use `#[type_hash(name = "name")]` to specify the internal name.
///
/// # Example
///
/// ```rust
/// use std::collections::hash_map::DefaultHasher;
///
/// # use tysh::TypeHash;
/// #
/// #[derive(TypeHash)]
/// pub struct A {
///     a: u8,
///     b: u16,
/// }
///
/// #[derive(TypeHash)]
/// #[type_hash(name = "A")]
/// pub struct B {
///     #[type_hash(name = "a")]
///     hoge: u8,
///     #[type_hash(name = "b")]
///     fuga: u16,
/// }
///
/// assert_eq!(
///     A::type_hash_one::<DefaultHasher>(),
///     B::type_hash_one::<DefaultHasher>(),
/// );
/// ```
///
/// Enum is also supported.
///
/// ```rust
/// # use tysh::TypeHash;
///
/// #[derive(TypeHash)]
/// enum A {
///     Hoge(String)
///     Fuag { x: i32, y: i32 },
///     Piyo,
/// }
/// ```
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let type_name = &input.ident;

    let content = match &input.data {
        syn::Data::Struct(v) => hashing_struct(&input, v),
        syn::Data::Enum(v) => hashing_enum(&input, v),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    };

    let output = quote! {
        impl ::tysh::TypeHash for #type_name {
            fn type_hash<H: ::core::hash::Hasher>(hasher: &mut H) {
                use ::core::hash::Hash;

                #content
            }
        }
    };

    output.into()
}

fn hashing_struct(input: &syn::DeriveInput, structure: &syn::DataStruct) -> TokenStream {
    let ident = parse_attrs(&input.attrs).unwrap_or(input.ident.to_string());
    let fields = structure.fields.iter().map(hashing_field);

    quote! {
        "@struct@".hash(hasher);
        #ident.hash(hasher);
        #(#fields)*
    }
}

fn hashing_enum(input: &syn::DeriveInput, enum_: &syn::DataEnum) -> TokenStream {
    let ident = parse_attrs(&input.attrs).unwrap_or(input.ident.to_string());
    let variants = enum_.variants.iter().map(hashing_variant);

    quote! {
        "@enum@".hash(hasher);
        #ident.hash(hasher);
        #(#variants)*
    }
}

fn hashing_variant(variant: &syn::Variant) -> TokenStream {
    let ident = parse_attrs(&variant.attrs).unwrap_or(variant.ident.to_string());
    let fields = variant.fields.iter().map(hashing_field);

    quote! {
        #ident.hash(hasher);
        #(#fields)*
    }
}

fn hashing_field(field: &syn::Field) -> TokenStream {
    let ident = parse_attrs(&field.attrs).unwrap_or(
        field
            .ident
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or("@ano@".into()),
    );
    let ty = field.ty.to_token_stream();

    quote! {
        "@field@".hash(hasher);
        #ident.hash(hasher);
        <#ty as ::tysh::TypeHash>::type_hash(hasher);
    }
}

fn parse_attrs(attrs: &[syn::Attribute]) -> Option<String> {
    let attrs = attrs
        .iter()
        .filter(|at| at.path().is_ident("type_hash"))
        .collect::<Vec<_>>();

    if attrs.len() > 1 {
        panic!("type_hash attribute can only be used once per field");
    }

    attrs.first().map(|v| match v.parse_args() {
        Ok(syn::Meta::NameValue(m)) => {
            if m.path.is_ident("name") {
                let syn::Expr::Lit(lit) = m.value else {
                    panic!("name attribute must be a string literal");
                };
                let syn::Lit::Str(lit) = lit.lit else {
                    panic!("name attribute must be a string literal");
                };
                lit.value()
            } else {
                panic!("invalid type_hash attribute: expected `name = \"...\"`");
            }
        }
        _ => {
            panic!("invalid type_hash attribute")
        }
    })
}
