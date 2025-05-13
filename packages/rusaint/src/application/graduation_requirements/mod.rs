use model::{GraduationRequirement, GraduationRequirements, GraduationStudent};

use super::{USaintApplication, USaintClient};
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::element::parser::ElementParser;
use crate::{
    RusaintError, define_elements,
    webdynpro::{
        client::body::Body,
        command::element::{
            action::ButtonPressEventCommand, complex::SapTableBodyCommand,
            text::InputFieldValueCommand,
        },
        element::{
            action::Button,
            complex::SapTable,
            text::{InputField, InputFieldDef},
        },
    },
};

/// [졸업사정표](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW8015)
#[derive(Debug)]
pub struct GraduationRequirementsApplication {
    client: USaintClient,
}

impl USaintApplication for GraduationRequirementsApplication {
    const APP_NAME: &'static str = "ZCMW8015";

    fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
        if client.name() != Self::APP_NAME {
            Err(RusaintError::InvalidClientError)
        } else {
            Ok(Self { client })
        }
    }
}

impl<'a> GraduationRequirementsApplication {
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

    /// 학생 정보를 반환합니다.
    pub async fn student_info(&self) -> Result<GraduationStudent, RusaintError> {
        let parser = ElementParser::new(self.body());
        let number = parser
            .element_from_def(&Self::STUDENT_NUM)?
            .value_into_u32()?;
        let name = &parser
            .element_from_def(&Self::STUDENT_NAME)?
            .value_string()?;
        let grade = parser
            .element_from_def(&Self::STUDENT_GRADE)?
            .value_into_u32()?;
        let semester = parser.element_from_def(&Self::PRCL)?.value_into_u32()?;
        let status = &parser.element_from_def(&Self::STATUS)?.value_string()?;
        let apply_year = parser
            .element_from_def(&Self::APPLY_YEAR)?
            .value_into_u32()?;
        let apply_type = &parser
            .element_from_def(&Self::NEWINCOR_CDT)?
            .value_string()?;
        let department = &parser
            .element_from_def(&Self::CG_IDT_DEPT)?
            .value_string()?;
        let mut majors = Vec::new();
        const IDTS: &[InputFieldDef] = &[
            GraduationRequirementsApplication::CG_IDT1,
            GraduationRequirementsApplication::CG_IDT2,
            GraduationRequirementsApplication::CG_IDT3,
            GraduationRequirementsApplication::CG_IDT4,
        ];
        for idt in IDTS {
            let major = parser.element_from_def(idt)?.value_string().ok();
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
        let audit_date = &parser.element_from_def(&Self::AUDIT_DATE)?.value_string()?;
        let graduation_points = parser.element_from_def(&Self::GR_CPOP)?.value_into_f32()?;
        let completed_points = parser
            .element_from_def(&Self::COMP_CPOP)?
            .value_into_f32()?;
        Ok(GraduationStudent::new(
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
    pub async fn requirements(&mut self) -> Result<GraduationRequirements, RusaintError> {
        {
            let event = ElementParser::new(self.body())
                .read(ButtonPressEventCommand::new(Self::SHOW_DETAILS))?;
            self.client.process_event(false, event).await?;
        }
        let parser = ElementParser::new(self.body());
        let audit_result = parser
            .read(InputFieldValueCommand::new(Self::AUDIT_RESULT))
            .is_ok_and(|str| str == "가능");
        let table = parser.read(SapTableBodyCommand::new(Self::MAIN_TABLE))?;
        let requirements = table
            .try_table_into::<GraduationRequirement>(&parser)?
            .into_iter()
            .map(|req| (req.name().to_owned(), req))
            .collect();
        Ok(GraduationRequirements::new(audit_result, requirements))
    }
    fn body(&self) -> &Body {
        self.client.body()
    }
}

/// 졸업사정표 애플리케이션에서 사용되는 데이터의 모듈
pub mod model;

#[cfg(test)]
mod test {
    use crate::webdynpro::command::WebDynproCommandExecutor;
    use crate::webdynpro::element::parser::ElementParser;
    use crate::{
        application::{
            USaintClientBuilder, graduation_requirements::GraduationRequirementsApplication,
        },
        global_test_utils::get_session,
        webdynpro::command::element::complex::SapTableBodyCommand,
    };

    #[tokio::test]
    async fn read_table() {
        let session = get_session().await.unwrap();
        let app = USaintClientBuilder::new()
            .session(session)
            .build_into::<GraduationRequirementsApplication>()
            .await
            .unwrap();
        let parser = ElementParser::new(app.body());
        let table = parser
            .read(SapTableBodyCommand::new(
                GraduationRequirementsApplication::MAIN_TABLE,
            ))
            .unwrap()
            .try_table_into::<Vec<(String, String)>>(&parser)
            .unwrap();
        dbg!(table);
    }
}
