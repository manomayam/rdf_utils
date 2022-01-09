use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::{media_type, ConcreteRdfSyntax, file_extension::{self, FileExtension}};

pub const SYNTAX_TO_EXTENSION_MAP: Lazy<HashMap<ConcreteRdfSyntax, FileExtension>> =
    Lazy::new(|| {
        let mut map: HashMap<ConcreteRdfSyntax, FileExtension> = HashMap::new();
        map.insert(super::JSON_LD, file_extension::JSONLD).unwrap();
        map.insert(super::N3, file_extension::N3).unwrap();
        map.insert(super::N_QUADS, file_extension::NQ).unwrap();
        map.insert(super::N_TRIPLES, file_extension::NT).unwrap();
        map.insert(super::OWL2_MANCHESTER, file_extension::OMN).unwrap();
        map.insert(super::OWL2_XML, file_extension::OWL).unwrap();
        map.insert(super::RDF_XML, file_extension::RDF).unwrap();
        map.insert(super::TRIG, file_extension::TRIG).unwrap();
        map.insert(super::TURTLE, file_extension::TTL).unwrap();
        map
    });

pub const EXTENSION_TO_SYNTAX_MAP: Lazy<HashMap<FileExtension, ConcreteRdfSyntax>> =
    Lazy::new(|| {
        let mut map: HashMap<FileExtension, ConcreteRdfSyntax> = HashMap::new();
        map.insert(file_extension::JSONLD, super::JSON_LD).unwrap();
        map.insert(file_extension::JSON, super::JSON_LD).unwrap();
        map.insert(file_extension::N3, super::N3).unwrap();
        map.insert(file_extension::NQ, super::N_QUADS).unwrap();
        map.insert(file_extension::NQUADS, super::N_QUADS).unwrap();
        map.insert(file_extension::NT, super::N_TRIPLES).unwrap();
        map.insert(file_extension::NTRIPLES, super::N_TRIPLES).unwrap();
        map.insert(file_extension::OMN, super::OWL2_MANCHESTER).unwrap();
        map.insert(file_extension::OWL, super::OWL2_XML).unwrap();
        map.insert(file_extension::RDF, super::RDF_XML).unwrap();
        map.insert(file_extension::RDFXML, super::RDF_XML).unwrap();
        map.insert(file_extension::TRIG, super::TRIG).unwrap();
        map.insert(file_extension::TTL, super::TURTLE).unwrap();
        map.insert(file_extension::TURTLE, super::TURTLE).unwrap();
        map
    });

pub const SYNTAX_TO_MEDIA_TYPE_MAP: Lazy<HashMap<ConcreteRdfSyntax, &'static mime::Mime>> =
    Lazy::new(|| {
        let mut map: HashMap<ConcreteRdfSyntax, &'static mime::Mime> = HashMap::new();
        map.insert(super::JSON_LD, &media_type::APPLICATION_JSON_LD);
        map.insert(super::N3, &media_type::TEXT_N3);
        map.insert(super::N_QUADS, &media_type::APPLICATION_N_QUADS);
        map.insert(super::N_TRIPLES, &media_type::APPLICATION_N_TRIPLES);
        map.insert(super::OWL2_MANCHESTER, &media_type::TEXT_OWL_MANCHESTER);
        map.insert(super::OWL2_XML, &media_type::APPLICATION_RDF_XML);
        map.insert(super::RDF_XML, &media_type::APPLICATION_RDF_XML);
        map.insert(super::TRIG, &media_type::APPLICATION_TRIG);
        map.insert(super::TURTLE, &media_type::TEXT_TURTLE);
        map
    });

pub const MEDIA_TYPE_TO_SYNTAX_MAP: Lazy<HashMap<&'static mime::Mime, ConcreteRdfSyntax>> =
    Lazy::new(|| {
        let mut map: HashMap<&'static mime::Mime, ConcreteRdfSyntax> = HashMap::new();
        map.insert(&media_type::APPLICATION_JSON_LD, super::JSON_LD);
        map.insert(&media_type::TEXT_N3, super::N3);
        map.insert(&media_type::APPLICATION_N_QUADS, super::N_QUADS);
        map.insert(&media_type::APPLICATION_N_TRIPLES, super::N_TRIPLES);
        map.insert(&media_type::TEXT_OWL_MANCHESTER, super::OWL2_MANCHESTER);
        map.insert(&media_type::APPLICATION_RDF_XML, super::RDF_XML);
        map.insert(&media_type::APPLICATION_TRIG, super::TRIG);
        map.insert(&media_type::TEXT_TURTLE, super::TURTLE);
        map
    });
