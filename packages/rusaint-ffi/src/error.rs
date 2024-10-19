#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum RusaintError {
    #[error(transparent)]
    General(#[from] rusaint::RusaintError),
}
