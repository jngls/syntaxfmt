use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident, Type, Variant,     Result as SynResult,
    Error as SynError,
};

use crate::{attributes::{args::FieldArgs, content::{Content, Skipped, ToConditionalTokens}}, intermediate::{fields::{SyntaxFields, SyntaxFieldsDecl}, parse_type::ParseType}};

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
    pub args: FieldArgs,
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

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        let args = FieldArgs::from_attributes(&input.attrs)?;
        let fields = SyntaxFields::parse_type(types, &input.fields)?;
        let name = input.ident.clone();
        Ok(Self {
            args,
            fields,
            name,
        })
    }
}

impl ToTokens for SyntaxVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.args.skipped() {
            return;
        }

        let span = self.name.span();
        let decl = self.decl();

        let default_content = Content::Tokens(self.fields.to_token_stream());

        let content = self.args.to_conditional_tokens(&quote! { self }, &default_content);

        tokens.extend(quote_spanned! { span => #decl => { #content }});
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxVariants {
    pub variants: Vec<SyntaxVariant>,
}

impl<'a> ParseType<'a> for SyntaxVariants {
    type Input = Punctuated<Variant, Comma>;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
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
