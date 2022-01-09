use std::{ffi::OsStr, ops::Deref, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A struct that denotes a file extension
pub struct FileExtension<'a>(pub &'a str);

impl<'a> Deref for FileExtension<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FileExtension<'a> {
    pub fn from_path(path: &'a Path) -> Option<Self> {
        Some(Self(path.extension().and_then(OsStr::to_str)?))
    }

    pub fn from_path_str(path_str: &'a str) -> Option<Self> {
        Self::from_path(Path::new(path_str))
    }
}

pub const TURTLE: FileExtension = FileExtension("turtle");
pub const TTL: FileExtension = FileExtension("ttl");
pub const NT: FileExtension = FileExtension("nt");
pub const NTRIPLES: FileExtension = FileExtension("ttl");
pub const NQ: FileExtension = FileExtension("nq");
pub const NQUADS: FileExtension = FileExtension("nquads");
pub const RDF: FileExtension = FileExtension("rdf");
pub const RDFXML: FileExtension = FileExtension("rdfxml");
pub const OMN: FileExtension = FileExtension("omn");
pub const OWL: FileExtension = FileExtension("owl");
pub const N3: FileExtension = FileExtension("n3");
pub const TRIG: FileExtension = FileExtension("trig");
pub const JSONLD: FileExtension = FileExtension("jsonld");
pub const JSON: FileExtension = FileExtension("json");
