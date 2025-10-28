use std::mem::take;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Data, DeriveInput, GenericParam, Generics, Ident, LifetimeParam,
    Result as SynResult, Type, WhereClause, parse_quote_spanned, punctuated::Punctuated,
    spanned::Spanned, token::Where,
};

use crate::{
    attributes::{
        args::TypeArgs,
        content::{Content, Skipped, ToConditionalTokens},
    },
    intermediate::{fields::SyntaxFields, parse_type::ParseType, variants::SyntaxVariants},
    syn_err,
};

#[derive(Debug, Clone)]
pub enum SyntaxTypeKind {
    Struct(SyntaxFields),
    Enum(SyntaxVariants),
}

impl<'a> ParseType<'a> for SyntaxTypeKind {
    type Input = Data;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        match input {
            Data::Struct(data_struct) => Ok(Self::Struct(SyntaxFields::parse_type(
                types,
                &data_struct.fields,
            )?)),
            Data::Enum(data_enum) => Ok(Self::Enum(SyntaxVariants::parse_type(
                types,
                &data_enum.variants,
            )?)),
            Data::Union(data_union) => syn_err(
                data_union.union_token,
                "syntaxfmt cannot be derived for unions",
            ),
        }
    }
}

impl ToTokens for SyntaxTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Struct(inner) => {
                let span = inner.span();
                let decl = inner.decl();
                tokens.extend(quote_spanned! { span => let Self #decl = self; });
                inner.to_tokens(tokens);
            }
            Self::Enum(inner) => {
                let span = inner.span();
                tokens.extend(quote_spanned! { span =>
                    #[allow(unreachable_patterns)]
                    match self {
                        #inner
                        _ => {}
                    }
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxType<'a> {
    pub args: TypeArgs,
    pub types: Vec<&'a Type>,
    pub kind: SyntaxTypeKind,
    pub generics: &'a Generics,
    pub name: &'a Ident,
}

impl<'a> SyntaxType<'a> {
    fn split_generics(&self) -> (TokenStream2, TokenStream2, TokenStream2, TokenStream2) {
        let state = self
            .args
            .args
            .state
            .as_ref()
            .map(|path| path.to_token_stream().clone())
            .unwrap_or(quote! { __SyntaxFmtState });

        let mut impl_gen = self.generics.params.clone();
        let ty_gen = self.generics.params.clone();
        let mut where_clause = self.generics.where_clause.clone().unwrap_or(WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::new(),
        });

        let span = impl_gen.span();
        let lifetimes = &self.args.lifetimes;
        impl_gen.extend(
            lifetimes
                .iter()
                .map(|lt| GenericParam::Lifetime(LifetimeParam::new(lt.clone()))),
        );
        if self.args.args.state.is_none() {
            impl_gen.push(parse_quote_spanned!(span => #state ));
        }

        if let Some(bound) = &self.args.args.state_bound {
            let span = bound.span();
            where_clause
                .predicates
                .push(syn::parse_quote_spanned! {span => #state: #bound });
        }

        for &field_ty in &self.types {
            let span = field_ty.span();
            where_clause.predicates.push(
                syn::parse_quote_spanned! { span => #field_ty: ::syntaxfmt::SyntaxFmt<#state> },
            );
        }

        let where_clause = (!where_clause.predicates.is_empty()).then_some(where_clause);
        (
            state,
            impl_gen.to_token_stream(),
            ty_gen.to_token_stream(),
            where_clause.to_token_stream(),
        )
    }
}

impl<'a> ParseType<'a> for SyntaxType<'a> {
    type Input = DeriveInput;

    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> SynResult<Self> {
        let args = TypeArgs::from_attributes(&input.attrs)?;
        let kind = SyntaxTypeKind::parse_type(types, &input.data)?;

        Ok(Self {
            args,
            types: take(types),
            kind,
            generics: &input.generics,
            name: &input.ident,
        })
    }
}

impl<'a> ToTokens for SyntaxType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name;
        let span = self.name.span();

        let (state, impl_gen, ty_gen, where_clause) = self.split_generics();

        let default_content = Content::Tokens(self.kind.to_token_stream());

        let content = if !self.args.skipped() {
            self.args
                .to_conditional_tokens(&quote! { self }, &default_content)
        } else {
            TokenStream2::new()
        };

        tokens.extend(quote_spanned! { span =>
            impl <#impl_gen> ::syntaxfmt::SyntaxFmt<#state> for #name<#ty_gen> #where_clause {
                fn syntax_fmt(&self, f: &mut ::syntaxfmt::SyntaxFormatter<#state>) -> ::std::fmt::Result {
                    #content
                    Ok(())
                }
            }
        });
    }
}
