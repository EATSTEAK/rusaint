#![warn(missing_docs)]
// TODO: Reduce error size and remove this attr
#![allow(clippy::result_large_err)]
//! _빠르고 간편하며 믿을 수 있는 숭실대학교 u-saint 클라이언트_
//!
//! <a href="https://github.com/EATSTEAK/rusaint"><img alt="GitHub Badge" src="https://img.shields.io/badge/github-eatsteak/rusaint-8da0cb?style=for-the-badge&labelColor=555555&logo=github"></a>
//! <a href="https://crates.io/crates/rusaint"><img alt="crates.io" src="https://img.shields.io/crates/v/rusaint.svg?style=for-the-badge&color=fc8d62&logo=rust"></a>
//! <a href="https://docs.rs/rusaint"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rusaint-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs"></a>
//! <a href="https://github.com/EATSTEAK/rusaint/LICENSE.md"><img alt="License" src="https://img.shields.io/github/license/EATSTEAK/rusaint?style=for-the-badge"></a>
//!
//! ---
//!
//! rusaint는 [숭실대학교 u-saint](https://saint.ssu.ac.kr)를 정확하고 빠르게, 간편하게 파싱하고 다양한 환경에서 조작할 수 있는 Rust 기반 비공식 u-saint 클라이언트입니다.
//!
//! u-saint의 기반인 [SAP Web Dynpro](https://en.wikipedia.org/wiki/Web_Dynpro)에서 사용하는 Lightspeed 라이브러리의 최소 동작을 구현하여 안전하게 u-saint 내부 요소들을 조작하고 파싱할 수 있습니다.
//!
//! - **JS 런타임 없음** — JS 런타임 없이 자체적으로 요청과 응답에 따른 처리를 수행하므로 HTTPS 요청이 가능한 모든 환경에서 실행 가능합니다.
//! - **빠른 속도** — 네이티브 환경으로 컴파일되는 Rust를 이용하고, 휴리스틱 없이 요청이 완료되면 곧바로 실행되어 빠르게 u-saint 를 조작 및 파싱 가능합니다.
//! - **멀티플랫폼 지원** — UniFFI를 통한 Kotlin, Swift, Python(예정) 지원 및 Node.js 용 WASM Wrapper(예정)를 제공하여 다양한 플랫폼에서 간편하게 이용할 수 있습니다.
//! - **간편한 기능 정의** — rusaint 에서 지원하지 않는 u-saint 애플리케이션에 대한 파싱 및 지원을 제공하는 API를 이용해 간편하게 정의할 수 있습니다.
//! ## 예시
//!
//! ```no_run
//! use rusaint::application::course_grades::{CourseGradesApplication, model::CourseType, model::SemesterGrade};
//! use rusaint::application::USaintClientBuilder;
//! use rusaint::webdynpro::element::Element;
//! use rusaint::RusaintError;
//! use std::sync::Arc;
//! use rusaint::USaintSession;
//! use futures::executor::block_on;
//!
//! // 성적 정보를 출력하는 애플리케이션
//! fn main() {
//!     block_on(print_grades());
//!     /* SemesterGrade { year: 2022, semester: "2 학기", attempted_credits: 17.5, earned_credits: 17.5, pf_earned_credits: 0.5, grade_points_average: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 99), general_rank: (1, 99), academic_probation: false, consult: false, flunked: false }
//!      */
//! }
//!
//! async fn print_grades() -> Result<(), RusaintError> {
//!     // USaintSession::with_token(id: &str, token: &str) 을 이용하여 비밀번호 없이 SSO 토큰으로 로그인 할 수 있음
//!     let session = Arc::new(USaintSession::with_password("20211561", "password").await?);
//!     let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGradesApplication>().await?;
//!     let grades: Vec<SemesterGrade> = app.semesters(CourseType::Bachelor).await?;
//!     for grade in grades {
//!         println!("{:?}", grade);
//!     }
//!     Ok(())
//! }
//! ```
#[cfg(feature = "application")]
/// rusaint에서 제공하는 기본 u-saint 애플리케이션
pub mod application;
#[cfg(feature = "application")]
mod error;
#[cfg(feature = "application")]
pub use error::ApplicationError;
#[cfg(feature = "application")]
pub use error::RusaintError;
#[cfg(feature = "application")]
pub use error::SsuSsoError;
#[cfg(feature = "application")]
mod session;

#[cfg(feature = "application")]
pub use session::obtain_ssu_sso_token;

#[cfg(feature = "application")]
pub use session::USaintSession;

#[cfg(feature = "application")]
/// u-saint 애플리케이션에서 공통으로 사용하는 데이터
pub mod model;

pub(crate) mod utils;
/// SAP WebDynpro 클라이언트를 파싱, 모방하는 클라이언트 엔진
pub mod webdynpro;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "uniffi")]
/// `uniffi` 지원을 위한 모듈
pub mod uniffi_support;

#[cfg(test)]
#[allow(missing_docs)]
pub mod global_test_utils {
    use crate::{USaintSession, model::SemesterType};
    use anyhow::{Error, Result};
    use dotenvy::dotenv;
    use lazy_static::lazy_static;
    use std::{fs::File, io::BufReader, sync::Arc};

    lazy_static! {
        pub(crate) static ref TARGET_YEAR: u32 = {
            dotenv().ok();
            std::env::var("TARGET_YEAR").unwrap().parse().unwrap()
        };
        pub(crate) static ref TARGET_SEMESTER: SemesterType = {
            dotenv().ok();
            let semester = std::env::var("TARGET_SEMESTER").unwrap();
            match semester.to_uppercase().as_str() {
                "1" | "ONE" => SemesterType::One,
                "SUMMER" => SemesterType::Summer,
                "2" | "TWO" => SemesterType::Two,
                "WINTER" => SemesterType::Winter,
                _ => panic!("{:?}", Error::msg("Invalid semester")),
            }
        };
    }

    pub async fn get_session() -> Result<Arc<USaintSession>> {
        let session_file_path =
            std::env::var("SSO_SESSION_FILE").unwrap_or("session.json".to_string());
        let f = File::open(&session_file_path)
            .map_err(|e| Error::msg(format!("Failed to open session file: {}", e)))?;
        let reader = BufReader::new(f);
        let session: USaintSession = USaintSession::from_json(reader)
            .map_err(|e| Error::msg(format!("Failed to parse session file: {}", e)))?;
        let session = Arc::new(session);
        Ok(session)
    }
}
