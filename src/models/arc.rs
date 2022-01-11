//! This module exports type aliases for sophia models that works with terms whose TermData is of type `Arc<str>`
use std::sync::Arc;

use sophia_inmem::dataset::{GenericDataset, GspoWrapper, OgpsWrapper};
use sophia_inmem::graph::{GenericGraph, OpsWrapper, SpoWrapper};
use sophia_term::blank_node::BlankNode;
use sophia_term::factory::ArcTermFactory;
use sophia_term::iri::Iri;
use sophia_term::literal::Literal;
use sophia_term::Term;

pub type ArcIri = Iri<Arc<str>>;
pub type ArcLiteral = Literal<Arc<str>>;
pub type ArcBlankNode = BlankNode<Arc<str>>;
pub type ArcTerm = Term<Arc<str>>;

pub type ArcGraph = GenericGraph<u16, ArcTermFactory>;
pub type BigArcGraph = GenericGraph<u32, ArcTermFactory>;
pub type IndexedArcGraph = OpsWrapper<SpoWrapper<GenericGraph<u16, ArcTermFactory>>>;
pub type IndexedBigArcGraph = OpsWrapper<SpoWrapper<GenericGraph<u32, ArcTermFactory>>>;

/// A heavily indexed dataset. Fast to query but slow to load, with a relatively high memory footprint.supports up to 2^16 terms
pub type IndexedArcDataset = OgpsWrapper<GspoWrapper<GenericDataset<u32, ArcTermFactory>>>;
/// A heavily indexed dataset. Fast to query but slow to load, with a relatively high memory footprint. supports up to 2^32 terms
pub type IndexedBigArcDataset = OgpsWrapper<GspoWrapper<GenericDataset<u32, ArcTermFactory>>>;

pub type ArcDataset = GenericDataset<u16, ArcTermFactory>;
pub type BigArcDataset = GenericDataset<u32, ArcTermFactory>;
