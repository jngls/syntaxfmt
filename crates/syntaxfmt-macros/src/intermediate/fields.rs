use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote_spanned};
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Type, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

use crate::{
    components::{
        attributes::Attributes, content::Content
    }, intermediate::parse_type::ParseType, SyntaxError
};

#[derive(Debug, Clone)]
pub struct SyntaxFieldNamed {
    pub attrs: Attributes,
    pub attrs_else: Option<Attributes>,
    pub name: Ident,
}

impl SyntaxFieldNamed {
    pub fn decl(&self) -> Ident {
        if self.attrs.skip.is_some() {
            Ident::new("_", self.name.span())
        } else {
            self.name.clone()
        }
    }
}

impl<'a> ParseType<'a> for SyntaxFieldNamed {
    type Input = Field;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let ty = &input.ty;
        let attrs = Attributes::parse_for_field(&input.attrs)?;
        let attrs_else = Attributes::parse_for_field_else(&input.attrs)?;
        let name = input.ident.clone().unwrap();
        if attrs.skip.is_none() {
            types.push(ty);
        }
        Ok(Self { attrs, attrs_else, name })
    }
}

impl ToTokens for SyntaxFieldNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip.is_some() {
            return;
        }

        let span = self.name.span();
        let name = &self.name;

        let default_content = Content::Tokens(quote_spanned! { span => #name.syntax_fmt(f)?; });

        let content = Attributes::to_conditional_tokens(&self.attrs, &self.attrs_else, name, &default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxFieldUnnamed {
    pub attrs: Attributes,
    pub attrs_else: Option<Attributes>,
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

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = Attributes::parse_for_field(&input.attrs)?;
        let attrs_else = Attributes::parse_for_field_else(&input.attrs)?;
        let ty = &input.ty;
        if attrs.skip.is_none() {
            types.push(ty);
        };
        Ok(Self {
            attrs,
            attrs_else,
            name: Ident::new("_", input.ident.span()),
        })
    }
}

impl ToTokens for SyntaxFieldUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip.is_some() {
            return;
        }

        let span = self.name.span();
        let name = &self.name;

        let default_content = Content::Tokens(quote_spanned! { span => #name.syntax_fmt(f)?; });

        let content = Attributes::to_conditional_tokens(&self.attrs, &self.attrs_else, name, &default_content);

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
            if field.attrs.skip.is_none() {
                decls.push(field.decl());
            }
        }
        decls
    }
}

impl<'a> ParseType<'a> for SyntaxFieldsNamed {
    type Input = FieldsNamed;

    fn parse_type(
        types: &mut Vec<&'a Type>,
        input: &'a Self::Input,
    ) -> Result<Self, SyntaxError> {
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

    fn parse_type(
        types: &mut Vec<&'a Type>,
        input: &'a Self::Input,
    ) -> Result<Self, SyntaxError> {
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
                tokens.extend(quote_spanned! { span => { #inner, .. } });
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

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
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
