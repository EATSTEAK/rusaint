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
- **멀티플랫폼 지원** — UniFFI를 통한 Kotlin, Swift, React-Native, Python 지원을 제공하여 다양한 플랫폼에서 간편하게 이용할 수 있습니다.
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
    /* SemesterSummary { year: 2022, semester: "2 학기", attempted_credits: 17.5, earned_credits: 17.5, pf_earned_credits: 0.5, grade_points_average: 4.5, grade_points_sum: 100.0, arithmetic_mean: 100.0, semester_rank: (1, 99), general_rank: (1, 99), academic_probation: false, consult: false, flunked: false }
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

## 개발

개발 및 테스트를 위해 아래와 같은 요구사항이 필요합니다:

- `SSO_ID`, `SSO_PASSWORD` 환경 변수 (숭실대학교 SSO ID/비밀번호)
- `TARGET_YEAR`, `TARGET_SEMESTER` 환경 변수 (학년도/학기, 예시: `2022`, `1`)
- 테스트를 실행하기 전에, `cargo run -p rusaint-session-helper >> session.json`을 실행하여 세션을 파일로 생성하여야 합니다 (사용하는 쉘에 따라 정확한 명령어는 달라질 수 있습니다).
  - 세션 파일 경로를 변경하고 싶다면, `SSO_SESSION_FILE` 환경 변수에 경로를 입력하여 변경할 수 있습니다.
- `rusaint`는 빠른 테스트 실행을 위해 `cargo-nextest`를 사용합니다. (`cargo install cargo-nextest`를 통해 설치할 수 있습니다)
- `cargo nextest run` 을 실행하여 테스트를 실행합니다.

## 멀티 플랫폼

rusaint는 [uniffi](https://github.com/mozilla/uniffi-rs)를 이용한 FFI 멀티플랫폼을 지원합니다. 현재 지원하는 플랫폼은 다음과 같습니다.

- Android (Kotlin)
- iOS (Swift)
- React Native
- Python

### Android (Kotlin)

<a href="https://central.sonatype.com/artifact/dev.eatsteak/rusaint"><img alt="Maven Central Version" src="https://img.shields.io/maven-central/v/dev.eatsteak/rusaint?style=for-the-badge&logo=apachemaven&color=C71A36">
</a>

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
  implementation("dev.eatsteak:rusaint:0.10.0")
}
```

#### 소스 코드에서 빌드

[languages/kotlin](/languages/kotlin) 내부의 README.md 파일을 참고하세요.

### iOS (Swift)

<a href="https://github.com/EATSTEAK/rusaint-ios"><img alt="SwiftPM Release" src="https://img.shields.io/github/v/release/eatsteak/rusaint-ios?style=for-the-badge&logo=swift&label=SwiftPM&color=F05138">
</a>
<img alt="Swift Compatibility" src="https://img.shields.io/badge/Compatibility-5.x-F05138?style=for-the-badge&logo=swift">
<img alt="Swift Targets" src="https://img.shields.io/badge/Target-iOS-F05138?style=for-the-badge&logo=swift">

#### SPM을 이용하여 설치

[SPM 레포지토리](https://github.com/EATSTEAK/rusaint-ios)로 설치할 수 있습니다.

#### 소스 코드에서 빌드

[languages/swift](/languages/swift) 내부의 build.sh 파일을 실행하여 .xcframework 형태로 빌드할 수 있습니다.

### React Native (Turbo Module)

<a href="https://www.npmjs.com/package/@rusaint/react-native"><img alt="NPM Distribution" src="https://img.shields.io/npm/v/%40rusaint%2Freact-native?style=for-the-badge&logo=npm&color=CB3837"></a>

#### 패키지 매니저를 이용하여 설치

```bash
pnpm add @rusaint/react-native # or yarn, npm
```

##### Expo에서 사용하기

현재 `@rusaint/react-native`는 Expo Module을 지원하지 않습니다. 따라서 Expo 프로젝트에서 사용하기 위해서는 `@react-native-community/cli`의 autolink 기능을 활성화하여 Turbo Module을 autolink 해야 합니다.

```bash
pnpm add @react-native-community/cli -D # or use yarn, npm
```

으로 `@react-native-community/cli`를 설치합니다.

```properties
# .env
EXPO_USE_COMMUNITY_AUTOLINKING=1 # Enable autolinking by @react-native-community/cli
```

`EXPO_USE_COMMUNITY_AUTOLINKING` 환경변수를 `expo prebuild` 과정에 제공하여 모듈의 autolink를 활성화합니다.

> [!WARNING]
> Community autolinking을 활성화 하면 Expo Go를 사용할 수 없습니다.

#### 소스 코드에서 빌드

```bash
# ./languages/react-native
yarn install
yarn ubrn:release-build # REQUIREMENTS: Cargo and android/ios targets for building rust binaries
yarn prepare # Run codegen
```

### Python

<a href="https://pypi.org/project/rusaint/"><img alt="PyPI - Version" src="https://img.shields.io/pypi/v/rusaint?style=for-the-badge&logo=pypi&color=3775A9"></a>

#### PyPI에서 설치

```bash
# Using pip
pip install rusaint
# Using uv
uv add rusaint
```

#### 소스 코드에서 빌드

rusaint는 [maturin](https://www.maturin.rs/)을 사용하여 Python wheel을 빌드합니다.

```bash
# ./languages/python
# with uv
uv tool install maturin
uvx maturin build --release --out dist --find-interpreter # Creates an release wheel in ./dist directory
```
