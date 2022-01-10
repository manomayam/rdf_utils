#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct AnySyntaxError {
    #[from]
    source: anyhow::Error,
}
