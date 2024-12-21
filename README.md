<h1 align="center">rusaint</h1>
<p align="center" style="font-style: italic;">빠르고 간편하며 믿을 수 있는 숭실대학교 u-saint 클라이언트</p>
<p align="center">
    <a href="https://github.com/EATSTEAK/rusaint"><img alt="GitHub Badge" src="https://img.shields.io/badge/github-eatsteak/rusaint-8da0cb?style=for-the-badge&labelColor=555555&logo=github"></a>
    <a href="https://crates.io/crates/rusaint"><img alt="crates.io" src="https://img.shields.io/crates/v/rusaint.svg?style=for-the-badge&color=fc8d62&logo=rust"></a>
    <a href="https://docs.rs/rusaint"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rusaint-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs"></a>
   <a href="https://github.com/EATSTEAK/rusaint/LICENSE.md"><img alt="License" src="https://img.shields.io/github/license/EATSTEAK/rusaint?style=for-the-badge"></a>
</p>

---

rusaint(_ru-saint, 루세인트_)는 [숭실대학교 u-saint](https://saint.ssu.ac.kr)를 정확하고 빠르게, 간편하게 파싱하고 다양한 환경에서 조작할 수 있는 Rust 기반 비공식
u-saint 클라이언트입니다.

u-saint의 기반인 [SAP Web Dynpro](https://en.wikipedia.org/wiki/Web_Dynpro)에서 사용하는 Lightspeed 라이브러리의 최소 동작을 구현하여 안전하게
u-saint 내부 요소들을 조작하고 파싱할 수 있습니다.

- **JS 런타임 없음** — JS 런타임 없이 자체적으로 요청과 응답에 따른 처리를 수행하므로 HTTPS 요청이 가능한 모든 환경에서 실행 가능합니다.
- **빠른 속도** — 네이티브 환경으로 컴파일되는 Rust를 이용하고, 휴리스틱 없이 요청이 완료되면 곧바로 실행되어 빠르게 u-saint 를 조작 및 파싱 가능합니다.
- **멀티플랫폼 지원** — UniFFI를 통한 Kotlin, Swift, Python(예정) 지원 및 Node.js 용 WASM Wrapper(예정)를 제공하여 다양한 플랫폼에서 간편하게 이용할 수 있습니다.
- **간편한 기능 정의** — rusaint 에서 지원하지 않는 u-saint 애플리케이션에 대한 파싱 및 지원을 제공하는 API를 이용해 간편하게 정의할 수 있습니다.

## 설치

```bash
cargo add rusaint
```

## 문서

[docs.rs](https://docs.rs/rusaint)

## 예시

```rust
use rusaint::application::course_grades::{CourseGrades, model::SemesterSummary};
use rusaint::session::USaintSession;
use futures::executor::block_on;

// 성적 정보를 출력하는 애플리케이션
fn main() {
    block_on(print_grades());
    /* SemesterSummary { year: 2022, semester: "2 학기", attempted_credits: 17.5, earned_credits: 17.5, pf_earned_credits: 0.5, grade_points_avarage: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 99), general_rank: (1, 99), academic_probation: false, consult: false, flunked: false }
     * ...
     */
}

async fn print_grades() -> Result<(), RusaintError> {
    // USaintSession::from_token(id: &str, token: &str) 을 이용하여 비밀번호 없이 SSO 토큰으로 로그인 할 수 있음
    let session = USaintSession::from_password("20211561", "password").await?;
    let mut app = USaintClientBuilder::new().session(session).build_into::<CourseGrades>().await?;
    let grades: Vec<SemesterGrade> = app.semesters(CourseType::Bachelor).await?;
    for grade in grades {
        println!("{:?}", grade);
    }
    Ok(())
}
```

## 멀티 플랫폼

rusaint는 [uniffi](https://github.com/mozilla/uniffi-rs)를 이용한 FFI 멀티플랫폼을 지원합니다. 현재 지원하는 플랫폼은 다음과 같습니다.

- Android (Kotlin)
- Swift

### Android (Kotlin)

#### Maven Central에서 설치하기

1. Maven Central을 gradle 저장소에 추가

```kotlin
repositories {
  mavenCentral()
  // ... any other repositories
}
```

2. rusaint를 의존성에 추가

```kotlin
dependencies {
  implementation("dev.eatsteak:rusaint:0.8.2")
}
```

#### 소스 코드에서 빌드

[languages/kotlin](/languages/kotlin) 내부의 README.md 파일을 참고하세요.

### Swift

#### SPM을 이용하여 설치

[SPM 레포지토리](https://github.com/EATSTEAK/rusaint-ios)로 설치할 수 있습니다.

#### 소스 코드에서 빌드

[languages/swift](/languages/swift) 내부의 build.sh 파일을 실행하여 .xcframework 형태로 빌드할 수 있습니다.
  
