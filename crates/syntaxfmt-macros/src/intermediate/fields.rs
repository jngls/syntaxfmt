use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Result as SynResult, Type,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

use crate::attributes::{
    args::FieldArgs,
    content::{Content, FieldKind, Skipped, ToConditionalTokens},
};

#[derive(Debug, Clone)]
pub struct SyntaxFieldNamed {
    pub args: FieldArgs,
    pub name: Ident,
}

impl SyntaxFieldNamed {
    pub fn from_field<'a>(types: &mut Vec<&'a Type>, input: &'a Field) -> SynResult<Self> {
        let ty = &input.ty;
        let args = FieldArgs::from_attributes(&input.attrs)?;
        let name = input.ident.clone().unwrap();
        if !args.skipped() {
            types.push(ty);
        }
        Ok(Self { args, name })
    }

    pub fn decl(&self) -> Ident {
        if self.args.skipped() {
            Ident::new("_", self.name.span())
        } else {
            self.name.clone()
        }
    }
}

impl ToTokens for SyntaxFieldNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.args.skipped() {
            return;
        }

        let name = &self.name;

        let default_content = Content::Tokens(quote! { #name.syntax_fmt(f)?; });

        let content = self.args.to_conditional_tokens(FieldKind::Field(name.clone()), &default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxFieldUnnamed {
    pub args: FieldArgs,
    pub name: Ident,
}

impl SyntaxFieldUnnamed {
    pub fn from_field<'a>(types: &mut Vec<&'a Type>, input: &'a Field) -> SynResult<Self> {
        let args = FieldArgs::from_attributes(&input.attrs)?;
        let ty = &input.ty;
        if !args.skipped() {
            types.push(ty);
        };
        Ok(Self {
            args,
            name: Ident::new("_", input.ident.span()),
        })
    }

    pub fn decl<'a>(&'a self) -> &'a Ident {
        &self.name
    }

    pub fn renamed(mut self, name: Ident) -> SyntaxFieldUnnamed {
        self.name = name;
        self
    }
}

impl ToTokens for SyntaxFieldUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.args.skipped() {
            return;
        }

        let name = &self.name;

        let default_content = Content::Tokens(quote! { #name.syntax_fmt(f)?; });

        let content = self.args.to_conditional_tokens(FieldKind::Field(name.clone()), &default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxFieldsNamed {
    pub fields: Vec<SyntaxFieldNamed>,
}

impl SyntaxFieldsNamed {
    fn from_fields_named<'a>(types: &mut Vec<&'a Type>, input: &'a FieldsNamed) -> SynResult<Self> {
        let mut fields = Vec::new();
        for field in &input.named {
            fields.push(SyntaxFieldNamed::from_field(types, field)?);
        }
        Ok(Self { fields })
    }

    pub fn decl(&self) -> Punctuated<Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            if !field.args.skipped() {
                decls.push(field.decl());
            }
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

#[derive(Debug, Default, Clone)]
pub struct SyntaxFieldsUnnamed {
    pub fields: Vec<SyntaxFieldUnnamed>,
}

impl SyntaxFieldsUnnamed {
    pub fn from_fields_unnamed<'a>(
        types: &mut Vec<&'a Type>,
        input: &'a FieldsUnnamed,
    ) -> SynResult<Self> {
        let mut fields = Vec::new();
        for (i, field) in input.unnamed.iter().enumerate() {
            let index = Ident::new(&format!("__{i}"), field.span());
            fields.push(SyntaxFieldUnnamed::from_field(types, field)?.renamed(index));
        }
        Ok(Self { fields })
    }

    pub fn decl(&self) -> Punctuated<Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            decls.push(field.decl().clone());
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

#[derive(Debug, Clone)]
pub enum SyntaxFieldsDecl {
    Named(Punctuated<Ident, Comma>),
    Unnamed(Punctuated<Ident, Comma>),
    Unit,
}

impl ToTokens for SyntaxFieldsDecl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SyntaxFieldsDecl::Named(inner) => {
                if inner.is_empty() {
                    tokens.extend(quote! { { .. } });
                } else {
                    tokens.extend(quote! { { #inner, .. } });
                }
            }
            SyntaxFieldsDecl::Unnamed(inner) => {
                tokens.extend(quote! { ( #inner ) });
            }
            SyntaxFieldsDecl::Unit => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxFields {
    Named(SyntaxFieldsNamed),
    Unnamed(SyntaxFieldsUnnamed),
    Unit,
}

impl SyntaxFields {
    pub fn from_fields<'a>(types: &mut Vec<&'a Type>, input: &'a Fields) -> SynResult<Self> {
        match &input {
            Fields::Named(fields_named) => Ok(Self::Named(SyntaxFieldsNamed::from_fields_named(
                types,
                fields_named,
            )?)),
            Fields::Unnamed(fields_unnamed) => Ok(Self::Unnamed(
                SyntaxFieldsUnnamed::from_fields_unnamed(types, fields_unnamed)?,
            )),
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
