use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    DeriveInput, Error as SynError, Expr, Path, parse_macro_input,
    token::Union,
};

use crate::intermediate::SyntaxType;

mod attributes;
mod components;
mod intermediate;

enum SyntaxError {
    ExpectedStringLit(Expr),
    ExpectedStringLitOrDual(Expr),
    ExpectedTraitPath(Expr),
    UnexpectedAttributeArg(Path),
    UnexpectedContentExpr(Expr),
    Union(Union),
}

#[proc_macro_derive(SyntaxFmt, attributes(syntax))]
pub fn derive_syntax_fmt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ty = match SyntaxType::parse(&input) {
        Ok(ty) => ty,
        Err(e) => {
            return match e {
                SyntaxError::ExpectedStringLit(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected string literal"),
                SyntaxError::ExpectedStringLitOrDual(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected string literal or tuple of two string literals (\"normal\", \"pretty\")"),
                SyntaxError::ExpectedTraitPath(t) =>
                    SynError::new_spanned(t, "syntaxfmt argument expected trait path"),
                SyntaxError::UnexpectedAttributeArg(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected attribute argument"),
                SyntaxError::UnexpectedContentExpr(t) =>
                    SynError::new_spanned(t, "syntaxfmt unexpected content expression"),
                SyntaxError::Union(t) =>
                    SynError::new_spanned(t, "syntaxfmt cannot be derived for unions"),
            }.to_compile_error().into();
        }
    };

    ty.to_token_stream().into()
}

// let fmt_body = match &input.data {
//     Data::Struct(data_struct) => generate_struct_fmt(&data_struct.fields),
//     Data::Enum(data_enum) => generate_enum_fmt(name, &data_enum.variants),
//     Data::Union(_) => {
//         return SynError::new_spanned(name, "SyntaxFmt cannot be derived for unions")
//             .to_compile_error()
//             .into();
//     }
// };

// let fmt_body = wrap_with_outer_format(fmt_body, &outer_format);

// // Wrap fmt_body with push_delim/pop_delim if type-level delims are specified
// let fmt_body = if delim.is_some() || pretty_delim.is_some() {
//     let d = delim.unwrap_or_else(|| ",".to_string());
//     let pd = pretty_delim.unwrap_or_else(|| ", ".to_string());
//     quote! {
//         f.push_delim(#d, #pd);
//         let __result = (|| -> ::std::fmt::Result { #fmt_body })();
//         f.pop_delim();
//         __result
//     }
// } else {
//     fmt_body
// };

// generics.params.push(syn::parse_quote! { __SyntaxFmtState });

// let where_clause = build_where_clause(&mut generics, &field_types, state_bound.as_ref());
// let (impl_generics_with_state, _, _) = generics.split_for_impl();

// let expanded = quote! {
//     impl #impl_generics_with_state ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for #name #ty_generics #where_clause {
//         fn syntax_fmt(&self, f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>) -> ::std::fmt::Result {
//             #fmt_body
//         }
//     }
// };

/*
fn build_where_clause(
    generics: &mut syn::Generics,
    field_types: &[syn::Type],
    state_bound: Option<&syn::TraitBound>,
) -> syn::WhereClause {
    let mut where_clause = generics.make_where_clause().clone();

    if let Some(bound) = state_bound {
        where_clause.predicates.push(syn::parse_quote! {
            __SyntaxFmtState: #bound
        });
    }

    for field_ty in field_types {
        where_clause.predicates.push(syn::parse_quote! {
            #field_ty: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState>
        });
    }
    where_clause
}

fn collect_field_types(data: &Data) -> Vec<syn::Type> {
    let mut types = Vec::new();
    match data {
        Data::Struct(data_struct) => collect_struct_field_types(&data_struct.fields, &mut types),
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                collect_struct_field_types(&variant.fields, &mut types);
            }
        }
        Data::Union(_) => {}
    }
    types
}

fn collect_struct_field_types(fields: &Fields, types: &mut Vec<syn::Type>) {
    for field in fields.iter() {
        let attrs = parse_field_attrs(&field.attrs);
        if attrs.skip {
            continue;
        }

        let ty = extract_option_inner(&field.ty);
        types.push(ty);
    }
}

fn extract_option_inner(ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty.clone();
                    }
                }
            }
        }
    }
    ty.clone()
}

fn parse_pretty_string_attrs(attrs: &[syn::Attribute], name: &str) -> PrettyString {
    let mut result = PrettyString::default();

    parse_syntax_attrs(attrs, |meta| {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if let Some(s) = extract_str_literal(value) {
                if path.is_ident(name) {
                    result.normal = Some(s);
                } else if path.is_ident(pretty_name) {
                    result.pretty = Some(s);
                }
            }
        }
    });

    result
}

fn parse_delimiters(attrs: &[syn::Attribute]) -> (Option<String>, Option<String>) {
    let result = parse_pretty_string_attrs(attrs, "delim", "pretty_delim");
    (result.normal, result.pretty)
}

fn parse_outer_format(attrs: &[syn::Attribute]) -> PrettyString {
    parse_pretty_string_attrs(attrs, "format")
}

fn parse_state_bound(attrs: &[syn::Attribute]) -> Option<syn::TraitBound> {
    let mut state_bound = None;

    parse_syntax_attrs(attrs, |meta| {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if path.is_ident("state_bound") {
                if let Some(s) = extract_str_literal(value) {
                    if let Ok(bound) = syn::parse_str::<syn::TraitBound>(&s) {
                        state_bound = Some(bound);
                    }
                }
            }
        }
    });

    state_bound
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut field_attrs = FieldAttrs::default();

    parse_syntax_attrs(attrs, |meta| match meta {
        Meta::NameValue(MetaNameValue { path, value, .. }) => {
            if path.is_ident("content") {
                field_attrs.content = Some(value.clone());
            } else if let Some(s) = extract_str_literal(value) {
                if path.is_ident("format") {
                    field_attrs.format.normal = Some(s);
                } else if path.is_ident("pretty_format") {
                    field_attrs.format.pretty = Some(s);
                } else if path.is_ident("delim") {
                    field_attrs.delim.normal = Some(s);
                } else if path.is_ident("pretty_delim") {
                    field_attrs.delim.pretty = Some(s);
                } else if path.is_ident("empty_suffix") {
                    field_attrs.empty_suffix = Some(s);
                }
            }
        }
        Meta::Path(path) => {
            if path.is_ident("skip") {
                field_attrs.skip = true;
            } else if path.is_ident("indent_region") {
                field_attrs.indent_region = true;
            } else if path.is_ident("indent") {
                field_attrs.indent = true;
            }
        }
        _ => {}
    });

    field_attrs
}

fn parse_syntax_attrs(attrs: &[syn::Attribute], mut f: impl FnMut(&Meta)) {
    for attr in attrs {
        if attr.path().is_ident("syntax") {
            if let Ok(meta_list) = attr.meta.require_list() {
                if let Ok(nested_list) = meta_list.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                ) {
                    for nested in &nested_list {
                        f(nested);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct PrettyString {
    normal: Option<String>,
    pretty: Option<String>,
}

impl PrettyString {
    fn get_pair(&self) -> (String, String) {
        let normal = self.normal.as_deref().unwrap_or("");
        let pretty = self
            .pretty
            .as_deref()
            .or(self.normal.as_deref())
            .unwrap_or("");
        (normal.to_string(), pretty.to_string())
    }

    fn get_delim_pair(&self) -> Option<(String, String)> {
        if self.normal.is_none() && self.pretty.is_none() {
            return None;
        }
        let normal = self.normal.as_deref().unwrap_or(",");
        let pretty = self.pretty.as_deref().unwrap_or(", ");
        Some((normal.to_string(), pretty.to_string()))
    }
}

#[derive(Default)]
struct FieldAttrs {
    format: PrettyString,
    delim: PrettyString,
    content: Option<syn::Expr>,
    empty_suffix: Option<String>,
    indent_region: bool,
    indent: bool,
    skip: bool,
}

fn split_format_string(format_str: &str) -> (&str, &str, bool) {
    if let Some(pos) = format_str.find("{content}") {
        (&format_str[..pos], &format_str[pos + 9..], true)
    } else {
        (format_str, "", false)
    }
}

fn generate_default_content(
    field_expr: &proc_macro2::TokenStream,
    content_expr: Option<&syn::Expr>,
) -> proc_macro2::TokenStream {
    if let Some(content_fn) = content_expr {
        return quote! { (#content_fn)(&#field_expr, f)?; };
    }

    quote! { #field_expr.syntax_fmt(f)?; }
}

fn expand_format_string(
    format_str: &str,
    field_expr: &proc_macro2::TokenStream,
    content_expr: Option<&syn::Expr>,
) -> proc_macro2::TokenStream {
    let (before, after, has_placeholder) = split_format_string(format_str);
    let mut statements = Vec::new();

    if !before.is_empty() {
        statements.push(quote! { write!(f, #before)?; });
    }

    if has_placeholder {
        statements.push(generate_default_content(field_expr, content_expr));
    }

    if !after.is_empty() {
        statements.push(quote! { write!(f, #after)?; });
    }

    quote! { #(#statements)* }
}

fn pretty_conditional(
    normal: proc_macro2::TokenStream,
    pretty: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        if f.is_pretty() {
            #pretty
        } else {
            #normal
        }
    }
}

fn wrap_with_outer_format(
    fmt_body: proc_macro2::TokenStream,
    outer_format: &PrettyString,
) -> proc_macro2::TokenStream {
    if outer_format.normal.is_none() && outer_format.pretty.is_none() {
        return fmt_body;
    }

    let (normal_fmt, pretty_fmt) = outer_format.get_pair();

    let wrap_body = |format_str: &str| -> proc_macro2::TokenStream {
        let (before, after, has_placeholder) = split_format_string(format_str);

        if !has_placeholder {
            return quote! {
                write!(f, #format_str)?;
                #fmt_body
            };
        }

        if before.is_empty() && after.is_empty() {
            return fmt_body.clone();
        }

        if after.is_empty() {
            return quote! {
                write!(f, #before)?;
                #fmt_body
            };
        }

        quote! {
            write!(f, #before)?;
            (|| -> ::std::fmt::Result { #fmt_body })()?;
            write!(f, #after)?;
            Ok(())
        }
    };

    if normal_fmt == pretty_fmt {
        wrap_body(&normal_fmt)
    } else {
        pretty_conditional(wrap_body(&normal_fmt), wrap_body(&pretty_fmt))
    }
}

fn generate_format_output(
    field_expr: &proc_macro2::TokenStream,
    format: &PrettyString,
    content_expr: Option<&syn::Expr>,
) -> proc_macro2::TokenStream {
    // No format specified - use default
    if format.normal.is_none() && format.pretty.is_none() {
        return generate_default_content(field_expr, content_expr);
    }

    let (normal_fmt, pretty_fmt) = format.get_pair();

    // Only pretty_format specified
    if format.normal.is_none() {
        let default_content = generate_default_content(field_expr, content_expr);
        let pretty_write = expand_format_string(&pretty_fmt, field_expr, content_expr);
        return quote! {
            if f.is_pretty() {
                #pretty_write
            } else {
                #default_content
            }
        };
    }

    // Normal format (with optional different pretty format)
    let normal_write = expand_format_string(&normal_fmt, field_expr, content_expr);

    if normal_fmt == pretty_fmt {
        normal_write
    } else {
        let pretty_write = expand_format_string(&pretty_fmt, field_expr, content_expr);
        pretty_conditional(normal_write, pretty_write)
    }
}

fn generate_struct_fmt(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => generate_named_fields_fmt(&fields_named.named),
        Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
            let field = fields_unnamed.unnamed.first().unwrap();
            let attrs = parse_field_attrs(&field.attrs);
            let format_output =
                generate_format_output(&quote! { self.0 }, &attrs.format, attrs.content.as_ref());
            quote! {
                #format_output
                Ok(())
            }
        }
        Fields::Unnamed(_) | Fields::Unit => quote! { Ok(()) },
    }
}

fn generate_named_fields_fmt(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> proc_macro2::TokenStream {
    let mut statements = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let attrs = parse_field_attrs(&field.attrs);

        if attrs.skip {
            continue;
        }

        if is_type_ident(&field.ty, "Option") {
            let field_expr = quote! { #field_name };
            let format_output =
                generate_format_output(&field_expr, &attrs.format, attrs.content.as_ref());
            statements.push(quote! {
                if let Some(#field_name) = &self.#field_name {
                    #format_output
                }
            });
        } else {
            let field_expr = quote! { self.#field_name };
            let mut field_statements = Vec::new();

            if let Some((delim, pretty_delim)) = attrs.delim.get_delim_pair() {
                field_statements.push(quote! {
                    f.push_delim(#delim, #pretty_delim);
                });
            }

            if attrs.indent {
                field_statements.push(quote! {
                    if f.is_pretty() {
                        f.indent()?;
                    }
                });
            }

            if attrs.indent_region {
                field_statements.push(quote! {
                    if f.is_pretty() {
                        f.inc_indent();
                    }
                });
            }

            let format_output =
                generate_format_output(&field_expr, &attrs.format, attrs.content.as_ref());

            field_statements.push(format_output);

            if attrs.indent_region {
                field_statements.push(quote! {
                    if f.is_pretty() {
                        f.dec_indent();
                    }
                });
            }

            if attrs.delim.normal.is_some() || attrs.delim.pretty.is_some() {
                field_statements.push(quote! {
                    f.pop_delim();
                });
            }

            if let Some(empty_suffix) = &attrs.empty_suffix {
                statements.push(quote! {
                    if self.#field_name.is_empty() {
                        write!(f, #empty_suffix)?;
                    } else {
                        #(#field_statements)*
                    }
                });
            } else {
                statements.extend(field_statements);
            }
        }
    }

    statements.push(quote! { Ok(()) });
    quote! { #(#statements)* }
}

fn generate_enum_fmt(
    name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> proc_macro2::TokenStream {
    let match_arms: Vec<_> = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let attrs = parse_field_attrs(&variant.attrs);

        match &variant.fields {
            Fields::Named(_) => {
                quote! {
                    #name::#variant_name { .. } => todo!("Named enum variants not yet supported")
                }
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let format_output = generate_format_output(
                    &quote! { inner },
                    &attrs.format,
                    attrs.content.as_ref(),
                );
                quote! {
                    #name::#variant_name(inner) => { #format_output Ok(()) }
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    #name::#variant_name(..) => todo!("Multi-field tuple variants not yet supported")
                }
            }
            Fields::Unit => {
                if attrs.format.normal.is_some() || attrs.format.pretty.is_some() {
                    let format_output = generate_format_output(
                        &quote! { "" },
                        &attrs.format,
                        attrs.content.as_ref(),
                    );
                    quote! { #name::#variant_name => { #format_output Ok(()) } }
                } else {
                    let lower_name = variant_name.to_string().to_lowercase();
                    quote! { #name::#variant_name => write!(f, #lower_name) }
                }
            }
        }
    }).collect();

    quote! {
        match self {
            #(#match_arms,)*
        }
    }
}

fn is_type_ident(ty: &syn::Type, ident_name: &str) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == ident_name;
        }
    }
    false
}
*/
