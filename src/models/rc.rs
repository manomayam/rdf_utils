//! This module exports type aliases for sophia models that works with terms whose TermData is of type `Rc<str>`
use std::rc::Rc;

use sophia_inmem::dataset::{GenericDataset, GspoWrapper, OgpsWrapper};
use sophia_inmem::graph::{GenericGraph, SpoWrapper, OpsWrapper};
use sophia_term::Term;
use sophia_term::blank_node::BlankNode;
use sophia_term::factory::RcTermFactory;
use sophia_term::iri::Iri;
use sophia_term::literal::Literal;


pub type RcIri = Iri<Rc<str>>;
pub type RcLiteral = Literal<Rc<str>>;
pub type RcBlankNode = BlankNode<Rc<str>>;
pub type RcTerm = Term<Rc<str>>;


pub type RcGraph = GenericGraph<u16, RcTermFactory>;
pub type BigRcGraph = GenericGraph<u32, RcTermFactory>;
pub type IndexedRcGraph = OpsWrapper<SpoWrapper<GenericGraph<u16, RcTermFactory>>>;
pub type IndexedBigRcGraph = OpsWrapper<SpoWrapper<GenericGraph<u32, RcTermFactory>>>;

/// A heavily indexed dataset. Fast to query but slow to load, with a relatively high memory footprint.supports up to 2^16 terms 
pub type IndexedRcDataset = OgpsWrapper<GspoWrapper<GenericDataset<u32, RcTermFactory>>>;
/// A heavily indexed dataset. Fast to query but slow to load, with a relatively high memory footprint. supports up to 2^32 terms 
pub type IndexedBigRcDataset = OgpsWrapper<GspoWrapper<GenericDataset<u32, RcTermFactory>>>;

pub type RcDataset = GenericDataset<u16, RcTermFactory>;
pub type BigRcDataset = GenericDataset<u32, RcTermFactory>;
