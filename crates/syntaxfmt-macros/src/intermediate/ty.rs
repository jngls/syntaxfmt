use std::mem::take;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    Data, DeriveInput, Generics, Ident, Type, WhereClause, parse_quote_spanned,
    punctuated::Punctuated, spanned::Spanned, token::Where,
};

use crate::{
    components::{attributes::Attributes, content::Content}, intermediate::{
        fields::SyntaxFields,
        parse_type::ParseType,
        variants::SyntaxVariants,
    }, SyntaxError
};

#[cfg(feature = "trace")]
use crate::{DEPTH, trace};

#[derive(Debug, Clone)]
pub enum SyntaxTypeKind {
    Struct(SyntaxFields),
    Enum(SyntaxVariants),
}

impl<'a> ParseType<'a> for SyntaxTypeKind {
    type Input = Data;

    #[cfg_attr(feature = "trace", trace)]
    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        match input {
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
    #[cfg_attr(feature = "trace", trace)]
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
    pub attrs: Attributes,
    pub types: Vec<&'a Type>,
    pub kind: SyntaxTypeKind,
    pub generics: &'a Generics,
    pub name: &'a Ident,
}

impl<'a> SyntaxType<'a> {
    #[cfg_attr(feature = "trace", trace)]
    fn split_generics(&self) -> (TokenStream2, TokenStream2, TokenStream2) {
        let mut impl_gen = self.generics.params.clone();
        let ty_gen = self.generics.params.clone();
        let mut where_clause = self.generics.where_clause.clone().unwrap_or(WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::new(),
        });

        let span = impl_gen.span();
        impl_gen.push(parse_quote_spanned!(span => __SyntaxFmtState));

        if let Some((_, bound)) = &self.attrs.state_bound {
            let span = bound.span();
            where_clause
                .predicates
                .push(syn::parse_quote_spanned! {span => __SyntaxFmtState: #bound });
        }

        for &field_ty in &self.types {
            let span = field_ty.span();
            where_clause
                .predicates
                .push(syn::parse_quote_spanned! { span => #field_ty: ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> });
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

    #[cfg_attr(feature = "trace", trace)]
    fn parse_type(types: &mut Vec<&'a Type>, input: &'a Self::Input) -> Result<Self, SyntaxError> {
        let attrs = Attributes::parse_for_type(&input.attrs)?;
        let kind = SyntaxTypeKind::parse_type(types, &input.data)?;

        Ok(Self {
            attrs,
            types: take(types),
            kind,
            generics: &input.generics,
            name: &input.ident,
        })
    }
}

impl<'a> ToTokens for SyntaxType<'a> {
    #[cfg_attr(feature = "trace", trace)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.attrs.skip {
            return;
        }

        let name = self.name;
        let span = self.name.span();

        let (impl_gen, ty_gen, where_clause) = self.split_generics();

        let insert = quote! { let field = self; };
        let default_content = Content::Tokens(self.kind.to_token_stream());
        let content = self.attrs.to_tokens(insert, default_content);

        tokens.extend(quote_spanned! { span =>
            impl <#impl_gen> ::syntaxfmt::SyntaxFmt<__SyntaxFmtState> for #name<#ty_gen> #where_clause {
                fn syntax_fmt(&self, f: &mut ::syntaxfmt::SyntaxFormatter<__SyntaxFmtState>) -> ::std::fmt::Result {
                    #content
                    Ok(())
                }
            }
        });
    }
}
