use std::io::BufRead;

use sophia_api::{quad::{stream::QuadSource, streaming_mode::StreamedQuad}, triple::stream::StreamResult};
use sophia_rio::parser::ScopedRioSourceQuad;

use super::errors::AnySyntaxError;

pub trait AnyHowAdaptableQuadSource<R: BufRead> {
    fn try_for_some_quad_any_how<F, E>(
        &mut self,
        f: &mut F,
    ) -> StreamResult<bool, AnySyntaxError, E>
    where
        F: FnMut(StreamedQuad<ScopedRioSourceQuad>) -> Result<(), E>,
        E: std::error::Error,
    {
        todo!()
    }
}
