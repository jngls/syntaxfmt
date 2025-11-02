use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident, Result as SynResult, Variant};

use crate::{
    attributes::{
        args::{CommonArgs, FieldArgs},
        content::{Content, Skipped, ToConditionalTokens},
        context::FieldKind,
    },
    intermediate::fields::{SyntaxFields, SyntaxFieldsDecl},
};

#[derive(Debug, Clone)]
pub struct SyntaxVariantDecl(Ident, SyntaxFieldsDecl);

impl ToTokens for SyntaxVariantDecl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.0;
        let fields_decl = &self.1;

        tokens.extend(quote! { Self::#name #fields_decl });
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxVariant {
    pub args: FieldArgs,
    pub fields: SyntaxFields,
    pub name: Ident,
}

impl SyntaxVariant {
    pub fn from_variant(parent_common: &CommonArgs, input: &Variant) -> SynResult<Self> {
        let name = input.ident.clone();
        let args = FieldArgs::new(FieldKind::SelfValue, parent_common, &input.attrs)?;
        let fields = SyntaxFields::from_fields(&args.args.common, &input.fields)?;
        Ok(Self { args, fields, name })
    }

    pub fn decl(&self) -> SyntaxVariantDecl {
        SyntaxVariantDecl(self.name.clone(), self.fields.decl())
    }
}

impl ToTokens for SyntaxVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.args.skipped() {
            return;
        }

        let decl = self.decl();

        let default_content = Content::Tokens(self.fields.to_token_stream());

        let content = self.args.to_conditional_tokens(&default_content);

        tokens.extend(quote! { #decl => { #content }});
    }
}

#[derive(Debug, Default, Clone)]
pub struct SyntaxVariants {
    pub variants: Vec<SyntaxVariant>,
}

impl SyntaxVariants {
    pub fn from_variants(
        parent_common: &CommonArgs,
        input: &Punctuated<Variant, Comma>,
    ) -> SynResult<Self> {
        let mut variants = Vec::new();
        for variant in input {
            variants.push(SyntaxVariant::from_variant(parent_common, variant)?);
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
