use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote_spanned};
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Result as SynResult, Type,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

use crate::{
    attributes::{
        args::FieldArgs,
        content::{Content, Skipped, ToConditionalTokens},
    },
    intermediate::parse_type::ParseType,
};

#[derive(Debug, Clone)]
pub struct SyntaxFieldNamed {
    pub args: FieldArgs,
    pub name: Ident,
}

impl SyntaxFieldNamed {
    pub fn decl(&self) -> Ident {
        if self.args.skipped() {
            Ident::new("_", self.name.span())
        } else {
            self.name.clone()
        }
    }
}

impl<'a> ParseType<'a> for SyntaxFieldNamed {
    type Input = Field;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        let ty = &input.ty;
        let args = FieldArgs::from_attributes(&input.attrs)?;
        let name = input.ident.clone().unwrap();
        if !args.skipped() {
            types.push(ty);
        }
        Ok(Self { args, name })
    }
}

impl ToTokens for SyntaxFieldNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.args.skipped() {
            return;
        }

        let span = self.name.span();
        let name = &self.name;

        let default_content = Content::Tokens(quote_spanned! { span => #name.syntax_fmt(f)?; });

        let content = self.args.to_conditional_tokens(name, &default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxFieldUnnamed {
    pub args: FieldArgs,
    pub name: Ident,
}

impl SyntaxFieldUnnamed {
    pub fn decl<'a>(&'a self) -> &'a Ident {
        &self.name
    }

    pub fn renamed(mut self, name: Ident) -> SyntaxFieldUnnamed {
        self.name = name;
        self
    }
}

impl<'a> ParseType<'a> for SyntaxFieldUnnamed {
    type Input = Field;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
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
}

impl ToTokens for SyntaxFieldUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.args.skipped() {
            return;
        }

        let span = self.name.span();
        let name = &self.name;

        let default_content = Content::Tokens(quote_spanned! { span => #name.syntax_fmt(f)?; });

        let content = self.args.to_conditional_tokens(name, &default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxFieldsNamed {
    pub fields: Vec<SyntaxFieldNamed>,
}

impl SyntaxFieldsNamed {
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

impl<'a> ParseType<'a> for SyntaxFieldsNamed {
    type Input = FieldsNamed;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        let mut fields = Vec::new();
        for field in &input.named {
            fields.push(SyntaxFieldNamed::parse_type(types, field)?);
        }
        Ok(Self { fields })
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
    pub fn decl(&self) -> Punctuated<Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            decls.push(field.decl().clone());
        }
        decls
    }
}

impl<'a> ParseType<'a> for SyntaxFieldsUnnamed {
    type Input = FieldsUnnamed;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        let mut fields = Vec::new();
        for (i, field) in input.unnamed.iter().enumerate() {
            let index = Ident::new(&format!("__{i}"), field.span());
            fields.push(SyntaxFieldUnnamed::parse_type(types, field)?.renamed(index));
        }
        Ok(Self { fields })
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
                let span = inner.span();
                if inner.is_empty() {
                    tokens.extend(quote_spanned! { span => { .. } });
                } else {
                    tokens.extend(quote_spanned! { span => { #inner, .. } });
                }
            }
            SyntaxFieldsDecl::Unnamed(inner) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => ( #inner ) });
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
    pub fn decl(&self) -> SyntaxFieldsDecl {
        match self {
            SyntaxFields::Named(inner) => SyntaxFieldsDecl::Named(inner.decl()),
            SyntaxFields::Unnamed(inner) => SyntaxFieldsDecl::Unnamed(inner.decl()),
            SyntaxFields::Unit => SyntaxFieldsDecl::Unit,
        }
    }
}

impl<'a> ParseType<'a> for SyntaxFields {
    type Input = Fields;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        match &input {
            Fields::Named(fields_named) => Ok(Self::Named(SyntaxFieldsNamed::parse_type(
                types,
                fields_named,
            )?)),
            Fields::Unnamed(fields_unnamed) => Ok(Self::Unnamed(SyntaxFieldsUnnamed::parse_type(
                types,
                fields_unnamed,
            )?)),
            Fields::Unit => Ok(Self::Unit),
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
