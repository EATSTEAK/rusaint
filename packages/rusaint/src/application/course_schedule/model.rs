use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, value::MapDeserializer},
};

use crate::application::course_schedule::utils::{
    request, request_lv1, request_lv2, request_lv3, request_text,
};
use crate::application::utils::de_with::deserialize_optional_string;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    define_elements,
    webdynpro::{
        client::WebDynproClient,
        element::{
            action::Button, complex::sap_table::FromSapTable, definition::ElementDefinition,
            layout::tab_strip::item::TabStripItem, selection::ComboBox,
        },
        error::{ElementError, WebDynproError},
    },
};

/// 강의를 찾을 때 사용하는 강의 카테고리
#[allow(unused)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Serialize, Deserialize)]
pub enum LectureCategory {
    /// 전공 강의
    Major {
        /// 단과대명
        collage: String,
        /// 학부명
        department: String,
        /// 전공명
        major: Option<String>,
    },
    /// 교양필수
    RequiredElective {
        /// 과목명
        lecture_name: String,
    },
    /// 교양선택
    OptionalElective {
        /// 교양 분류
        category: String,
    },
    /// 채플
    Chapel {
        /// 과목명
        lecture_name: String,
    },
    /// 교직
    Education,
    /// 대학원
    Graduated {
        /// 단과대명
        collage: String,
        /// 학부명
        department: String,
    },
    /// 연계전공
    ConnectedMajor {
        /// 전공명
        major: String,
    },
    /// 융합전공
    UnitedMajor {
        /// 전공명
        major: String,
    },
    /// 교수명 검색
    FindByProfessor {
        /// 교수명
        keyword: String,
    },
    /// 과목명 검색
    FindByLecture {
        /// 과목명
        keyword: String,
    },
    /// 타전공인정과목
    RecognizedOtherMajor {
        /// 단과대명
        collage: String,
        /// 학부명
        department: String,
        /// 전공명
        major: Option<String>,
    },
    /// 숭실사이버대
    Cyber,
}

impl LectureCategory {
    /// 전공과목 분류의 [`LectureCategory`]를 만듭니다.
    pub fn major(collage: &str, department: &str, major: Option<&str>) -> Self {
        Self::Major {
            collage: collage.to_string(),
            department: department.to_string(),
            major: major.map(|str| str.to_string()),
        }
    }

    /// 교양필수 분류의 [`LectureCategory`]를 만듭니다.
    pub fn required_elective(lecture_name: &str) -> Self {
        Self::RequiredElective {
            lecture_name: lecture_name.to_string(),
        }
    }

    /// 교양선택 분류의 [`LectureCategory`]를 만듭니다.
    pub fn optional_elective(category: &str) -> Self {
        Self::OptionalElective {
            category: category.to_string(),
        }
    }

    /// 채플 분류의 [`LectureCategory`]를 만듭니다.
    pub fn chapel(lecture_name: &str) -> Self {
        Self::Chapel {
            lecture_name: lecture_name.to_string(),
        }
    }

    /// 교직 분류의 [`LectureCategory`]를 만듭니다.
    pub fn education() -> Self {
        Self::Education
    }

    /// 대학원 분류의 [`LectureCategory`]를 만듭니다.
    pub fn graduated(collage: &str, department: &str) -> Self {
        Self::Graduated {
            collage: collage.to_string(),
            department: department.to_string(),
        }
    }

    /// 연계전공 분류의 [`LectureCategory`]를 만듭니다.
    pub fn connected_major(major: &str) -> Self {
        Self::ConnectedMajor {
            major: major.to_string(),
        }
    }

    /// 융합전공 분류의 [`LectureCategory`]를 만듭니다.
    pub fn united_major(major: &str) -> Self {
        Self::UnitedMajor {
            major: major.to_string(),
        }
    }

    /// 교수명으로 찾기 위한 [`LectureCategory`]를 만듭니다.
    pub fn find_by_professor(keyword: &str) -> Self {
        Self::FindByProfessor {
            keyword: keyword.to_string(),
        }
    }

    /// 과목명으로 찾기 위한 [`LectureCategory`]를 만듭니다.
    pub fn find_by_lecture(keyword: &str) -> Self {
        Self::FindByLecture {
            keyword: keyword.to_string(),
        }
    }

    /// 타전공인정과목 분류의 [`LectureCategory`]를 만듭니다.
    pub fn recognized_other_major(collage: &str, department: &str, major: Option<&str>) -> Self {
        Self::RecognizedOtherMajor {
            collage: collage.to_string(),
            department: department.to_string(),
            major: major.map(|str| str.to_string()),
        }
    }

    /// 숭실사이버대 분류의 [`LectureCategory`]를 만듭니다.
    pub fn cyber() -> Self {
        Self::Cyber
    }

    pub(super) async fn request_query(
        &self,
        client: &mut WebDynproClient,
    ) -> Result<(), WebDynproError> {
        match self {
            LectureCategory::Major {
                collage,
                department,
                major,
            } => {
                // 학부전공별
                define_elements! {
                    TAB_OTHERS: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_OTHERS";
                    OTHERS_DDK_LV3: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV3";
                    OTHERS_DDK_LV4: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV4";
                    OTHERS_DDK_LV5: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.DDK_LV5";
                    SEARCH_OTHERS: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHERS.BUTTON";
                }
                if let Some(major) = major {
                    request_lv3(
                        client,
                        TAB_OTHERS,
                        0,
                        OTHERS_DDK_LV3,
                        OTHERS_DDK_LV4,
                        OTHERS_DDK_LV5,
                        SEARCH_OTHERS,
                        collage,
                        department,
                        major,
                    )
                    .await?;
                } else {
                    request_lv2(
                        client,
                        TAB_OTHERS,
                        0,
                        OTHERS_DDK_LV3,
                        OTHERS_DDK_LV4,
                        SEARCH_OTHERS,
                        collage,
                        department,
                    )
                    .await?;
                }
            }
            LectureCategory::RequiredElective { lecture_name } => {
                // 교양필수
                define_elements! {
                    TAB_GENERAL_REQ: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GENERAL_REQ";
                    GENERAL_REQ_TYPE: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_REQ.SM_OBJID";
                    SEARCH_GENERAL_REQ: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_REQ.BUTTON_SEARCH";
                }
                request_lv1(
                    client,
                    TAB_GENERAL_REQ,
                    1,
                    GENERAL_REQ_TYPE,
                    SEARCH_GENERAL_REQ,
                    lecture_name,
                )
                .await?;
            }
            LectureCategory::OptionalElective { category } => {
                // 교양선택
                define_elements! {
                    TAB_GENERAL_OPT: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GENERAL_OPT";
                    GENERAL_OPT_DISCIPLINES: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_OPT.DISCIPLINES";
                    SEARCH_GENERAL_OPT: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_GENERAL_OPT.BUTTON_SEARCH";
                }
                request_lv1(
                    client,
                    TAB_GENERAL_OPT,
                    2,
                    GENERAL_OPT_DISCIPLINES,
                    SEARCH_GENERAL_OPT,
                    category,
                )
                .await?;
            }
            LectureCategory::Chapel { lecture_name } => {
                // 채플
                define_elements! {
                    TAB_CHAPEL: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_CHAPEL_REQ";
                    CHAPEL_TYPE: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_CHAPEL_REQ.SM_OBJID";
                    SEARCH_CHAPEL: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_CHAPEL_REQ.BUTTON_SEARCH";
                }
                request_lv1(
                    client,
                    TAB_CHAPEL,
                    3,
                    CHAPEL_TYPE,
                    SEARCH_CHAPEL,
                    lecture_name,
                )
                .await?;
            }
            LectureCategory::Education => {
                // 교직
                define_elements! {
                    TAB_EDU: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_EDU";
                    SEARCH_EDU: Button<'_> = "ZCMW2100.ID_0001:VIW_MAIN.BUTTON_EDU";
                }
                request(client, TAB_EDU, 4, SEARCH_EDU).await?;
            }
            LectureCategory::Graduated {
                collage,
                department,
            } => {
                // 대학원
                define_elements! {
                    TAB_GRADUATE: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_GRADUATE";
                    GRADUATE_DDK_LV3: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.DDK_LV3";
                    GRADUATE_DDK_LV4: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.DDK_LV4";
                    SEARCH_GRADUATE: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_GRADUATE.BUTTON";
                }
                request_lv2(
                    client,
                    TAB_GRADUATE,
                    7,
                    GRADUATE_DDK_LV3,
                    GRADUATE_DDK_LV4,
                    SEARCH_GRADUATE,
                    collage,
                    department,
                )
                .await?;
            }
            LectureCategory::ConnectedMajor { major } => {
                // 연계전공
                define_elements! {
                    TAB_YOMA: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_YOMA";
                    COMBO_YOMA: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_YOMA.CONNECT_MAJO";
                    SEARCH_YOMA: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_YOMA.BUTTON_SEARCH";
                }
                request_lv1(client, TAB_YOMA, 8, COMBO_YOMA, SEARCH_YOMA, major).await?;
            }
            LectureCategory::UnitedMajor { major } => {
                // 융합전공
                define_elements! {
                    TAB_UNMA: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_UNMA";
                    COMBO_UNMA: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_UNMA.CG_OBJID";
                    SEARCH_UNMA: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_UNMA.BUTTON_SEARCH";
                }
                request_lv1(client, TAB_UNMA, 9, COMBO_UNMA, SEARCH_UNMA, major).await?;
            }
            LectureCategory::FindByProfessor { keyword } => {
                // 교수명검색
                define_elements! {
                    TAB_PROFESSOR: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_PROFESSOR";
                    COMBO_PROFESSOR: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_PROFESSOR.PROFESSOR"; // TODO: implement ComboBoxString
                    SEARCH_PROFESSOR: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_PROFESSOR.BUTTON_SEARCH";
                }
                request_text(
                    client,
                    TAB_PROFESSOR,
                    10,
                    COMBO_PROFESSOR,
                    SEARCH_PROFESSOR,
                    keyword,
                )
                .await?;
            }
            LectureCategory::FindByLecture { keyword } => {
                // 과목검색
                define_elements! {
                    TAB_SEARCH: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_SEARCH";
                    COMBO_SEARCH: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_SEARCH.SEARCH_TEXT";
                    SEARCH_SEARCH: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_SEARCH.BUTTON_SEARCH";
                }
                request_text(client, TAB_SEARCH, 11, COMBO_SEARCH, SEARCH_SEARCH, keyword).await?;
            }
            LectureCategory::RecognizedOtherMajor {
                collage,
                department,
                major,
            } => {
                // 타전공인정과목
                define_elements! {
                    TAB_OTHER_GC: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_OTHER_GC";
                    OTHER_GC_DDK_LV3: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.DDK_LV3";
                    OTHER_GC_DDK_LV4: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.DDK_LV4";
                    OTHER_GC_DDK_LV5: ComboBox<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.DDK_LV5";
                    SEARCH_OTHER_GC: Button<'_> = "ZCMW2100.ID_0001:VIW_TAB_OTHER_GC.BTN_OTHER_GC";
                }
                if let Some(major) = major {
                    request_lv3(
                        client,
                        TAB_OTHER_GC,
                        12,
                        OTHER_GC_DDK_LV3,
                        OTHER_GC_DDK_LV4,
                        OTHER_GC_DDK_LV5,
                        SEARCH_OTHER_GC,
                        collage,
                        department,
                        major,
                    )
                    .await?;
                } else {
                    request_lv2(
                        client,
                        TAB_OTHER_GC,
                        12,
                        OTHER_GC_DDK_LV3,
                        OTHER_GC_DDK_LV4,
                        SEARCH_OTHER_GC,
                        collage,
                        department,
                    )
                    .await?;
                }
            }
            LectureCategory::Cyber => {
                // 숭실사이버대
                define_elements! {
                    TAB_CYBER: TabStripItem<'_> = "ZCMW2100.ID_0001:VIW_MAIN.TAB_CYBER";
                    SEARCH_CYBER: Button<'_> = "ZCMW2100.ID_0001:VIW_MAIN.BTN_CYBER";
                }
                request(client, TAB_CYBER, 14, SEARCH_CYBER).await?;
            }
        }
        Ok(())
    }
}

/// 과목 정보
#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Lecture {
    /// 계획
    #[serde(
        rename(deserialize = "계획"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    syllabus: Option<String>,
    /// 이수구분(주전공)
    #[serde(rename(deserialize = "이수구분(주전공)"))]
    category: String,
    /// 이수구분(다전공)
    #[serde(
        rename(deserialize = "이수구분(다전공)"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    sub_category: Option<String>,
    /// 공학인증
    #[serde(
        rename(deserialize = "공학인증"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    abeek_info: Option<String>,
    /// 교과영역
    #[serde(
        rename(deserialize = "교과영역"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    field: Option<String>,
    /// 과목번호
    #[serde(rename(deserialize = "과목번호"))]
    code: String,
    /// 과목명
    #[serde(rename(deserialize = "과목명"))]
    name: String,
    /// 분반
    #[serde(
        rename(deserialize = "분반"),
        default,
        deserialize_with = "deserialize_optional_string"
    )]
    division: Option<String>,
    /// 교수명
    #[serde(rename(deserialize = "교수명"))]
    professor: String,
    /// 개설학과
    #[serde(rename(deserialize = "개설학과"))]
    department: String,
    /// 시간/학점(설계)
    #[serde(rename(deserialize = "시간/학점(설계)"))]
    time_points: String,
    /// 수강인원
    #[serde(rename(deserialize = "수강인원"))]
    personeel: String,
    /// 여석
    #[serde(rename(deserialize = "여석"))]
    remaining_seats: String,
    /// 강의시간(강의실)
    #[serde(rename(deserialize = "강의시간(강의실)"))]
    schedule_room: String,
    /// 수강대상
    #[serde(rename(deserialize = "수강대상"))]
    target: String,
}

impl<'body> FromSapTable<'body> for Lecture {
    fn from_table(
        header: &'body crate::webdynpro::element::complex::sap_table::SapTableHeader,
        row: &'body crate::webdynpro::element::complex::sap_table::SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, parser)?;
        let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
        Ok(
            Lecture::deserialize(map_de).map_err(|e| ElementError::InvalidContent {
                element: row.table_def().id().to_string(),
                content: e.to_string(),
            })?,
        )
    }
}

#[cfg(feature = "uniffi")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
/// 새로운 `LectureCategory`를 만드는 빌더입니다.
pub struct LectureCategoryBuilder {}

#[cfg(feature = "uniffi")]
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl LectureCategoryBuilder {
    #[uniffi::constructor]
    /// `LectureCategoryBuilder`를 만듭니다.
    pub fn new() -> Self {
        Self {}
    }

    /// 전공과목 분류의 [`LectureCategory`]를 만듭니다.
    pub fn major(
        &self,
        collage: &str,
        department: &str,
        major: &Option<String>,
    ) -> LectureCategory {
        LectureCategory::major(collage, department, major.as_ref().map(String::as_str))
    }

    /// 교양필수 분류의 [`LectureCategory`]를 만듭니다.
    pub fn required_elective(&self, lecture_name: &str) -> LectureCategory {
        LectureCategory::required_elective(lecture_name)
    }

    /// 교양선택 분류의 [`LectureCategory`]를 만듭니다.
    pub fn optional_elective(&self, category: &str) -> LectureCategory {
        LectureCategory::optional_elective(category)
    }

    /// 채플 분류의 [`LectureCategory`]를 만듭니다.
    pub fn chapel(&self, lecture_name: &str) -> LectureCategory {
        LectureCategory::chapel(lecture_name)
    }

    /// 교직 분류의 [`LectureCategory`]를 만듭니다.
    pub fn education(&self) -> LectureCategory {
        LectureCategory::education()
    }

    /// 대학원 분류의 [`LectureCategory`]를 만듭니다.
    pub fn graduated(&self, collage: &str, department: &str) -> LectureCategory {
        LectureCategory::graduated(collage, department)
    }

    /// 연계전공 분류의 [`LectureCategory`]를 만듭니다.
    pub fn connected_major(&self, major: &str) -> LectureCategory {
        LectureCategory::connected_major(major)
    }

    /// 융합전공 분류의 [`LectureCategory`]를 만듭니다.
    pub fn united_major(&self, major: &str) -> LectureCategory {
        LectureCategory::united_major(major)
    }

    /// 교수명으로 찾기 위한 [`LectureCategory`]를 만듭니다.
    pub fn find_by_professor(&self, keyword: &str) -> LectureCategory {
        LectureCategory::find_by_professor(keyword)
    }

    /// 과목명으로 찾기 위한 [`LectureCategory`]를 만듭니다.
    pub fn find_by_lecture(&self, keyword: &str) -> LectureCategory {
        LectureCategory::find_by_lecture(keyword)
    }

    /// 타전공인정과목 분류의 [`LectureCategory`]를 만듭니다.
    pub fn recognized_other_major(
        &self,
        collage: &str,
        department: &str,
        major: &Option<String>,
    ) -> LectureCategory {
        LectureCategory::recognized_other_major(
            collage,
            department,
            major.as_ref().map(String::as_str),
        )
    }

    /// 숭실사이버대 분류의 [`LectureCategory`]를 만듭니다.
    pub fn cyber(&self) -> LectureCategory {
        LectureCategory::cyber()
    }
}

#[cfg(feature = "uniffi")]
impl Default for LectureCategoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
