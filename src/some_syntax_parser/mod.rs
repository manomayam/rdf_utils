pub mod errors;

/// This module provides [`SomeSyntaxQuadParser`](quads::SomeSyntaxQuadParser) which can be instantiated for runtime chosen concrete-syntax.
pub mod quads;
/// This module provides abstraction, that can be used either as QuadParser or TripleParser, for runtime chosen concrete syntax
pub mod rdf;
/// This module provides [`SomeSyntaxTripleParser`](triples::SomeSyntaxTripleParser) which can be instantiated for runtime chosen concrete-syntax.
pub mod triples;
