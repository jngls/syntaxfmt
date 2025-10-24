use syn::{
    Attribute, Expr, ExprPath, Meta, MetaNameValue, Path, punctuated::Punctuated, token::Comma,
};

use crate::{
    SyntaxError,
    components::{
        ClearNone, DualString, Format, PopDelims, PopIndentRegion, PushDelims, PushIndentRegion,
        SetNone, SplitFormat, SyntaxFieldPostContent, SyntaxFieldPreContent, SyntaxTypePostContent,
        SyntaxTypePreContent, WriteContent, WriteIndent,
    },
};

pub trait SyntaxAttributes<'a>: Sized + Default {
    type PreContent;
    type PostContent;

    fn format(&self) -> Option<&Format>;
    fn delim(&self) -> Option<&PushDelims>;
    fn content(&self) -> Option<&WriteContent>;
    fn indent_region(&self) -> Option<&PushIndentRegion>;
    fn indent(&self) -> Option<&WriteIndent>;
    fn skip(&self) -> bool;

    fn set_format(&mut self, value: Format);
    fn set_delim(&mut self, value: PushDelims);
    fn set_content(&mut self, value: WriteContent);
    fn set_indent_region(&mut self, value: PushIndentRegion);
    fn set_indent(&mut self, value: WriteIndent);
    fn set_skip(&mut self, value: bool);

    fn parse(attrs: &[Attribute]) -> Result<Self, SyntaxError> {
        let mut _self = Self::default();
        for attr in attrs {
            if attr.path().is_ident("syntax") {
                _self.parse_meta(&attr.meta)?;
            }
        }
        Ok(_self)
    }

    fn parse_meta(&mut self, meta: &Meta) -> Result<(), SyntaxError> {
        if let Ok(meta_list) = meta.require_list() {
            if let Ok(nested_list) =
                meta_list.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
            {
                for meta in nested_list {
                    match meta {
                        Meta::Path(path) => self.parse_meta_path(path)?,
                        Meta::NameValue(MetaNameValue { path, value, .. }) => {
                            self.parse_meta_name_value(path, value)?
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_meta_path(&mut self, path: Path) -> Result<(), SyntaxError> {
        if path.is_ident("indent_region") {
            self.set_indent_region(PushIndentRegion);
        } else if path.is_ident("indent") {
            self.set_indent(WriteIndent);
        } else if path.is_ident("skip") {
            self.set_skip(true);
        } else {
            self.parse_meta_path_special(path)?;
        }
        Ok(())
    }

    fn parse_meta_name_value(&mut self, path: Path, value: Expr) -> Result<(), SyntaxError> {
        if path.is_ident("format") {
            self.set_format(DualString::parse(&value).map(DualString::into_format)?);
        } else if path.is_ident("delim") {
            self.set_delim(PushDelims::parse(&value)?);
        } else if path.is_ident("content") {
            self.set_content(WriteContent::parse(&value)?);
        } else {
            self.parse_meta_name_value_special(path, value)?;
        }
        Ok(())
    }

    fn parse_meta_path_special(&mut self, path: Path) -> Result<(), SyntaxError> {
        return Err(SyntaxError::UnexpectedAttributeArg(path));
    }

    fn parse_meta_name_value_special(
        &mut self,
        path: Path,
        _value: Expr,
    ) -> Result<(), SyntaxError> {
        return Err(SyntaxError::UnexpectedAttributeArg(path));
    }

    fn split_pre_post(&'a self) -> (Self::PreContent, Self::PostContent);
}

#[rustfmt::skip]
macro_rules! impl_common_attribute_args {
    () => {
        fn format(&self) -> Option<&Format> { self.format.as_ref() }
        fn delim(&self) -> Option<&PushDelims> { self.delim.as_ref() }
        fn content(&self) -> Option<&WriteContent> { self.content.as_ref() }
        fn indent_region(&self) -> Option<&PushIndentRegion> { self.indent_region.as_ref() }
        fn indent(&self) -> Option<&WriteIndent> { self.indent.as_ref() }
        fn skip(&self) -> bool { self.skip }

        fn set_format(&mut self, value: Format) { self.format = Some(value); }
        fn set_delim(&mut self, value: PushDelims) { self.delim = Some(value); }
        fn set_content(&mut self, value: WriteContent) { self.content = Some(value); }
        fn set_indent_region(&mut self, value: PushIndentRegion) { self.indent_region = Some(value); }
        fn set_indent(&mut self, value: WriteIndent) { self.indent = Some(value); }
        fn set_skip(&mut self, value: bool) { self.skip = value; }
    };
}

#[derive(Default)]
pub struct SyntaxTypeAttributes {
    pub format: Option<Format>,
    pub delim: Option<PushDelims>,
    pub state_bound: Option<ExprPath>,
    pub content: Option<WriteContent>,
    pub indent_region: Option<PushIndentRegion>,
    pub indent: Option<WriteIndent>,
    pub skip: bool,
}

impl<'a> SyntaxAttributes<'a> for SyntaxTypeAttributes {
    type PreContent = SyntaxTypePreContent<'a>;
    type PostContent = SyntaxTypePostContent<'a>;

    impl_common_attribute_args!();

    fn parse_meta_name_value_special(&mut self, path: Path, expr: Expr) -> Result<(), SyntaxError> {
        if path.is_ident("state_bound") {
            match expr {
                Expr::Path(expr_path) => self.state_bound = Some(expr_path),
                expr => return Err(SyntaxError::ExpectedTraitPath(expr)),
            }
        } else {
            return Err(SyntaxError::UnexpectedAttributeArg(path));
        }
        Ok(())
    }

    fn split_pre_post(&'a self) -> (Self::PreContent, Self::PostContent) {
        let (write_prefix, write_suffix, _) = self.format().split_format();
        let write_indent = self.indent();
        let push_delim = self.delim();
        let push_indent = self.indent_region();
        let pop_delim = push_delim.map(|_| PopDelims);
        let pop_indent = push_indent.map(|_| PopIndentRegion);
        (
            Self::PreContent {
                write_prefix,
                write_indent,
                push_delim,
                push_indent,
            },
            Self::PostContent {
                write_suffix,
                pop_delim,
                pop_indent,
            },
        )
    }
}

#[derive(Default)]
pub struct SyntaxFieldAttributes {
    pub format: Option<Format>,
    pub delim: Option<PushDelims>,
    pub none: Option<SetNone>,
    pub content: Option<WriteContent>,
    pub indent_region: Option<PushIndentRegion>,
    pub indent: Option<WriteIndent>,
    pub skip: bool,
}

impl<'a> SyntaxAttributes<'a> for SyntaxFieldAttributes {
    type PreContent = SyntaxFieldPreContent<'a>;
    type PostContent = SyntaxFieldPostContent<'a>;

    impl_common_attribute_args!();

    fn parse_meta_name_value_special(&mut self, path: Path, expr: Expr) -> Result<(), SyntaxError> {
        if path.is_ident("none") {
            self.none = Some(SetNone::parse(&expr)?);
        } else {
            return Err(SyntaxError::UnexpectedAttributeArg(path));
        }
        Ok(())
    }

    fn split_pre_post(&'a self) -> (Self::PreContent, Self::PostContent) {
        let (write_prefix, write_suffix, _) = self.format().split_format();
        let write_indent = self.indent();
        let push_delim = self.delim();
        let push_indent = self.indent_region();
        let set_none = self.none.as_ref();
        let pop_delim = push_delim.map(|_| PopDelims);
        let pop_indent = push_indent.map(|_| PopIndentRegion);
        let clear_none = self.none.as_ref().map(|_| ClearNone);
        (
            Self::PreContent {
                write_prefix,
                write_indent,
                push_delim,
                push_indent,
                set_none,
            },
            Self::PostContent {
                write_suffix,
                pop_delim,
                pop_indent,
                clear_none,
            },
        )
    }
}
