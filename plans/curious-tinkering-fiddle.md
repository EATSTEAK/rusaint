## Context
현재 Android에서는 `packages/rusaint-ffi/src/lib.rs`의 `android_tls` 모듈이 `webpki_roots::TLS_SERVER_ROOTS`로 `rustls::ClientConfig`를 직접 구성해 서버 인증서를 검증하고 있다. 이번 변경의 목적은 이 경로를 Android 플랫폼 trust store를 사용하는 `rustls-platform-verifier-android` 기반 검증으로 바꿔, `languages/kotlin`과 `languages/react-native` 양쪽 Android 런타임에서 동일하게 동작하게 만드는 것이다. 추가로, 사용자 피드백대로 `rustls-platform-verifier`를 활성화하면 Android에서 별도의 preconfigured TLS config 주입은 불필요하므로, 기존 `tls_client_config(...)` 주입 경로를 제거하는 방향으로 정리한다.

## Recommended approach
1. `packages/rusaint-ffi/Cargo.toml`의 Android 전용 의존성을 `webpki-roots` 중심 구성에서 `rustls-platform-verifier-android` 중심 구성으로 바꾼다.
   - 제거 후보: `webpki-roots`, Android 전용 수동 `rustls::ClientConfig` 구성에만 필요했던 의존성
   - 유지 후보: `rusaint`의 `rustls-no-provider` 여부는 실제 verifier 활성화 방식에 맞춰 재검토
   - 추가 후보: `rustls-platform-verifier` 계열 crate와 Android init bridge에 필요한 JNI 연동 의존성

2. `packages/rusaint-ffi/src/lib.rs`의 Android TLS 경로를 단순화한다.
   - 제거: `android_tls` 모듈, `OnceCell<Arc<ClientConfig>>`, `android_tls_config()` export
   - 제거: `client_builder()`에서 Android일 때 `builder.tls_client_config(...)`를 주입하는 분기
   - 유지: `client_builder()` 자체는 공통 builder 생성 entrypoint로 사용
   - 추가: Android `Context`를 받아 verifier를 1회 초기화하는 FFI entrypoint
   - 목표: Android에서는 reqwest/rustls가 platform verifier를 기본 verifier로 사용하도록 만들고, 별도 preconfigured TLS config을 만들지 않는다.

3. `languages/kotlin`에는 generated binding을 건드리지 말고, hand-written Android bootstrap API를 추가한다.
   - 수정 파일: `languages/kotlin/lib/build.gradle.kts`
   - 추가 파일 후보: `languages/kotlin/lib/src/main/kotlin/...` 아래 수동 Kotlin 파일 1개
   - 역할: `initialize(context: Context)` 같은 공개 API를 제공하고, 내부에서 `rusaint-ffi`의 Android verifier 초기화 함수를 1회 호출
   - 이유: 현재 Kotlin 경로에는 `Context`를 Rust로 넘길 런타임 초기화 훅이 없어서 build.gradle만으로는 해결되지 않음
   - build.gradle에서는 Android verifier 쪽 artifact/repository 및 필요 시 keep rule 경로를 반영한다.

4. `languages/react-native`는 기존 bootstrap 흐름을 재사용해 Android module에서 자동 초기화한다.
   - 수정 파일: `languages/react-native/android/src/main/java/dev/eatsteak/rusaint/reactnative/RusaintReactNativeModule.kt`
   - 재사용: `installRustCrate()`가 앱 시작 초기에 호출되는 기존 흐름
   - 변경: `installRustCrate()` 초입에서 `reactApplicationContext.applicationContext`를 이용해 verifier 초기화를 선행하고, 이후 기존 `nativeInstallRustCrate(...)`를 호출
   - 수정 파일: `languages/react-native/android/build.gradle`
   - build.gradle에서는 Kotlin 쪽과 동일하게 Android verifier artifact/repository 및 keep rule 반영을 맞춘다.

5. `packages/rusaint/src/client.rs`는 `preconfigured tls config` 관련 경로 정리에 한해 최소 수정 후보로 본다.
   - 현재 재사용 가능 지점: `USaintClientBuilder`와 `build()`의 공통 reqwest client 생성 흐름
   - 변경 가능 지점: `tls_client_config(...)` 필드/메서드와 `use_preconfigured_tls(...)` 분기가 Android verifier 활성화 후 불필요해지면 제거 또는 비활성화
   - 원칙: Kotlin/RN 변경을 위해 억지로 유지하지 말고, Android verifier가 기본 경로로 동작하도록 필요한 최소 범위만 정리한다.

## Critical files
- `packages/rusaint-ffi/Cargo.toml`
- `packages/rusaint-ffi/src/lib.rs`
- `languages/kotlin/lib/build.gradle.kts`
- `languages/kotlin/lib/consumer-rules.pro`
- `languages/kotlin/lib/src/main/kotlin/...` (새 bootstrap 파일)
- `languages/react-native/android/build.gradle`
- `languages/react-native/android/proguard-rules.pro`
- `languages/react-native/android/src/main/java/dev/eatsteak/rusaint/reactnative/RusaintReactNativeModule.kt`

## Existing code to reuse
- `packages/rusaint-ffi/src/lib.rs:46` `client_builder()` 공통 builder 생성 entrypoint
- `packages/rusaint/src/client.rs:189` `USaintClientBuilder::build(...)`의 공통 reqwest client 생성 흐름
- `languages/react-native/android/src/main/java/dev/eatsteak/rusaint/reactnative/RusaintReactNativeModule.kt:22` `installRustCrate()` 초기 bootstrap 지점

## Risks and checks
- Kotlin generated binding 파일은 재생성 대상이므로 수정하지 않는다.
- Kotlin은 JNA 기반, RN은 JNI/JSI 기반이어서 Android verifier 초기화 entrypoint를 두 경로에서 각각 안정적으로 호출해야 한다.
- `rustls-platform-verifier` Rust crate와 Android artifact 버전이 어긋나지 않게 맞춘다.
- R8/Proguard에서 verifier/JNA 관련 클래스가 제거되지 않게 keep rule을 추가한다.
- 기존 `preconfigured tls config` 제거 후에도 Android에서 reqwest/rustls가 의도대로 platform verifier를 기본 verifier로 사용하는지 실제 연결 테스트로 확인한다.

## Verification
1. Rust 레벨 검증
   - Android 타깃 빌드가 성공하는지 확인
   - `client_builder()`에서 Android 전용 preconfigured TLS config 주입이 제거되었는지 확인
   - verifier 초기화가 중복 호출되어도 안전한지 확인
   - Android에서 기본 reqwest/rustls 경로가 platform verifier를 사용해 접속에 성공하는지 확인

2. Kotlin Android 검증
   - 앱 또는 샘플에서 새 `initialize(context)` API를 먼저 호출한 뒤 기존 `USaintSessionBuilder` / application builder 흐름이 정상 동작하는지 확인
   - 초기화를 생략했을 때 의도한 오류가 나는지 확인
   - release/R8 빌드에서도 verifier 관련 클래스가 제거되지 않는지 확인

3. React Native Android 검증
   - 기존 JS import 시점의 `installRustCrate()` 흐름에서 verifier 초기화가 먼저 수행되는지 확인
   - 이후 기존 RN API 호출이 정상 동작하는지 확인
   - reload/HMR이나 재호출 시 중복 초기화가 no-op인지 확인

4. End-to-end 검증
   - 실제 Android 기기/에뮬레이터에서 Kotlin, React Native 각각 SSU 서버 접속이 성공하는지 확인
   - Android 시스템 trust store 기반 검증이 사용되는지 로그 또는 동작 차이로 확인