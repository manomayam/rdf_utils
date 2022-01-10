//! This library parses RDF content based on content type (or file name) and outputs sophia-compliant quad-source.

//! This is useful in situations where one have RDF in some serialization, and just need the parsed triples/quads, without having to concern oneself with picking the correct parser.

pub mod anyhow_quads;
pub mod anyhow_triples;
pub mod anyhow_rdf;
