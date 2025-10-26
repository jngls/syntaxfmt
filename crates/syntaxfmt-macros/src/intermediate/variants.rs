use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident, Type, Variant};

use crate::{components::{attributes::Attributes, content::Content}, intermediate::{fields::{SyntaxFields, SyntaxFieldsDecl}, parse_type::ParseType}, SyntaxError};

#[derive(Debug, Clone)]
pub struct SyntaxVariantDecl(Ident, SyntaxFieldsDecl);

impl ToTokens for SyntaxVariantDecl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let span = self.0.span();

        let name = &self.0;
        let fields_decl = &self.1;

        tokens.extend(quote_spanned! { span => Self::#name #fields_decl });
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxVariant {
    pub attrs: Attributes,
    pub attrs_else: Option<Attributes>,
    pub fields: SyntaxFields,
    pub name: Ident,
}

impl SyntaxVariant {
    pub fn decl(&self) -> SyntaxVariantDecl {
        SyntaxVariantDecl(self.name.clone(), self.fields.decl())
    }
}

impl<'a> ParseType<'a> for SyntaxVariant {
    type Input = Variant;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = Attributes::parse_for_field(&input.attrs)?;
        let attrs_else = Attributes::parse_for_field_else(&input.attrs)?;
        let fields = SyntaxFields::parse_type(types, &input.fields)?;
        let name = input.ident.clone();
        Ok(Self {
            attrs,
            attrs_else,
            fields,
            name,
        })
    }
}

impl ToTokens for SyntaxVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip.is_some() {
            return;
        }

        let span = self.name.span();
        let decl = self.decl();

        let default_content = Content::Tokens(self.fields.to_token_stream());

        let content = Attributes::to_conditional_tokens(&self.attrs, &self.attrs_else, &quote! { self }, &default_content);

        tokens.extend(quote_spanned! { span => #decl => { #content }});
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxVariants {
    pub variants: Vec<SyntaxVariant>,
}

impl<'a> ParseType<'a> for SyntaxVariants {
    type Input = Punctuated<Variant, Comma>;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut variants = Vec::new();
        for variant in input {
            variants.push(SyntaxVariant::parse_type(types, variant)?);
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
