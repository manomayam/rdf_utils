use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;
use sophia_api::triple::stream::{StreamError, StreamResult};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
enum InnerSyntaxError {
    TurtleError(#[from] TurtleError),
    RdfXmlError(#[from] RdfXmlError),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
/// An error that abstracts over other syntax parsing errors. Currently it can be constructed from [`TurtleError`](TurtleError), and [`RdfXmlError`](RdfXmlError)
pub struct SomeSyntaxError(InnerSyntaxError);

impl From<TurtleError> for SomeSyntaxError {
    fn from(e: TurtleError) -> Self {
        Self(e.into())
    }
}

impl From<RdfXmlError> for SomeSyntaxError {
    fn from(e: RdfXmlError) -> Self {
        Self(e.into())
    }
}

pub type SomeHowStreamError<SinkErr> = StreamError<SomeSyntaxError, SinkErr>;

/// This function adapts StreamError by marshalling it's SourceError variant from known types to `SomeHowSyntaxError` type
pub fn adapt_stream_error<SourceErr, SinkErr>(
    e: StreamError<SourceErr, SinkErr>,
) -> SomeHowStreamError<SinkErr>
where
    SourceErr: Into<SomeSyntaxError> + std::error::Error,
    SinkErr: std::error::Error,
{
    match e {
        StreamError::SourceError(ev) => StreamError::SourceError(ev.into()),
        StreamError::SinkError(ev) => StreamError::SinkError(ev),
    }
}

pub type SomeHowStreamResult<T, SinkErr> = StreamResult<T, SomeSyntaxError, SinkErr>;

pub fn adapt_stream_result<T, SourceErr, SinkErr>(
    r: StreamResult<T, SourceErr, SinkErr>,
) -> SomeHowStreamResult<T, SinkErr>
where
    SourceErr: Into<SomeSyntaxError> + std::error::Error,
    SinkErr: std::error::Error,
{
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(adapt_stream_error(e)),
    }
}
