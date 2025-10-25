use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident, Type, Variant};

use crate::{components::{attributes::Attributes, content::Content}, intermediate::{fields::{SyntaxFields, SyntaxFieldsDecl}, parse_type::ParseType}, SyntaxError};

#[cfg(feature = "trace")]
use crate::{trace, DEPTH};

#[derive(Debug, Clone)]
pub struct SyntaxVariantDecl<'a>(&'a Ident, SyntaxFieldsDecl<'a>);

impl<'a> ToTokens for SyntaxVariantDecl<'a> {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let span = self.0.span();

        let name = self.0;
        let fields_decl = &self.1;

        tokens.extend(quote_spanned! { span => Self::#name #fields_decl });
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxVariant {
    pub attrs: Attributes,
    pub fields: SyntaxFields,
    pub name: Ident,
}

impl SyntaxVariant {
    #[cfg_attr(feature = "trace", trace)]
    pub fn decl(&self) -> SyntaxVariantDecl {
        SyntaxVariantDecl(&self.name, self.fields.decl())
    }
}

impl<'a> ParseType<'a> for SyntaxVariant {
    type Input = Variant;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = Attributes::parse_for_field(&input.attrs)?;
        let fields = SyntaxFields::parse_type(types, &input.fields)?;
        let name = input.ident.clone();
        Ok(Self {
            attrs,
            fields,
            name,
        })
    }
}

impl ToTokens for SyntaxVariant {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let span = self.name.span();
        let decl = self.decl();

        let insert = quote_spanned! { span => let field = self; };
        let default_content = Content::Tokens(self.fields.to_token_stream());
        let content = self.attrs.to_tokens(insert, default_content);

        tokens.extend(quote_spanned! { span => #decl => { #content }});
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxVariants {
    pub variants: Vec<SyntaxVariant>,
}

impl<'a> ParseType<'a> for SyntaxVariants {
    type Input = Punctuated<Variant, Comma>;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut variants = Vec::new();
        for variant in input {
            variants.push(SyntaxVariant::parse_type(types, variant)?);
        }
        Ok(Self { variants })
    }
}

impl ToTokens for SyntaxVariants {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for variant in &self.variants {
            variant.to_tokens(tokens);
        }
    }
}
