---
name: uniffi-upgrade
description: "UniFFI 버전 업그레이드를 수행하는 스킬. 사용자가 uniffi 업그레이드, uniffi 버전 업데이트, uniffi 최신 버전 적용 등을 요청할 때 트리거. 반드시 rusaint 프로젝트 내에서만 실행."
---

# UniFFI Upgrade

UniFFI 및 관련 툴체인 버전 업그레이드를 수행한다.

## 1. 버전 조사

다음 두 출처에서 최신 버전을 확인한다:

- **uniffi (crates.io)**: `WebSearch`로 "uniffi crate latest version crates.io 2026" 검색 후 정확한 버전 확인. context7에서도 `/mozilla/uniffi-rs` 문서 확인.
- **uniffi-bindgen-react-native (npm)**: `WebSearch`로 "uniffi-bindgen-react-native npm latest version" 검색. GitHub 저장소(jhugman/uniffi-bindgen-react-native)의 CHANGELOG.md에서 uniffi 버전별 대응 버전 확인.

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
- https://mozilla.github.io/uniffi-rs/Upgrading.html 조회
- 현재 버전과 목표 버전 사이의 모든 breaking changes 파악

## 3. 프로젝트 파일 수정

### UniFFI 버전 업데이트

| 파일 | 수정 내용 |
|------|-----------|
| `Cargo.toml` (root) | `[workspace.dependencies]`의 `uniffi` 버전 변경 |
| `Cargo.lock` | `cargo update`로 자동 갱신 |

### uniffi-bindgen-react-native 버전 업데이트

| 파일 | 수정 내용 |
|------|-----------|
| `languages/react-native/package.json` | `dependencies.uniffi-bindgen-react-native` 버전 변경 |
| `languages/react-native/rusaint-react-native.podspec` | ubrn pod dependency 버전 변경 |

### 바인딩 재생성

업데이트 후 반드시 바인딩을 재생성:

```sh
# Kotlin
cargo run -p uniffi-bindgen generate --library ../packages/rusaint-ffi/src/lib.rs --language kotlin -o languages/kotlin/lib/src/main/kotlin

# Swift
cargo run -p uniffi-bindgen generate --library packages/rusaint-ffi/src/lib.rs --language swift -o languages/swift/generated

# React Native
cd languages/react-native && yarn ubrn:clean && yarn ubrn:ios && yarn ubrn:android
```

주의: 실제 명령어는 `generate_kotlin.sh`, `generate_swift.sh`, `ubrn.config.yaml`을 확인하여 정확한 경로와 옵션을 사용. 위 명령은 참고용.

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

# Kotlin/Android (generate_kotlin.sh 실행 후)
cd languages/kotlin && ./gradlew :lib:build

# Swift/iOS (generate_swift.sh 및 build.sh 실행 후)
cd languages/swift && ./build.sh

# React Native
cd languages/react-native && yarn ubrn:release-build

# Python
cd languages/python && maturin develop
```

CI 워크플로우 파일(`.github/workflows/`)의 관련 버전도 업데이트해야 하는지 확인.

## 주의사항

- 절차를 모두 수행하기 전 사용자에게 작업 계획을 보여주고 승인을 받을 것
- 업그레이드가 불가능하면 원인을 명확히 설명할 것
- `cargo fmt --all`을 실행하여 포맷팅 유지
