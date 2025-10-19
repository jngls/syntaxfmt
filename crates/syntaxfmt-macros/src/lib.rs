use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, MetaNameValue,
};

#[proc_macro_derive(SyntaxFmt, attributes(syntax))]
pub fn derive_syntax_fmt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut generics = input.generics.clone();
    let (_, ty_generics, _) = input.generics.split_for_impl();

    let (delim, pretty_delim) = parse_delimiters(&input.attrs);
    let state_bound = parse_state_bound(&input.attrs);
    let outer_format = parse_outer_format(&input.attrs);
    let field_types = collect_field_types(&input.data);

    let fmt_body = match &input.data {
        Data::Struct(data_struct) => generate_struct_fmt(&data_struct.fields),
        Data::Enum(data_enum) => generate_enum_fmt(name, &data_enum.variants),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "SyntaxFmt cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let fmt_body = wrap_with_outer_format(fmt_body, &outer_format);

    let delim_const = delim.map(|d| quote! { const DELIM: &'static str = #d; });
    let pretty_delim_const = pretty_delim.map(|d| quote! { const PRETTY_DELIM: &'static str = #d; });

    generics.params.push(syn::parse_quote! { __SyntaxFmtState });

    let where_clause = build_where_clause(&mut generics, &field_types, state_bound.as_ref());
    let (impl_generics_with_state, _, _) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics_with_state ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for #name #ty_generics #where_clause {
            #delim_const
            #pretty_delim_const

            fn syntax_fmt(&self, ctx: &mut ::syntaxfmt::SyntaxFmtContext<'_, '_, __SyntaxFmtState>) -> ::std::fmt::Result {
                #fmt_body
            }
        }
    };

    TokenStream::from(expanded)
}

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
    let process_field = |field: &syn::Field| {
        let attrs = parse_field_attrs(&field.attrs);
        if !attrs.skip && !is_type_ident(&field.ty, "bool") {
            types.push(extract_option_inner(&field.ty));
        }
    };

    match fields {
        Fields::Named(fields) => fields.named.iter().for_each(process_field),
        Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                types.push(extract_option_inner(&field.ty));
            }
        }
        Fields::Unit => {}
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

// Helper to extract string literal from Meta::NameValue
fn extract_str_literal(value: &syn::Expr) -> Option<String> {
    if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Str(s), .. }) = value {
        Some(s.value())
    } else {
        None
    }
}

// Helper to parse two string attributes into a PrettyString
fn parse_pretty_string_attrs(
    attrs: &[syn::Attribute],
    normal_name: &str,
    pretty_name: &str,
) -> PrettyString {
    let mut result = PrettyString::default();

    parse_syntax_attrs(attrs, |meta| {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if let Some(s) = extract_str_literal(value) {
                if path.is_ident(normal_name) {
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
    parse_pretty_string_attrs(attrs, "format", "pretty_format")
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
                }
            }
        }
        Meta::Path(path) => {
            if path.is_ident("skip") {
                field_attrs.skip = true;
            } else if path.is_ident("indent_inc") {
                field_attrs.indent_inc = true;
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
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
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
        let pretty = self.pretty.as_deref().or(self.normal.as_deref()).unwrap_or("");
        (normal.to_string(), pretty.to_string())
    }

    fn has_content(&self) -> bool {
        self.normal.is_some() || self.pretty.is_some()
    }
}

#[derive(Default)]
struct FieldAttrs {
    format: PrettyString,
    content: Option<syn::Expr>,
    indent_inc: bool,
    indent: bool,
    skip: bool,
}

// Split format string by {content} placeholder, returning (before, after, has_placeholder)
fn split_format_string(format_str: &str) -> (&str, &str, bool) {
    if let Some(pos) = format_str.find("{content}") {
        (&format_str[..pos], &format_str[pos + 9..], true)
    } else {
        (format_str, "", false)
    }
}

// Build write statements for format string parts
fn build_format_statements(
    before: &str,
    after: &str,
    has_placeholder: bool,
    field_expr: &proc_macro2::TokenStream,
    content_expr: Option<&syn::Expr>,
) -> Vec<proc_macro2::TokenStream> {
    let mut statements = Vec::new();

    if !before.is_empty() {
        statements.push(quote! { write!(ctx, #before)?; });
    }

    if has_placeholder {
        if let Some(content_fn) = content_expr {
            statements.push(quote! { (#content_fn)(&#field_expr, ctx)?; });
        } else {
            statements.push(quote! { #field_expr.syntax_fmt(ctx)?; });
        }
    }

    if !after.is_empty() {
        statements.push(quote! { write!(ctx, #after)?; });
    }

    statements
}

fn expand_format_string(
    format_str: &str,
    field_expr: &proc_macro2::TokenStream,
    content_expr: Option<&syn::Expr>,
) -> proc_macro2::TokenStream {
    let (before, after, has_placeholder) = split_format_string(format_str);
    let statements = build_format_statements(before, after, has_placeholder, field_expr, content_expr);
    quote! { #(#statements)* }
}

// Generate conditional based on pretty mode
fn pretty_conditional(
    normal: proc_macro2::TokenStream,
    pretty: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        if ctx.is_pretty() {
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
    if !outer_format.has_content() {
        return fmt_body;
    }

    let (normal_fmt, pretty_fmt) = outer_format.get_pair();

    let wrap_body = |format_str: &str| -> proc_macro2::TokenStream {
        let (before, after, has_placeholder) = split_format_string(format_str);

        if !has_placeholder {
            return quote! {
                write!(ctx, #format_str)?;
                #fmt_body
            };
        }

        if after.is_empty() {
            return if before.is_empty() {
                fmt_body.clone()
            } else {
                quote! {
                    write!(ctx, #before)?;
                    #fmt_body
                }
            };
        }

        // Have suffix, need to execute body and then add suffix
        quote! {
            write!(ctx, #before)?;
            (|| -> ::std::fmt::Result { #fmt_body })()?;
            write!(ctx, #after)?;
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
    if !format.has_content() {
        return if let Some(content_fn) = content_expr {
            quote! { (#content_fn)(&#field_expr, ctx)?; }
        } else {
            quote! { #field_expr.syntax_fmt(ctx)?; }
        };
    }

    let (normal_fmt, pretty_fmt) = format.get_pair();
    let normal_write = expand_format_string(&normal_fmt, field_expr, content_expr);
    let pretty_write = expand_format_string(&pretty_fmt, field_expr, content_expr);

    if normal_fmt == pretty_fmt {
        normal_write
    } else {
        pretty_conditional(normal_write, pretty_write)
    }
}

fn generate_struct_fmt(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => generate_named_fields_fmt(&fields_named.named),
        Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
            generate_tuple_field_fmt(fields_unnamed.unnamed.first().unwrap())
        }
        Fields::Unnamed(_) | Fields::Unit => quote! { Ok(()) },
    }
}

// Helper to wrap code in pretty-mode conditional
fn wrap_in_pretty_check(code: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        if ctx.is_pretty() {
            #code
        }
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

        if is_type_ident(&field.ty, "bool") {
            let format_output = generate_format_output(
                &quote! { &true },
                &attrs.format,
                attrs.content.as_ref(),
            );

            statements.push(quote! {
                if self.#field_name {
                    #format_output
                }
            });
        } else if is_type_ident(&field.ty, "Option") {
            let field_expr = quote! { #field_name };
            let format_output = generate_format_output(
                &field_expr,
                &attrs.format,
                attrs.content.as_ref(),
            );

            statements.push(quote! {
                if let Some(#field_name) = &self.#field_name {
                    #format_output
                }
            });
        } else {
            let field_expr = quote! { self.#field_name };
            let format_output = generate_format_output(
                &field_expr,
                &attrs.format,
                attrs.content.as_ref(),
            );

            if attrs.indent {
                statements.push(wrap_in_pretty_check(quote! { ctx.indent(Self::INDENT)?; }));
            }

            if attrs.indent_inc {
                statements.push(wrap_in_pretty_check(quote! { ctx.inc_indent(); }));
            }

            statements.push(format_output);

            if attrs.indent_inc {
                statements.push(wrap_in_pretty_check(quote! { ctx.dec_indent(); }));
            }
        }
    }

    statements.push(quote! { Ok(()) });
    quote! { #(#statements)* }
}

fn generate_tuple_field_fmt(field: &syn::Field) -> proc_macro2::TokenStream {
    let attrs = parse_field_attrs(&field.attrs);
    let format_output = generate_format_output(
        &quote! { self.0 },
        &attrs.format,
        attrs.content.as_ref(),
    );

    quote! {
        #format_output
        Ok(())
    }
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
                if attrs.format.has_content() {
                    let format_output = generate_format_output(
                        &quote! { "" },
                        &attrs.format,
                        attrs.content.as_ref(),
                    );
                    quote! { #name::#variant_name => { #format_output Ok(()) } }
                } else {
                    let lower_name = variant_name.to_string().to_lowercase();
                    quote! { #name::#variant_name => write!(ctx, #lower_name) }
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
