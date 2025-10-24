use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_quote, punctuated::Punctuated, token::Where, Data, DeriveInput, Generics, Ident, Type, WhereClause};

use crate::{components::{content::WriteContent, parse_tokens::ParseTokens}, intermediate::{attributes::{SyntaxAttributes, SyntaxTypeAttributes}, fields::SyntaxFields, parse_type::ParseType, variants::SyntaxVariants}, SyntaxError};

pub enum SyntaxTypeKind {
    Struct(SyntaxFields),
    Enum(SyntaxVariants),
}

impl<'a> ParseType<'a> for SyntaxTypeKind {
    type Input = Data;

    fn parse_type(types: &mut Vec<&'a Type>, data: &'a Self::Input) -> Result<Self, SyntaxError> {
        match data {
            Data::Struct(data_struct) => Ok(Self::Struct(SyntaxFields::parse_type(
                types,
                &data_struct.fields,
            )?)),
            Data::Enum(data_enum) => Ok(Self::Enum(SyntaxVariants::parse_type(
                types,
                &data_enum.variants,
            )?)),
            Data::Union(data_union) => Err(SyntaxError::Union(data_union.union_token)),
        }
    }
}

impl ToTokens for SyntaxTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Struct(inner) => {
                let decl = inner.decl();
                tokens.extend(quote! { let Self #decl = &self; });
                inner.to_tokens(tokens);
            }
            Self::Enum(inner) => {
                tokens.extend(quote! {
                    match self {
                        #inner
                        _ => {}
                    }
                });
            }
        }
    }
}

pub struct SyntaxType<'a> {
    pub attrs: SyntaxTypeAttributes,
    pub types: Vec<&'a Type>,
    pub kind: SyntaxTypeKind,
    pub generics: &'a Generics,
    pub name: &'a Ident,
}

impl<'a> SyntaxType<'a> {
    fn split_generics(&self) -> (TokenStream2, TokenStream2, TokenStream2) {
        let mut impl_gen = self.generics.params.clone();
        let ty_gen = self.generics.params.clone();
        let mut where_clause = self.generics.where_clause.clone().unwrap_or(WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::new(),
        });

        impl_gen.push(parse_quote!(__SyntaxFmtState));

        if let Some(bound) = &self.attrs.state_bound {
            where_clause
                .predicates
                .push(syn::parse_quote! { __SyntaxFmtState: #bound });
        }

        for &field_ty in &self.types {
            where_clause
                .predicates
                .push(syn::parse_quote! { #field_ty: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> });
        }

        let where_clause = (!where_clause.predicates.is_empty()).then_some(where_clause);
        (
            impl_gen.to_token_stream(),
            ty_gen.to_token_stream(),
            where_clause.to_token_stream(),
        )
    }
}

impl<'a> ParseType<'a> for SyntaxType<'a> {
    type Input = DeriveInput;

    fn parse_type(_: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let mut types = Vec::new();

        let attrs = SyntaxTypeAttributes::parse_tokens(&input.attrs)?;
        let kind = SyntaxTypeKind::parse_type(&mut types, &input.data)?;

        Ok(Self {
            attrs,
            types,
            kind,
            generics: &input.generics,
            name: &input.ident,
        })
    }
}

impl<'a> ToTokens for SyntaxType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let name = self.name;

        let (impl_gen, ty_gen, where_clause) = self.split_generics();

        let (pre, post) = self.attrs.split_pre_post();

        let content = WriteContent::tokens_or(&self.attrs.content, name, &self.kind);

        let final_content = WriteContent::gen_content(&self.attrs, &pre, &content, &post);

        tokens.extend(quote! {
            impl <#impl_gen> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for #name<#ty_gen> #where_clause {
                fn syntax_fmt(&self, f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>) -> ::std::fmt::Result {
                    #final_content
                    Ok(())
                }
            }
        });

    }
}
