<h1 align="center">rusaint</h1>
<p align="center" style="font-style: italic;">빠르고 간편하며 믿을 수 있는 숭실대학교 u-saint 클라이언트</p>
<p align="center">
    <a href="https://github.com/EATSTEAK/rusaint"><img alt="GitHub Badge" src="https://img.shields.io/badge/github-eatsteak/rusaint-8da0cb?style=for-the-badge&labelColor=555555&logo=github"></a>
    <a href="https://github.com/EATSTEAK/rusaint/releases"><img alt="Cargo version" src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FEATSTEAK%2Frusaint%2Fmain%2FCargo.toml&query=%24.package.version&prefix=v&style=for-the-badge&logo=rust&label=version"></a>
   <a href="https://github.com/EATSTEAK/rusaint/LICENSE.md"><img alt="License" src="https://img.shields.io/github/license/EATSTEAK/rusaint?style=for-the-badge"></a>
</p>

---

rusaint는 [숭실대학교 u-saint](https://saint.ssu.ac.kr)를 정확하고 빠르게, 간편하게 파싱하고 다양한 환경에서 조작할 수 있는 Rust 기반 비공식 u-saint 클라이언트입니다.

u-saint의 기반인 [SAP Web Dynpro](https://en.wikipedia.org/wiki/Web_Dynpro)에서 사용하는 Lightspeed 라이브러리의 최소 동작을 구현하여 안전하게 u-saint 내부 요소들을 조작하고 파싱할 수 있습니다.

- **JS 런타임 없음** — JS 런타임 없이 자체적으로 요청과 응답에 따른 처리를 수행하므로 HTTPS 요청이 가능한 모든 환경에서 실행 가능합니다.
- **빠른 속도** — 네이티브 환경으로 컴파일되는 Rust를 이용하고, 휴리스틱 없이 요청이 완료되면 곧바로 실행되어 빠르게 u-saint 를 조작 및 파싱 가능합니다.
- **멀티플랫폼 지원(예정)** — UniFFI를 통한 Kotlin, Swift, Python 지원 및 Node.js 용 WASM Wrapper 를 제공하여 다양한 플랫폼에서 간편하게 이용할 수 있습니다.
- **간편한 기능 정의(예정)** — rusaint 에서 지원하지 않는 u-saint 애플리케이션에 대한 파싱 및 지원을 제공하는 API를 이용해 간편하게 정의할 수 있습니다.

## 설치

rusaint는 현재 **미완성 버전**으로 공개된 저장소에서 설치할 수 없습니다. 직접 저장소를 복제하여 설치하세요.

## 문서

_추가 예정_

## 예시

```rust
use anyhow::Result;
use rusaint::application::course_grades::{CourseGrades, data::GradeSummary};
use rusaint::session::USaintSession;
use futures::executor::block_on;

// 성적 정보를 출력하는 애플리케이션
fn main() {
    block_on(print_grades());
    /* GradeSummary { year: 2023, semester: "여름학기", attempt_credits: 2.0, earn_credits: 2.0, pf_credits: 2.0, grade_points_avarage: 0.0, grade_points_sum: 0.0, arithmetic_mean: 0.0, semester_rank: (0, 0), general_rank: (0, 0), academic_probation: false, consult: false, flunked: false }
     * GradeSummary { year: 2022, semester: "2 학기", attempt_credits: 17.5, earn_credits: 17.5, pf_credits: 0.5, grade_points_avarage: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 99), general_rank: (1, 99), academic_probation: false, consult: false, flunked: false }
     * GradeSummary { year: 2022, semester: "1 학기", attempt_credits: 19.5, earn_credits: 19.5, pf_credits: 0.5, grade_points_avarage: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 100), general_rank: (1, 100), academic_probation: false, consult: false, flunked: false }
     * GradeSummary { year: 2021, semester: "2 학기", attempt_credits: 19.5, earn_credits: 19.5, pf_credits: 0.5, grade_points_avarage: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 99), general_rank: (1, 99), academic_probation: false, consult: false, flunked: false }
     * GradeSummary { year: 2021, semester: "1 학기", attempt_credits: 20.5, earn_credits: 20.5, pf_credits: 2.5, grade_points_avarage: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 103), general_rank: (1, 103), academic_probation: false, consult: false, flunked: false }
     */
}

async fn print_grades() -> Result<()> {
    // USaintSession::from_token(id: &str, token: &str) 을 이용하여 비밀번호 없이 SSO 토큰으로 로그인 할 수 있음
    let session = USaintSession::from_password("20211561", "password").await?;
    let app = CourseGrades::new(session).await?;
    let grades: Vec<GradeSummary> = app.grade_summary().await?;
    for grade in grades {
        println!("{:?}", grade);
    }
}
```
