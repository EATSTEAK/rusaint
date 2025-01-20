use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 한 주의 요일을 표현합니다.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum Weekday {
    /// 월요일
    Mon = 0,
    /// 화요일
    Tue = 1,
    /// 수요일
    Wed = 2,
    /// 목요일
    Thu = 3,
    /// 금요일
    Fri = 4,
    /// 토요일
    Sat = 5,
    /// 일요일
    Sun = 6,
}

/// 개인의 수업 시간표 정보를 조회합니다.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PersonalCourseSchedule {
    schedule: HashMap<Weekday, Vec<CourseScheduleInformation>>,
}

/// 강의의 시간표 정보입니다.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CourseScheduleInformation {
    name: String,
    professor: String,
    time: String,
    classroom: String,
}

impl CourseScheduleInformation {
    pub(crate) fn from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> CourseScheduleInformation {
        let mut iter = iter.skip_while(|s| s.is_empty());
        // Consume empty strings at start
        CourseScheduleInformation {
            name: iter.next().unwrap().to_string(),
            professor: iter.next().unwrap().to_string(),
            time: iter.next().unwrap().to_string(),
            classroom: iter.next().unwrap_or("").to_string(),
        }
    }

    /// 강의명을 반환합니다.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 강의 교수자를 반환합니다.
    pub fn professor(&self) -> &str {
        &self.professor
    }

    /// 강의 시간을 반환합니다. hh:mm-hh:mm 형태입니다.
    pub fn time(&self) -> &str {
        &self.time
    }

    /// 강의실을 반환합니다.
    pub fn classroom(&self) -> &str {
        &self.classroom
    }
}

impl PersonalCourseSchedule {
    pub(super) fn new(schedule: HashMap<Weekday, Vec<CourseScheduleInformation>>) -> Self {
        Self { schedule }
    }

    /// 시간표 배열을 반환합니다.
    pub fn schedule(&self) -> &HashMap<Weekday, Vec<CourseScheduleInformation>> {
        &self.schedule
    }
}
