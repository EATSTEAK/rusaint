/// 학기 종류
///
/// 각 애플리케이션에서의 변환은 애플리케이션 내에서 직접 처리하여야 합니다.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum SemesterType {
    /// 1학기
    One,
    /// 여름학기
    Summer,
    /// 2학기
    Two,
    /// 겨울학기
    Winter,
}

impl ToString for SemesterType {
    fn to_string(&self) -> String {
        match self {
            Self::One => "1학기".to_string(),
            Self::Summer => "여름학기".to_string(),
            Self::Two => "2학기".to_string(),
            Self::Winter => "겨울학기".to_string(),
        }
    }
}
