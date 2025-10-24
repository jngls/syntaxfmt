use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident, Type, Variant};

use crate::{components::{content::WriteContent, parse_tokens::ParseTokens}, intermediate::{attributes::{SyntaxAttributes, SyntaxFieldAttributes}, fields::{SyntaxFields, SyntaxFieldsDecl}, parse_type::ParseType}, SyntaxError};

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
    pub fn decl(&self) -> SyntaxVariantDecl {
        SyntaxVariantDecl(&self.name, self.fields.decl())
    }
}

impl<'a> ParseType<'a> for SyntaxVariant {
    type Input = Variant;

    fn parse_type(types: &mut Vec<&'a Type>, variant: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = SyntaxFieldAttributes::parse_tokens(&variant.attrs)?;
        let fields = SyntaxFields::parse_type(types, &variant.fields)?;
        let name = variant.ident.clone();
        Ok(Self {
            attrs,
            fields,
            name,
        })
    }
}

impl ToTokens for SyntaxVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let name = &self.name;

        let fields = &self.fields;

        let decl = self.decl();

        let (pre, post) = self.attrs.split_pre_post();

        let content = WriteContent::tokens_or(&self.attrs.content, name, fields);

        let final_content = WriteContent::gen_content(&self.attrs, &pre, &content, &post);

        tokens.extend(quote! { #decl => { #final_content }});
    }
}

pub struct SyntaxVariants {
    pub variants: Vec<SyntaxVariant>,
}

impl<'a> ParseType<'a> for SyntaxVariants {
    type Input = Punctuated<Variant, Comma>;

    fn parse_type(types: &mut Vec<&'a Type>, punct_variants: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut variants = Vec::new();
        for variant in punct_variants {
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
