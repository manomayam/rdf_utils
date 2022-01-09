//! This module defines struct for rdf concrete syntax. It also exports few concrete syntax types
use sophia_api::term::SimpleIri;

pub mod mappings;
pub mod file_extension;
pub mod media_type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A concrete rdf syntax is a syntax in which we can serialize rdf graphs or datasets unambiguously. see [https://www.w3.org/TR/rdf11-concepts/#rdf-documents](https://www.w3.org/TR/rdf11-concepts/#rdf-documents)
pub struct ConcreteRdfSyntax<'a>(SimpleIri<'a>);

///RDF 1.1 Turtle: Terse RDF Triple Language
///
/// Spec: [http://www.w3.org/TR/turtle/](http://www.w3.org/TR/turtle/)
pub const TURTLE: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "http://www.w3.org/TR/",
    Some("turtle/"),
));

///RDF 1.1 XML Syntax
///
/// Spec [https://www.w3.org/TR/rdf-syntax-grammar/](https://www.w3.org/TR/rdf-syntax-grammar/)
pub const RDF_XML: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "http://www.w3.org/TR/",
    Some("rdf-syntax-grammar/"),
));

///Notation3 (N3): A readable RDF syntax
///
/// Spec [https://www.w3.org/TeamSubmission/n3/](https://www.w3.org/TeamSubmission/n3/)
pub const N3: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "http://www.w3.org/TeamSubmission/",
    Some("n3/"),
));

/// RDF 1.1 N-Triples: A line-based syntax for an RDF graph
///
/// Spec: [https://www.w3.org/TR/n-triples/](https://www.w3.org/TR/n-triples/)
pub const N_TRIPLES: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "http://www.w3.org/TR/",
    Some("n-triples/"),
));

/// RDF 1.1 N-Quads: A line-based syntax for RDF datasets
///
/// Spec: [https://www.w3.org/TR/n-quads/](https://www.w3.org/TR/n-quads/)
pub const N_QUADS: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "http://www.w3.org/TR/",
    Some("n-quads/"),
));

/// OWL 2 Web Ontology Language XML Serialization (Second Edition)
///
/// Spec: [http://www.w3.org/TR/owl2-xml-serialization/](http://www.w3.org/TR/owl2-xml-serialization/)
pub const OWL2_XML: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "https://www.w3.org/TR/",
    Some("owl2-xml-serialization/"),
));

/// OWL 2 Web Ontology Language Manchester Syntax (Second Edition)
///
/// Spec: [http://www.w3.org/TR/owl2-manchester-syntax/](http://www.w3.org/TR/owl2-manchester-syntax/)
pub const OWL2_MANCHESTER: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "https://www.w3.org/TR/",
    Some("owl2-manchester-syntax/"),
));

/// RDF 1.1 TriG: RDF Dataset Language
///
/// Spec: [https://www.w3.org/TR/trig/](https://www.w3.org/TR/trig/)
pub const TRIG: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "https://www.w3.org/TR/",
    Some("trig/"),
));

/// JSON-LD 1.1: A JSON-based Serialization for Linked Data
///
/// Spec: [https://www.w3.org/TR/json-ld/](https://www.w3.org/TR/json-ld/)
pub const JSON_LD: ConcreteRdfSyntax = ConcreteRdfSyntax(SimpleIri::new_unchecked(
    "https://www.w3.org/TR/",
    Some("json-ld/"),
));
