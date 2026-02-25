use std::{fmt::Display, ops::Deref, str::FromStr};

use clap::ValueEnum;
use rusaint::application::course_grades::model::CourseType as RusaintCourseType;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct CourseType(RusaintCourseType);

impl Deref for CourseType {
    type Target = RusaintCourseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CourseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self.0 {
            RusaintCourseType::Bachelor => "학사",
            RusaintCourseType::Master => "석사",
            RusaintCourseType::Phd => "박사",
            RusaintCourseType::PhdIntergrated => "석박사통합",
            RusaintCourseType::Research => "연구",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Error)]
pub enum ParseCourseTypeError {
    #[error("invalid course type")]
    InvalidCourseType,
}

impl FromStr for CourseType {
    type Err = ParseCourseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bachelor" => Ok(CourseType(RusaintCourseType::Bachelor)),
            "master" => Ok(CourseType(RusaintCourseType::Master)),
            "phd" => Ok(CourseType(RusaintCourseType::Phd)),
            "phd-integrated" => Ok(CourseType(RusaintCourseType::PhdIntergrated)),
            "research" => Ok(CourseType(RusaintCourseType::Research)),
            _ => Err(ParseCourseTypeError::InvalidCourseType),
        }
    }
}

impl ValueEnum for CourseType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CourseType(RusaintCourseType::Bachelor),
            CourseType(RusaintCourseType::Master),
            CourseType(RusaintCourseType::Phd),
            CourseType(RusaintCourseType::PhdIntergrated),
            CourseType(RusaintCourseType::Research),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            RusaintCourseType::Bachelor => Some(clap::builder::PossibleValue::new("bachelor")),
            RusaintCourseType::Master => Some(clap::builder::PossibleValue::new("master")),
            RusaintCourseType::Phd => Some(clap::builder::PossibleValue::new("phd")),
            RusaintCourseType::PhdIntergrated => {
                Some(clap::builder::PossibleValue::new("phd-integrated"))
            }
            RusaintCourseType::Research => Some(clap::builder::PossibleValue::new("research")),
        }
    }
}
