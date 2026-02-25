use std::{fmt::Display, ops::Deref, str::FromStr};

use clap::ValueEnum;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct CourseType(rusaint::application::course_grades::model::CourseType);

impl Deref for CourseType {
    type Target = rusaint::application::course_grades::model::CourseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CourseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self.0 {
            rusaint::application::course_grades::model::CourseType::Bachelor => "학사",
            rusaint::application::course_grades::model::CourseType::Master => "석사",
            rusaint::application::course_grades::model::CourseType::Phd => "박사",
            rusaint::application::course_grades::model::CourseType::PhdIntergrated => "석박사통합",
            rusaint::application::course_grades::model::CourseType::Research => "연구",
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
            "bachelor" => Ok(CourseType(
                rusaint::application::course_grades::model::CourseType::Bachelor,
            )),
            "master" => Ok(CourseType(
                rusaint::application::course_grades::model::CourseType::Master,
            )),
            "phd" => Ok(CourseType(
                rusaint::application::course_grades::model::CourseType::Phd,
            )),
            "phd-integrated" => Ok(CourseType(
                rusaint::application::course_grades::model::CourseType::PhdIntergrated,
            )),
            "research" => Ok(CourseType(
                rusaint::application::course_grades::model::CourseType::Research,
            )),
            _ => Err(ParseCourseTypeError::InvalidCourseType),
        }
    }
}

impl ValueEnum for CourseType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CourseType(rusaint::application::course_grades::model::CourseType::Bachelor),
            CourseType(rusaint::application::course_grades::model::CourseType::Master),
            CourseType(rusaint::application::course_grades::model::CourseType::Phd),
            CourseType(rusaint::application::course_grades::model::CourseType::PhdIntergrated),
            CourseType(rusaint::application::course_grades::model::CourseType::Research),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self.0 {
            rusaint::application::course_grades::model::CourseType::Bachelor => {
                Some(clap::builder::PossibleValue::new("bachelor"))
            }
            rusaint::application::course_grades::model::CourseType::Master => {
                Some(clap::builder::PossibleValue::new("master"))
            }
            rusaint::application::course_grades::model::CourseType::Phd => {
                Some(clap::builder::PossibleValue::new("phd"))
            }
            rusaint::application::course_grades::model::CourseType::PhdIntergrated => {
                Some(clap::builder::PossibleValue::new("phd-integrated"))
            }
            rusaint::application::course_grades::model::CourseType::Research => {
                Some(clap::builder::PossibleValue::new("research"))
            }
        }
    }
}
