//! These parsers modules parses quads/triples from some known rdf concrete syntax. This is useful when one want to abstract over quad/triple source without concerning about underlying parsers.
//! 
//! These source abstractions are based on closed-enums instead of open-dynamic-trait-objects, as [`QuadSource`](sophia_api::quad::stream::QuadSource) and [`TripleSource`](sophia_api::triple::stream::TripleSource) are not allowed for trait objects

pub mod errors;
pub mod quads;
pub mod rdf;
pub mod triples;
