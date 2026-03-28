---
name: uniffi-upgrade
description: "UniFFI 버전 업그레이드를 수행하는 스킬. 사용자가 uniffi 업그레이드, uniffi 버전 업데이트, uniffi 최신 버전 적용 등을 요청할 때 트리거. 반드시 rusaint 프로젝트 내에서만 실행."
---

# UniFFI Upgrade

UniFFI 및 관련 툴체인 버전 업그레이드를 수행한다.

## 프로젝트 구조

### 크레이트 구성

| 크레이트 | 역할 | uniffi 사용 방식 |
|----------|------|------------------|
| `packages/rusaint` | 코어 도메인 로직 | `#[cfg(feature = "uniffi")]`로 게이트, `setup_scaffolding!()` |
| `packages/rusaint-ffi` | FFI 래퍼 | 항상 활성화, `setup_scaffolding!()`, `async_runtime = "tokio"` |
| `uniffi-bindgen` | CLI 래퍼 | `uniffi = { features = ["cli"] }` |

### 사용 중인 proc-macro 패턴

버전 업그레이드 시 다음 패턴이 breaking changes의 영향을 받는지 확인:

- `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`, `#[derive(uniffi::Enum)]`
- `#[uniffi::export(async_runtime = "tokio")]`
- `#[uniffi::constructor]`
- `uniffi::custom_type!` (custom type 변환)
- `#[uniffi(flat_error)]` (에러 처리)
- `uniffi::setup_scaffolding!()` (두 크레이트에서 각각 호출)

### 주요 설정 파일

| 파일 | 역할 |
|------|------|
| `Cargo.toml` (root) | `workspace.dependencies.uniffi` 버전 핀 |
| `packages/rusaint/uniffi.toml` | 코어 크레이트 Kotlin/Swift 바인딩 설정 |
| `packages/rusaint-ffi/uniffi.toml` | FFI 크레이트 Kotlin/Swift/Python 바인딩 설정 |
| `packages/rusaint/Cargo.toml` | `uniffi` feature 게이트, `build-dependencies` |
| `packages/rusaint-ffi/Cargo.toml` | `uniffi = { features = ["tokio"] }`, `build-dependencies` |

### 바인딩 생성 실제 경로

**주의:** `generate_kotlin.sh`, `generate_swift.sh`는 존재하지 않음.

| 언어 | 생성 방식 | 파일 위치 |
|------|-----------|-----------|
| Kotlin | Gradle `generateBindings` 태스크 | `languages/kotlin/lib/build.gradle.kts` |
| Swift | `build.sh` 스크립트 내 | `languages/swift/build.sh` |
| React Native | yarn `ubrn:*` 스크립트 | `languages/react-native/ubrn.config.yaml` |

## 1. 버전 조사

다음 두 출처에서 최신 버전을 확인한다:

- **uniffi (crates.io)**: `WebFetch`로 `https://crates.io/api/v1/crates/uniffi` 조회 후 버전 확인. context7에서도 `/mozilla/uniffi-rs` 문서 확인.
- **uniffi-bindgen-react-native (npm)**: `WebFetch`로 `https://registry.npmjs.org/uniffi-bindgen-react-native` 조회. GitHub 저장소(jhugman/uniffi-bindgen-react-native)의 CHANGELOG.md에서 uniffi 버전별 대응 버전 확인.

### 호환성 판단

uniffi-bindgen-react-native는 uniffi 버전에 강하게 결합됨. npm에 퍼블리시된 버전 중 target uniffi 버전을 지원하는 최신 버전을 찾는다.

- 두 버전이 같은 메이저.마이너(예: 0.29.x)를 지원하면 해당 버전 사용
- uniffi-bindgen-react-native가 더 낮은 uniffi 버전만 지원하면, **그 낮은 버전으로 uniffi를 맞춤**
- 업그레이드가 불가능(현재보다 낮거나 동일)하면 사용자에게 알리고 종료

### 결과 보고

사용자에게 다음을 명확히 전달 후 승인을 받는다:
- 현재 uniffi 버전 → 목표 uniffi 버전
- 현재 uniffi-bindgen-react-native 버전 → 목표 버전
- 업데이트할 툴체인 목록(AGP, NDK, Swift 등)
- 마이그레이션 필요 변경사항 요약

## 2. 마이그레이션 가이드 확인

context7에서 `resolve-library-id`로 `uniffi-rs`를 찾고, `query-docs`로 "upgrading migration guide"를 조회.

추가로 다음을 확인:
- `WebFetch`로 https://raw.githubusercontent.com/mozilla/uniffi-rs/main/CHANGELOG.md 조회
- 현재 버전과 목표 버전 **사이의 모든 중간 버전**에 대해 breaking changes를 순차적으로 확인
  - 예: 0.29 → 0.31인 경우, 0.30과 0.31의 breaking changes를 모두 확인
- 프로젝트에서 사용하는 proc-macro 패턴(위 표 참조)에 영향이 있는지 확인

### CLI 인터페이스 변경 확인

바인딩 생성에 사용하는 CLI 플래그가 변경되었는지 확인:
- 현재 사용 중인 플래그: `--library`, `--language`, `--no-format`, `--out-dir`
- `--lib-file`은 v0.31에서 제거됨 (현재 프로젝트는 미사용)
- `--no-format` 플래그가 여전히 지원되는지 확인

## 3. 프로젝트 파일 수정

### UniFFI 버전 업데이트

| 파일 | 수정 내용 |
|------|-----------|
| `Cargo.toml` (root) | `[workspace.dependencies]`의 `uniffi` 버전 변경 |
| `Cargo.lock` | `cargo update`로 자동 갱신 |

**참고:** `build-dependencies`는 `workspace = true`를 사용하므로 root 버전 변경만으로 충분.

### uniffi.toml 검증

버전 업그레이드 후 다음 파일의 설정 옵션이 여전히 유효한지 확인:
- `packages/rusaint/uniffi.toml`
- `packages/rusaint-ffi/uniffi.toml`

`generate_immutable_records` 등 설정 옵션이 새 버전에서 변경되었는지 확인.

### uniffi-bindgen-react-native 버전 업데이트

| 파일 | 수정 내용 |
|------|-----------|
| `languages/react-native/package.json` | `dependencies.uniffi-bindgen-react-native` 버전 변경 |
| `languages/react-native/rusaint-react-native.podspec` | ubrn pod dependency 버전 변경 |

수정 후 **반드시 `yarn install` 실행** (누락 시 구버전 바인딩 생성됨).

### 바인딩 재생성

업데이트 후 반드시 바인딩을 재생성:

```sh
# Kotlin (Gradle 태스크로 실행)
cd languages/kotlin && ./gradlew :lib:generateBindings

# Swift (build.sh 내에서 생성)
cd languages/swift && ./build.sh

# React Native
cd languages/react-native && yarn ubrn:clean && yarn ubrn:ios && yarn ubrn:android
```

## 4. 툴체인 업데이트

최신 UniFFI가 요구하는 툴체인 버전을 확인 후 업데이트:

| 툴체인 | 파일 | 확인 방법 |
|--------|------|-----------|
| AGP | `languages/kotlin/gradle/libs.versions.toml` | UniFFI Kotlin 바인딩 호환 AGP 확인 |
| Kotlin | `languages/kotlin/gradle/libs.versions.toml` | AGP 버전에 맞는 Kotlin 버전 |
| NDK | `languages/kotlin/lib/build.gradle.kts` | UniFFI Android 빌드 요구사항 |
| compileSdk/targetSdk | `languages/kotlin/lib/build.gradle.kts` | AGP 버전에 맞는 SDK 버전 |
| Swift | `languages/swift/Rusaint-iOS/.../project.pbxproj` | UniFFI Swift 바인딩 호환 버전 |
| iOS deployment target | `languages/swift/build.sh` | UniFFI 요구 최소 버전 |

context7과 공식 문서에서 각 툴체인의 최신 호환 버전을 확인. 현재보다 높은 버전이 필요하면 업데이트.

## 5. 빌드 검증

각 타겟별로 빌드가 정상 동작하는지 확인:

```sh
# Rust 코어
cargo build --all-features
cargo clippy --all-features
cargo test --lib --all-features

# Kotlin/Android (Gradle generateBindings 태스크 포함)
cd languages/kotlin && ./gradlew :lib:build

# Swift/iOS (build.sh 내에서 바인딩 생성 + 빌드)
cd languages/swift && ./build.sh

# React Native
cd languages/react-native && yarn ubrn:release-build

# Python
cd languages/python && maturin develop
```

## 6. CI 워크플로우 확인

CI 워크플로우 파일에 하드코딩된 bindgen 명령어가 있는지 확인:
- `.github/workflows/ios-build.yml`
- `.github/workflows/ios-release.yml`

CLI 인터페이스가 변경된 경우 CI 파일도 함께 업데이트.

## 주의사항

- 절차를 모두 수행하기 전 사용자에게 작업 계획을 보여주고 승인을 받을 것
- 업그레이드가 불가능하면 원인을 명확히 설명할 것
- `cargo fmt --all`을 실행하여 포맷팅 유지
- 여러 버전을 건너뛰는 경우 모든 중간 버전의 breaking changes를 확인할 것
