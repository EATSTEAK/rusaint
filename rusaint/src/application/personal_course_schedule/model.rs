/// 개인의 수업 시간표 정보를 조회합니다.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PersonalCourseSchedule {
    schedule: [[String; 10]; 7],
}

impl PersonalCourseSchedule {
    pub(super) fn new(schedule: [[String; 10]; 7]) -> Self {
        Self { schedule }
    }

    /// 시간표 배열을 반환합니다.
    pub fn schedule(&self) -> &[[String; 10]; 7] {
        &self.schedule
    }
}
