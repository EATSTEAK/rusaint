# Agent Instructions

## Project Overview

rusaint는 숭실대학교 u-saint 시스템 클라이언트 라이브러리입니다. Rust로 작성되었으며, FFI(UniFFI)를 통해 Kotlin, Swift, React Native, Python 등 다양한 언어에서 사용할 수 있습니다.

## Workspace Structure

- `packages/rusaint` — 핵심 라이브러리 크레이트
- `packages/rusaint-ffi` — UniFFI 바인딩 크레이트
- `packages/rusaint-cli` — CLI 도구
- `session-helper` — 테스트용 세션 생성 도구
- `uniffi-bindgen` — UniFFI 바인딩 생성기
- `languages/` — 각 언어별 바인딩 프로젝트 (kotlin, swift, react-native, python)

## Development Workflow

### Formatting

코드 포맷팅은 반드시 workspace 전체에 대해 실행합니다:

```sh
cargo fmt --all
```

포맷 검사만 수행할 때:

```sh
cargo fmt --all -- --check
```

### Linting

Clippy를 사용하여 lint 검사를 수행합니다:

```sh
cargo clippy --all-features
```

### Testing

#### 세션 파일 생성

통합 테스트 실행 전, 세션 파일이 필요합니다. `.env` 파일에 SSU 로그인 정보를 설정한 후 (`.env.example` 참조), 다음 명령으로 세션 파일을 생성합니다:

```sh
cd packages/rusaint && cargo run --package rusaint-session-helper > session.json
```

#### 테스트 실행

통합 테스트 (세션 파일 필요):

```sh
cd packages/rusaint && cargo test --all-features
```

### Code Review Checklist

- `cargo fmt --all -- --check` 통과 여부
- `cargo clippy --all-features` 경고 없음
- `cargo test --lib --all-features` 통과 여부
- 새로운 public API에 대한 FFI 바인딩 추가 (`packages/rusaint-ffi`)
- UniFFI 호환 속성 (`#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]`) 적용 여부
