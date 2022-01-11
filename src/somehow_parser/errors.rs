use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;


#[derive(Debug, thiserror::Error)]
#[error(transparent)]
enum InnerSyntaxError {
    TurtleError(#[from] TurtleError),
    RdfXmlError(#[from] RdfXmlError)
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
/// An error that abstracts over other syntax parsing errors. Currently it can be constructed from [`TurtleError`](TurtleError), and [`RdfXmlError`](RdfXmlError)
pub struct SomeHowSyntaxError(InnerSyntaxError);

impl From<TurtleError> for SomeHowSyntaxError {
    fn from(e: TurtleError) -> Self {
        Self(e.into())
    }
}

impl From<RdfXmlError> for SomeHowSyntaxError {
    fn from(e: RdfXmlError) -> Self {
        Self(e.into())
    }
}
