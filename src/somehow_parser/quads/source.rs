use std::io::BufRead;

use rio_turtle::{NQuadsParser, TriGParser, TurtleError};
use sophia_api::{
    quad::{stream::QuadSource, streaming_mode::StreamedQuad},
    triple::stream::{StreamError, StreamResult},
};
use sophia_rio::parser::{ScopedRioSourceQuad, StrictRioSource};

use crate::somehow_parser::errors::SomeHowSyntaxError;

/// This quad-source proxies quad generation from underlying quad-source of allowed types
enum InnerQuadSource<R: BufRead> {
    FromNQuadsParser(StrictRioSource<NQuadsParser<R>, TurtleError>),

    FromTriGParser(StrictRioSource<TriGParser<R>, TurtleError>),
}

/// A [`QuadSource`](QuadSource) implementation, that abstract over other quad-sources that are specialized for some concrete syntaxes. Currently abstracts over quad-sources from [`NQuadsParser`](sophia_turtle::parser::nq::NQuadsParser) and [`TriGParser`](sophia_turtle::parser::trig::TriGParser).
pub struct SomeHowQuadSource<R: BufRead>(InnerQuadSource<R>);

impl<R: BufRead> From<StrictRioSource<NQuadsParser<R>, TurtleError>> for SomeHowQuadSource<R> {
    fn from(s: StrictRioSource<NQuadsParser<R>, TurtleError>) -> Self {
        Self(InnerQuadSource::FromNQuadsParser(s))
    }
}

impl<R: BufRead> From<StrictRioSource<TriGParser<R>, TurtleError>> for SomeHowQuadSource<R> {
    fn from(s: StrictRioSource<TriGParser<R>, TurtleError>) -> Self {
        Self(InnerQuadSource::FromTriGParser(s))
    }
}

pub type SomeHowStreamError<SinkErr> = StreamError<SomeHowSyntaxError, SinkErr>;

pub type SomeHowStreamResult<T, SinkErr> = StreamResult<T, SomeHowSyntaxError, SinkErr>;

/// This function adapts StreamError by marshalling it's SourceError variant from known types to `SomeHowSyntaxError` type
fn adapt_stream_error<SourceErr, SinkErr>(
    e: StreamError<SourceErr, SinkErr>,
) -> SomeHowStreamError<SinkErr>
where
    SourceErr: Into<SomeHowSyntaxError> + std::error::Error,
    SinkErr: std::error::Error,
{
    match e {
        StreamError::SourceError(ev) => StreamError::SourceError(ev.into()),
        StreamError::SinkError(ev) => StreamError::SinkError(ev),
    }
}

fn adapt_stream_result<T, SourceErr, SinkErr>(
    r: StreamResult<T, SourceErr, SinkErr>,
) -> SomeHowStreamResult<T, SinkErr>
where
    SourceErr: Into<SomeHowSyntaxError> + std::error::Error,
    SinkErr: std::error::Error,
{
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(adapt_stream_error(e)),
    }
}

impl<R: BufRead> QuadSource for SomeHowQuadSource<R> {
    type Error = SomeHowSyntaxError;

    type Quad = ScopedRioSourceQuad;

    fn try_for_some_quad<F, E>(&mut self, f: &mut F) -> StreamResult<bool, Self::Error, E>
    where
        F: FnMut(StreamedQuad<Self::Quad>) -> Result<(), E>,
        E: std::error::Error,
    {
        match &mut self.0 {
            InnerQuadSource::FromNQuadsParser(s) => adapt_stream_result(s.try_for_some_quad(f)),
            InnerQuadSource::FromTriGParser(s) => adapt_stream_result(s.try_for_some_quad(f)),
        }
    }
}
