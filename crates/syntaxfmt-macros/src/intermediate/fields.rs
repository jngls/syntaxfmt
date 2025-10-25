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

#[cfg(feature = "trace")]
use crate::{trace, DEPTH};

#[derive(Debug, Clone)]
pub struct SyntaxFieldNamed {
    pub attrs: Attributes,
    pub name: Ident,
}

impl SyntaxFieldNamed {
    #[cfg_attr(feature = "trace", trace)]
    pub fn decl(&self) -> &Ident {
        &self.name
    }
}

impl<'a> ParseType<'a> for SyntaxFieldNamed {
    type Input = Field;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let ty = &input.ty;
        let attrs = Attributes::parse_for_field(&input.attrs)?;
        let name = if attrs.skip {
            Ident::new("_", input.ident.span())
        } else {
            types.push(ty);
            input.ident.clone().unwrap()
        };
        Ok(Self { attrs, name })
    }
}

impl ToTokens for SyntaxFieldNamed {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let span = self.name.span();
        let name = &self.name;

        let insert = quote_spanned! { span => let field = #name; };
        let default_content = Content::Tokens(quote_spanned! { span => #name.syntax_fmt(f)?; });
        let content = self.attrs.to_tokens(insert, default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxFieldUnnamed {
    pub attrs: Attributes,
    pub name: Ident,
    //    pub ty: &'a Type,
}

impl SyntaxFieldUnnamed {
    #[cfg_attr(feature = "trace", trace)]
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

    #[cfg_attr(feature = "trace", trace)]
    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = Attributes::parse_for_field(&input.attrs)?;
        let ty = &input.ty;
        if !attrs.skip {
            types.push(ty);
        };
        Ok(Self {
            attrs,
            name: Ident::new("_", input.ident.span()),
        })
    }
}

impl ToTokens for SyntaxFieldUnnamed {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let span = self.name.span();
        let name = &self.name;

        let insert = quote_spanned! { span => let field = #name; };
        let default_content = Content::Tokens(quote_spanned! { span => #name.syntax_fmt(f)?; });
        let content = self.attrs.to_tokens(insert, default_content);

        tokens.extend(content);
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxFieldsNamed {
    pub fields: Vec<SyntaxFieldNamed>,
}

impl SyntaxFieldsNamed {
    #[cfg_attr(feature = "trace", trace)]
    pub fn decl(&self) -> Punctuated<&Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            decls.push(field.decl());
        }
        decls
    }
}

impl<'a> ParseType<'a> for SyntaxFieldsNamed {
    type Input = FieldsNamed;

    #[cfg_attr(feature = "trace", trace)]
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
    #[cfg_attr(feature = "trace", trace)]
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
    #[cfg_attr(feature = "trace", trace)]
    pub fn decl(&self) -> Punctuated<&Ident, Comma> {
        let mut decls = Punctuated::new();
        for field in &self.fields {
            decls.push(field.decl());
        }
        decls
    }
}

impl<'a> ParseType<'a> for SyntaxFieldsUnnamed {
    type Input = FieldsUnnamed;

    #[cfg_attr(feature = "trace", trace)]
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
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for field in &self.fields {
            field.to_tokens(tokens);
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxFieldsDecl<'a> {
    Named(Punctuated<&'a Ident, Comma>),
    Unnamed(Punctuated<&'a Ident, Comma>),
    Unit,
}

impl<'a> ToTokens for SyntaxFieldsDecl<'a> {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SyntaxFieldsDecl::Named(inner) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span => { #inner } });
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
    #[cfg_attr(feature = "trace", trace)]
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

    #[cfg_attr(feature = "trace", trace)]
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
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SyntaxFields::Named(inner) => inner.to_tokens(tokens),
            SyntaxFields::Unnamed(inner) => inner.to_tokens(tokens),
            SyntaxFields::Unit => {}
        }
    }
}
