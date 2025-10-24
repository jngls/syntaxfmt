use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Type, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

use crate::{
    SyntaxError,
    components::{
        content::WriteContent,
        parse_tokens::ParseTokens,
    },
    intermediate::{
        attributes::{SyntaxAttributes, SyntaxFieldAttributes},
        parse_type::ParseType,
    },
};

pub struct SyntaxFieldNamed {
    pub attrs: SyntaxFieldAttributes,
    pub name: Ident,
}

impl SyntaxFieldNamed {
    pub fn decl(&self) -> &Ident {
        &self.name
    }
}

impl<'a> ParseType<'a> for SyntaxFieldNamed {
    type Input = Field;

    fn parse_type(types: &mut Vec<&'a Type>, field: &'a Self::Input) -> Result<Self, SyntaxError> {
        let ty = &field.ty;
        let attrs = SyntaxFieldAttributes::parse_tokens(&field.attrs)?;
        let name = if attrs.skip {
            Ident::new("_", field.ident.span())
        } else {
            types.push(ty);
            field.ident.clone().unwrap()
        };
        Ok(Self { attrs, name })
    }
}

impl ToTokens for SyntaxFieldNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let name = &self.name;

        let (pre, post) = self.attrs.split_pre_post();

        let content =
            WriteContent::tokens_or(&self.attrs.content, name, quote! { #name.syntax_fmt(f)?; });

        tokens.extend(WriteContent::gen_content(&self.attrs, &pre, &content, &post));
    }
}

pub struct SyntaxFieldUnnamed {
    pub attrs: SyntaxFieldAttributes,
    pub name: Ident,
    //    pub ty: &'a Type,
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

    fn parse_type(types: &mut Vec<&'a Type>, field: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = SyntaxFieldAttributes::parse_tokens(&field.attrs)?;
        let ty = &field.ty;
        if !attrs.skip {
            types.push(ty);
        };
        Ok(Self {
            attrs,
            name: Ident::new("_", field.ident.span()),
        })
    }
}

impl ToTokens for SyntaxFieldUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let name = &self.name;

        let (pre, post) = self.attrs.split_pre_post();

        let content =
            WriteContent::tokens_or(&self.attrs.content, name, quote! { #name.syntax_fmt(f)?; });

        tokens.extend(WriteContent::gen_content(&self.attrs, &pre, &content, &post));
    }
}

pub struct SyntaxFieldsNamed {
    pub fields: Vec<SyntaxFieldNamed>,
}

impl SyntaxFieldsNamed {
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

    fn parse_type(
        types: &mut Vec<&'a Type>,
        fields_named: &'a Self::Input,
    ) -> Result<Self, SyntaxError> {
        let mut fields = Vec::new();
        for field in &fields_named.named {
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

pub struct SyntaxFieldsUnnamed {
    pub fields: Vec<SyntaxFieldUnnamed>,
}

impl SyntaxFieldsUnnamed {
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

    fn parse_type(
        types: &mut Vec<&'a Type>,
        fields_unnamed: &'a Self::Input,
    ) -> Result<Self, SyntaxError> {
        let mut fields = Vec::new();
        for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
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

    fn parse_type(types: &mut Vec<&'a Type>, fields: &'a Self::Input) -> Result<Self, SyntaxError> {
        match &fields {
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
