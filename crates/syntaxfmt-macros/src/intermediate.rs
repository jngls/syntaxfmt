use proc_macro2::{TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, token::{Comma, Where}, Data, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Type, Variant, WhereClause
};

use crate::{
    SyntaxError,
    attributes::{SyntaxAttributes, SyntaxFieldAttributes, SyntaxTypeAttributes},
};

pub struct SyntaxFieldNamed {
    pub attrs: SyntaxFieldAttributes,
    pub name: Ident,
}

impl SyntaxFieldNamed {
    pub fn parse<'a>(types: &mut Vec<&'a Type>, field: &'a Field) -> Result<Self, SyntaxError> {
        let ty = &field.ty;
        let attrs = SyntaxFieldAttributes::parse(&field.attrs)?;
        let name = if attrs.skip {
            Ident::new("_", field.ident.span())
        } else {
            types.push(ty);
            field.ident.clone().unwrap()
        };
        println!("//     Parse named field: {name}: {}", ty.to_token_stream().to_string());
        Ok(Self { attrs, name })
    }

    pub fn decl(&self) -> &Ident {
        &self.name
    }
}

impl ToTokens for SyntaxFieldNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let name = &self.name;

        let (pre, post) = self.attrs.split_pre_post();

        println!("//     Gen named field: {name}");

        let content = self
            .attrs
            .content
            .as_ref()
            .map(|c| c.to_token_stream())
            .unwrap_or_else(|| quote! {
                #name.syntax_fmt(f)?;
            });

        tokens.extend(quote! {
            #pre
            #content
            #post
        })
    }
}

pub struct SyntaxFieldUnnamed {
    pub attrs: SyntaxFieldAttributes,
    pub index: Ident,
//    pub ty: &'a Type,
}

impl SyntaxFieldUnnamed {
    pub fn parse<'a>(types: &mut Vec<&'a Type>, field: &'a Field, index: Ident) -> Result<Self, SyntaxError> {
        let attrs = SyntaxFieldAttributes::parse(&field.attrs)?;
        let ty = &field.ty;
        let index = if attrs.skip {
            Ident::new("_", field.ident.span())
        } else {
            types.push(ty);
            index
        };
        println!("//     Parse unnamed field: {index}: {}", ty.to_token_stream().to_string());
        Ok(Self { attrs, index })
    }

    pub fn decl<'a>(&'a self) -> &'a Ident {
        &self.index
    }
}

impl ToTokens for SyntaxFieldUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let index = &self.index;

        let (pre, post) = self.attrs.split_pre_post();

        println!("//     Gen unnamed field: {index}");

        let content = self
            .attrs
            .content
            .as_ref()
            .map(|c| c.to_token_stream())
            .unwrap_or_else(|| quote! {
                #index.syntax_fmt(f)?;
            });

        tokens.extend(quote! {
            #pre
            #content
            #post
        })
    }
}

pub struct SyntaxFieldsNamed {
    pub fields: Vec<SyntaxFieldNamed>,
}

impl SyntaxFieldsNamed {
    pub fn parse<'a>(
        types: &mut Vec<&'a Type>,
        fields_named: &'a FieldsNamed,
    ) -> Result<Self, SyntaxError> {
        let mut fields = Vec::new();
        for field in &fields_named.named {
            fields.push(SyntaxFieldNamed::parse(types, field)?);
        }
        Ok(Self { fields })
    }

    pub fn decl(&self) -> Punctuated<&Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            decls.push(field.decl());
        }
        decls
    }
}

impl ToTokens for SyntaxFieldsNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for field in &self.fields {
            field.to_tokens(tokens);
        }
    }
}

pub struct SyntaxFieldsUnnamed {
    pub fields: Vec<SyntaxFieldUnnamed>,
}

impl SyntaxFieldsUnnamed {
    pub fn parse<'a>(
        types: &mut Vec<&'a Type>,
        fields_unnamed: &'a FieldsUnnamed,
    ) -> Result<Self, SyntaxError> {
        let mut fields = Vec::new();
        for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
            let index = Ident::new(&format!("__{i}"), field.span());
            fields.push(SyntaxFieldUnnamed::parse(types, field, index)?);
        }
        Ok(Self { fields })
    }

    pub fn decl(&self) -> Punctuated<&Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            decls.push(field.decl());
        }
        decls
    }
}

impl ToTokens for SyntaxFieldsUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for field in &self.fields {
            field.to_tokens(tokens);
        }
    }
}

pub enum SyntaxFieldsDecl<'a> {
    Named(Punctuated<&'a Ident, Comma>),
    Unnamed(Punctuated<&'a Ident, Comma>),
    Unit,
}

impl<'a> ToTokens for SyntaxFieldsDecl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SyntaxFieldsDecl::Named(inner) => {
                tokens.extend(quote! { { #inner } });
            }
            SyntaxFieldsDecl::Unnamed(inner) => {
                tokens.extend(quote! { ( #inner ) });
            }
            SyntaxFieldsDecl::Unit => {}
        }
    }
}

pub enum SyntaxFields {
    Named(SyntaxFieldsNamed),
    Unnamed(SyntaxFieldsUnnamed),
    Unit,
}

impl SyntaxFields {
    pub fn parse<'a>(types: &mut Vec<&'a Type>, fields: &'a Fields) -> Result<Self, SyntaxError> {
        match &fields {
            Fields::Named(fields_named) => {
                Ok(Self::Named(SyntaxFieldsNamed::parse(types, fields_named)?))
            }
            Fields::Unnamed(fields_unnamed) => Ok(Self::Unnamed(SyntaxFieldsUnnamed::parse(
                types,
                fields_unnamed,
            )?)),
            Fields::Unit => Ok(Self::Unit),
        }
    }

    pub fn decl(&self) -> SyntaxFieldsDecl {
        match self {
            SyntaxFields::Named(inner) => SyntaxFieldsDecl::Named(inner.decl()),
            SyntaxFields::Unnamed(inner) => SyntaxFieldsDecl::Unnamed(inner.decl()),
            SyntaxFields::Unit => SyntaxFieldsDecl::Unit,
        }
    }
}

impl ToTokens for SyntaxFields {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SyntaxFields::Named(inner) => inner.to_tokens(tokens),
            SyntaxFields::Unnamed(inner) => inner.to_tokens(tokens),
            SyntaxFields::Unit => {}
        }
    }
}

pub struct SyntaxVariantDecl<'a>(&'a Ident, SyntaxFieldsDecl<'a>);

impl<'a> ToTokens for SyntaxVariantDecl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.0;
        let fields_decl = &self.1;

        tokens.extend(quote! {
            Self::#name #fields_decl
        });
    }
}

pub struct SyntaxVariant {
    pub attrs: SyntaxFieldAttributes,
    pub fields: SyntaxFields,
    pub name: Ident,
}

impl SyntaxVariant {
    pub fn parse<'a>(types: &mut Vec<&'a Type>, variant: &'a Variant) -> Result<Self, SyntaxError> {
        let attrs = SyntaxFieldAttributes::parse(&variant.attrs)?;
        let fields = SyntaxFields::parse(types, &variant.fields)?;
        let name = variant.ident.clone();
        Ok(Self {
            attrs,
            fields,
            name,
        })
    }

    pub fn decl(&self) -> SyntaxVariantDecl {
        SyntaxVariantDecl(&self.name, self.fields.decl())
    }
}

impl ToTokens for SyntaxVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let fields = &self.fields;

        let decl = self.decl();

        println!("//         variant inner decl: {}", decl.to_token_stream().to_string());


        let (pre, post) = self.attrs.split_pre_post();

        let content = self
            .attrs
            .content
            .as_ref()
            .map(|c| c.to_token_stream())
            .unwrap_or_else(|| quote! {
                #fields
            });

        tokens.extend(quote! { #decl => {
            #pre
            #content
            #post
        } });
    }
}

pub struct SyntaxVariants {
    pub variants: Vec<SyntaxVariant>,
}

impl SyntaxVariants {
    pub fn parse<'a>(
        types: &mut Vec<&'a Type>,
        variants_iter: impl Iterator<Item = &'a Variant>,
    ) -> Result<Self, SyntaxError> {
        let mut variants = Vec::new();
        for variant in variants_iter {
            variants.push(SyntaxVariant::parse(types, variant)?);
        }
        Ok(Self { variants })
    }
}

impl ToTokens for SyntaxVariants {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for variant in &self.variants {
            variant.to_tokens(tokens);
        }
    }
}

pub enum SyntaxTypeKind {
    Struct(SyntaxFields),
    Enum(SyntaxVariants),
}

impl SyntaxTypeKind {
    pub fn parse<'a>(types: &mut Vec<&'a Type>, data: &'a Data) -> Result<Self, SyntaxError> {
        match data {
            Data::Struct(data_struct) => Ok(Self::Struct(SyntaxFields::parse(
                types,
                &data_struct.fields,
            )?)),
            Data::Enum(data_enum) => Ok(Self::Enum(SyntaxVariants::parse(
                types,
                data_enum.variants.iter(),
            )?)),
            Data::Union(data_union) => Err(SyntaxError::Union(data_union.union_token)),
        }
    }
}

impl ToTokens for SyntaxTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Struct(inner) => {
                let decl = inner.decl();
                println!("//         inner decl: {}", decl.to_token_stream().to_string());
                tokens.extend(quote! { let Self #decl = &self; });
                inner.to_tokens(tokens);
            }
            Self::Enum(inner) => {
                tokens.extend(quote! {
                    match self {
                        #inner
                        _ => {}
                    }
                });
            }
        }
    }
}

pub struct SyntaxType<'a> {
    pub attrs: SyntaxTypeAttributes,
    pub types: Vec<&'a Type>,
    pub kind: SyntaxTypeKind,
    pub generics: &'a Generics,
    pub name: &'a Ident,
}

impl<'a> SyntaxType<'a> {
    pub fn parse(input: &'a DeriveInput) -> Result<Self, SyntaxError> {
        let mut types = Vec::new();

        println!("// Parse type {}:\n/*\n{}\n*/\n", input.ident, input.to_token_stream());

        let attrs = SyntaxTypeAttributes::parse(&input.attrs)?;
        let kind = SyntaxTypeKind::parse(&mut types, &input.data)?;

        println!("");

        Ok(Self {
            attrs,
            types,
            kind,
            generics: &input.generics,
            name: &input.ident,
        })
    }

    fn split_generics(&self) -> (TokenStream2, TokenStream2, TokenStream2) {
        let mut impl_gen = self.generics.params.clone();
        let ty_gen = self.generics.params.clone();
        let mut where_clause = self.generics.where_clause.clone().unwrap_or(WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::new(),
        });

        impl_gen.push(parse_quote!(__SyntaxFmtState));

        if let Some(bound) = &self.attrs.state_bound {
            where_clause
                .predicates
                .push(syn::parse_quote! { __SyntaxFmtState: #bound });
        }

        for &field_ty in &self.types {
            where_clause
                .predicates
                .push(syn::parse_quote! { #field_ty: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> });
        }

        let where_clause = (!where_clause.predicates.is_empty()).then_some(where_clause);
        (
            impl_gen.to_token_stream(),
            ty_gen.to_token_stream(),
            where_clause.to_token_stream(),
        )
    }
}

impl<'a> ToTokens for SyntaxType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        println!("// Gen type: {}", self.name);

        let name = self.name;

        let (impl_gen, ty_gen, where_clause) = self.split_generics();

        let (pre, post) = self.attrs.split_pre_post();

        let content = self
            .attrs
            .content
            .as_ref()
            .map(|c| c.to_token_stream())
            .unwrap_or_else(|| self.kind.to_token_stream());

        tokens.extend(quote! {
            impl <#impl_gen> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for #name<#ty_gen> #where_clause {
                fn syntax_fmt(&self, f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>) -> ::std::fmt::Result {
                    #pre
                    #content
                    #post
                    Ok(())
                }
            }
        });

        println!("");
        println!("");
    }
}
