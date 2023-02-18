#![doc = include_str!("../README.md")]

use proc_macro2::TokenStream;
use syn::{
    parse_macro_input,
    DeriveInput,
    parse::Error,
    spanned::Spanned
};
use quote::quote;

#[cfg(test)]
mod tests;

/// This macro generates an implementation of the `Keyed` trait for structs
/// that have a single `Key<T>` defined
#[proc_macro_derive(Entity)]
pub fn derive_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_entity(&input).unwrap_or_else(|err| err.to_compile_error()).into()
}

/// This macro generates an implementation of the `Label` trait for structs
/// that have a field marked with `#[label]` attribute
#[proc_macro_derive(Label, attributes(label))]
pub fn derive_label(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_label(&input).unwrap_or_else(|err| err.to_compile_error()).into()
}

/// Returns the implementation of the `Keyed` trait
fn impl_entity(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (key_type, key_expr) = match input.data {
        syn::Data::Struct(ref body) => {
            let (ty, ident) = single_key(&body.fields)?;
            (
                ty,
                quote! {
                    Ok(&self.#ident)
                }
            )
        },
        _ => panic!("#[derive(Entity)] can only be used on structs"),
    };

    Ok(
        quote!{
            #[automatically_derived]
            impl #impl_generics ::dbent::Keyed for #name #ty_generics #where_clause {
                type KeyType = #key_type;

                #[inline]
                fn key(&self) -> ::dbent::Result<&Key<Self::KeyType>> {
                    #key_expr
                }
            }
        }
    )
}

/// Returns the implementation of the `Label` trait
fn impl_label(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (label_type, label_expr) = match input.data {
        syn::Data::Struct(ref body) => {
            let (ty, ident) = single_label(&body.fields)?;
            (
                ty,
                quote! {
                    Ok(&self.#ident)
                }
            )
        },
        _ => panic!("#[derive(Label)] can only be used on structs"),
    };

    Ok(
        quote!{
            #[automatically_derived]
            impl #impl_generics ::dbent::Label for #name #ty_generics #where_clause {
                type LabelType = #label_type;

                #[inline]
                fn label(&self) -> ::dbent::Result<&Self::LabelType> {
                    #label_expr
                }
            }
        }
    )
}

/// Returns the key type and name if the first field found in the struct is a `Key<T>`
fn single_key(fields: &syn::Fields) -> Result<(TokenStream, TokenStream), Error> {
    let (ty, ident) = match fields {
        syn::Fields::Named(fields) => {
            let field = fields
                .named
                .first()
                .ok_or_else(|| Error::new(fields.span(), "#[derive(Entity)] needs at least a single Key field defined"))?;

            match &field.ty {
                syn::Type::Path(typepath) => {
                    let seg = typepath
                        .path
                        .segments
                        .last()
                        .ok_or_else(|| Error::new(field.span(), "#[derive(Entity)] needs at least a single Key field defined; no segments found"))?;

                    if seg.ident != "Key" {
                        return Err(Error::new(field.span(), "#[derive(Entity)] needs the first field to be a Key; aliasing the Key to something else breaks the macro"));
                    }

                    (argument_type(field, &seg.arguments)?, &field.ident)
                },
                _ => return Err(Error::new(field.span(), "#[derive(Entity)] needs a single Key field defined as the first field in the struct")),
            }
        },
        _ => return Err(Error::new(fields.span(), "#[derive(Entity)] can only be used on structs with named fields")),
    };

    Ok((ty, quote! { #ident }))
}

/// Returns the generic argument type for the key
fn argument_type(field: &syn::Field, args: &syn::PathArguments) -> Result<TokenStream, Error> {
    match args {
        syn::PathArguments::AngleBracketed(angle_args) => {
            let arg = angle_args
                .args
                .first()
                .ok_or_else(|| Error::new(field.span(), "#[derive(Entity)] needs the Key to define a single generic argument type"))?;

            match arg {
                syn::GenericArgument::Type(gen_ty) => Ok(quote! { #gen_ty }),
                _ => Err(Error::new(field.span(), "#[derive(Entity)] only supports types as generic argument for Key")),
            }
        },
        _ => Err(Error::new(field.span(), "#[derive(Entity)] needs the Key to define a single generic argument type")),
    }
}


/// Returns the field type and name if a single field marked with `#[label]` is found
fn single_label(fields: &syn::Fields) -> Result<(TokenStream, TokenStream), Error> {
    let (ty, ident) = match fields {
        syn::Fields::Named(fields) => {
            let label_fields = fields
                .named
                .iter()
                .filter(marked_with_label)
                .collect::<Vec<_>>();

            if label_fields.len() != 1 {
                return Err(Error::new(fields.span(), "#[derive(Label)] needs to have 1 field marked with #[label]"));
            }

            let field = label_fields[0];

            (&field.ty, &field.ident)
        },
        _ => return Err(Error::new(fields.span(), "#[derive(Label)] can only be used on structs with named fields")),
    };

    Ok((quote! { #ty }, quote! { #ident }))
}

/// Returns true if this field is marked with `#[label]`
fn marked_with_label(field: &&syn::Field) -> bool {
    for attr in &field.attrs {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "label" {
                return true
            }
        }
    }

    false
}
