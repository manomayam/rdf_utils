use std::fmt::Display;

use mime::Mime;
use thiserror::Error;

use super::{
    correspondence::{EXTENSION_TO_SYNTAX_CORRESPONDENCE, MEDIA_TYPE_TO_SYNTAX_CORRESPONDENCE, Correspondent},
    file_extension::FileExtension,
    syntax::Syntax,
};

#[derive(Debug, Clone)]
/// An enum for hinting concrete-syntax of an rdf-document
pub enum SyntaxHint<'a> {
    Syntax(&'a Syntax),
    MediaType(&'a Mime),
    FileExtension(&'a FileExtension),
}

impl Display for SyntaxHint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxHint::Syntax(v) => write!(f, "Syntax: {}", v),
            SyntaxHint::MediaType(v) => write!(f, "MediaType: {}", v),
            SyntaxHint::FileExtension(v) => write!(f, "FileExtension: {}", v),
        }
    }
}

/// An error of unknown syntax hint
#[derive(Debug, Error, Clone)]
#[error("Unknown SyntaxHint {0}")]
pub struct UnknownSyntaxHintError<'a>(SyntaxHint<'a>);

impl<'a, T: AsRef<SyntaxHint<'a>>> From<&'_ T> for UnknownSyntaxHintError<'a> {
    fn from(hint: &T) -> Self {
        Self(hint.as_ref().clone())
    }
}

impl<'a> TryInto<Correspondent<Syntax>> for SyntaxHint<'a> {
    type Error = UnknownSyntaxHintError<'a>;

    /// For given SyntaxHint, resolves corresponding syntax.
    fn try_into(self) -> Result<Correspondent<Syntax>, Self::Error> {
        if let SyntaxHint::Syntax(s) = self {
            return Ok(Correspondent{
                value: s.clone(), is_total: true,
            });
        }

        let correspondent_opt = match self {
            SyntaxHint::Syntax(_) => unreachable!(),
            SyntaxHint::MediaType(mt) => MEDIA_TYPE_TO_SYNTAX_CORRESPONDENCE.get(mt),
            SyntaxHint::FileExtension(ext) => EXTENSION_TO_SYNTAX_CORRESPONDENCE.get(ext),
        };

        if let Some(correspondent) = correspondent_opt {
            Ok(correspondent.clone())
        } else {
            Err(UnknownSyntaxHintError(self.clone()))
        }
    }
}

impl<'a> TryInto<Syntax> for SyntaxHint<'a> {
    type Error = UnknownSyntaxHintError<'a>;

    fn try_into(self) -> Result<Syntax, Self::Error> {
        Ok(TryInto::<Correspondent<Syntax>>::try_into(self)?.value.clone())
    }
}
