use model::{GraduationRequirement, GraduationRequirementsInfo, GraduationStudentInfo};

use crate::{
    define_elements,
    webdynpro::{
        client::body::Body,
        command::element::action::ButtonPressCommand,
        element::{
            action::Button,
            complex::SapTable,
            definition::ElementDefinition,
            text::{InputField, InputFieldDef},
            Element,
        },
        error::{ElementError, WebDynproError},
    },
    RusaintError,
};

use super::{USaintApplication, USaintClient};

/// [졸업사정표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW8015)
pub struct GraduationRequirements {
    client: USaintClient,
}

impl USaintApplication for GraduationRequirements {
    const APP_NAME: &'static str = "ZCMW8015";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> GraduationRequirements {
    // 학생정보
    define_elements! {
        // 학번
        STUDENT_NUM: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_STUDENT12";
        // 성명
        STUDENT_NAME: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_STNAME";
        // 학년
        STUDENT_GRADE: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_GRADE";
        // 이수학기
        PRCL: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_PRCL";
        // 학적상태
        STATUS: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_STATUST";
        // 입학년도
        APPLY_YEAR: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_APPLY_PERYR";
        // 입학유형
        NEWINCOR_CDT: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_NEWINCOR_CDT";
        // 소속
        CG_IDT_DEPT: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT_DEPT";
        // 제 1~4전공
        CG_IDT1: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT1";
        CG_IDT2: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT2";
        CG_IDT3: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT3";
        CG_IDT4: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_CG_IDT4";
        // 졸업사정일자
        AUDIT_DATE: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_AUDIT_DAT";
        // 졸업학점
        GR_CPOP: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_GR_CPOP";
        // 인정학점
        COMP_CPOP: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_COMP_CPOP";
    }

    // 졸업사정결과
    define_elements! {
        AUDIT_RESULT: InputField<'a> = "ZCMW8015.ID_0001:MAIN.TC01_AUDIT_RESULT_T";
        SHOW_DETAILS: Button<'a> = "ZCMW8015.ID_0001:MAIN.BTN_DTL";
        MAIN_TABLE: SapTable<'a> = "ZCMW8015.ID_0001:MAIN.TABLE";
    }

    fn parse_str_from_inputfield(input_field: &InputField<'_>) -> Result<String, ElementError> {
        Ok(input_field
            .value()
            .ok_or_else(|| ElementError::NoSuchContent {
                element: input_field.id().to_owned(),
                content: "value of InputField".to_string(),
            })?
            .to_owned())
    }

    fn parse_u32_from_inputfield(input_field: &InputField<'_>) -> Result<u32, ElementError> {
        Self::parse_str_from_inputfield(input_field)?
            .parse::<u32>()
            .or_else(|_| {
                Err(ElementError::InvalidContent {
                    element: input_field.id().to_owned(),
                    content: "value is not correct u32".to_string(),
                })
            })
    }

    fn parse_f32_from_inputfield(input_field: &InputField<'_>) -> Result<f32, ElementError> {
        Self::parse_str_from_inputfield(input_field)?
            .parse::<f32>()
            .or_else(|_| {
                Err(ElementError::InvalidContent {
                    element: input_field.id().to_owned(),
                    content: "value is not correct f32".to_string(),
                })
            })
    }

    /// 학생 정보를 반환합니다.
    pub async fn student_info(&self) -> Result<GraduationStudentInfo, WebDynproError> {
        let number = Self::parse_u32_from_inputfield(&Self::STUDENT_NUM.from_body(self.body())?)?;
        let name = &Self::parse_str_from_inputfield(&Self::STUDENT_NAME.from_body(self.body())?)?;
        let grade = Self::parse_u32_from_inputfield(&Self::STUDENT_GRADE.from_body(self.body())?)?;
        let semester = Self::parse_u32_from_inputfield(&Self::PRCL.from_body(self.body())?)?;
        let status = &Self::parse_str_from_inputfield(&Self::STATUS.from_body(self.body())?)?;
        let apply_year =
            Self::parse_u32_from_inputfield(&Self::APPLY_YEAR.from_body(self.body())?)?;
        let apply_type =
            &Self::parse_str_from_inputfield(&Self::NEWINCOR_CDT.from_body(self.body())?)?;
        let department =
            &Self::parse_str_from_inputfield(&Self::CG_IDT_DEPT.from_body(self.body())?)?;
        let mut majors = Vec::new();
        const IDTS: &[InputFieldDef] = &[
            GraduationRequirements::CG_IDT1,
            GraduationRequirements::CG_IDT2,
            GraduationRequirements::CG_IDT3,
            GraduationRequirements::CG_IDT4,
        ];
        for idt in IDTS {
            let major = Self::parse_str_from_inputfield(&idt.from_body(self.body())?).ok();
            if let Some(major) = major {
                if !major.trim().is_empty() {
                    majors.push(major);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let audit_date =
            &Self::parse_str_from_inputfield(&Self::AUDIT_DATE.from_body(self.body())?)?;
        let graduation_points =
            Self::parse_f32_from_inputfield(&Self::GR_CPOP.from_body(self.body())?)?;
        let completed_points =
            Self::parse_f32_from_inputfield(&Self::COMP_CPOP.from_body(self.body())?)?;
        Ok(GraduationStudentInfo::new(
            number,
            name,
            grade,
            semester,
            status,
            apply_year,
            apply_type,
            department,
            majors,
            audit_date,
            graduation_points,
            completed_points,
        ))
    }

    /// 졸업사정 결과와 졸업 필요 요건별 충족 여부와 세부 정보를 반환합니다.
    pub async fn requirements(&mut self) -> Result<GraduationRequirementsInfo, WebDynproError> {
        self.client
            .send(ButtonPressCommand::new(Self::SHOW_DETAILS))
            .await?;
        let audit_result = Self::AUDIT_RESULT
            .from_body(self.body())?
            .lsdata()
            .value()
            .and_then(|str| Some(str == "가능"))
            .unwrap_or(false);
        let table_element = Self::MAIN_TABLE.from_body(self.body())?;
        let table = table_element.table()?;
        let requirements = table
            .try_table_into::<GraduationRequirement>(self.body())?
            .into_iter()
            .map(|req| (req.name().to_owned(), req))
            .collect();
        Ok(GraduationRequirementsInfo::new(audit_result, requirements))
    }
    fn body(&self) -> &Body {
        self.client.body()
    }
}

/// 졸업사정표 애플리케이션에서 사용되는 데이터의 모듈
pub mod model;

#[cfg(test)]
mod test {

    use serial_test::serial;

    use crate::{
        application::{graduation_requirements::GraduationRequirements, USaintClientBuilder},
        global_test_utils::get_session,
        webdynpro::element::definition::ElementDefinition,
    };

    #[tokio::test]
    #[serial]
    async fn read_table() {
        let session = get_session().await.unwrap();
        let app = USaintClientBuilder::new()
            .session(session)
            .build_into::<GraduationRequirements>()
            .await
            .unwrap();
        let table_element = GraduationRequirements::MAIN_TABLE
            .from_body(app.body())
            .unwrap();
        let table = table_element
            .table()
            .unwrap()
            .try_table_into::<Vec<(String, String)>>(app.body())
            .unwrap();
        dbg!(table);
    }
}
