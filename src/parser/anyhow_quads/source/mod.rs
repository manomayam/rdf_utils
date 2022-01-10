use std::{error::Error, io::BufRead};

use rio_turtle::{NQuadsParser, TriGParser, TurtleError};
use sophia_api::quad::stream::QuadSource;
use sophia_rio::parser::{ScopedRioSourceQuad, StrictRioSource, ScopedGRioSourceQuad};

use self::{wrapper::AnyHowAdaptableQuadSource, errors::AnySyntaxError};

pub mod errors;
mod wrapper;


pub struct AnyHowQuadSource<R: BufRead> {
    inner: Box<dyn QuadSource<Error = TurtleError, Quad = ScopedGRioSourceQuad>>,
}

impl<R: BufRead> QuadSource for AnyHowQuadSource<R> {
    type Error = AnySyntaxError;

    type Quad = ScopedRioSourceQuad;

    fn try_for_some_quad<F, E>(
        &mut self,
        f: &mut F,
    ) -> sophia_api::triple::stream::StreamResult<bool, Self::Error, E>
    where
        F: FnMut(sophia_api::quad::streaming_mode::StreamedQuad<Self::Quad>) -> Result<(), E>,
        E: Error,
    {
        todo!()
    }
}
