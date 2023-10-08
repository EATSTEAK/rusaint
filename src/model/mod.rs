/// 학기 종류 이늄
/// 각 애플리케이션에서의 변환은 애플리케이션 내에서 직접 처리하여야 합니다.
#[derive(Debug)]
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
