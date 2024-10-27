/// Rusaint에서 반환하는 기본 오류
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum RusaintError {
    #[error(transparent)]
    General(#[from] rusaint::RusaintError),
}
