uniffi::setup_scaffolding!();

/// rusaint에서 제공하는 기본 u-saint 애플리케이션
pub mod application;

#[cfg(target_os = "android")]
mod android {
    use jni::{
        JNIEnv,
        objects::{JClass, JObject},
        sys::jboolean,
    };

    fn throw_runtime_exception(env: &mut JNIEnv, message: impl AsRef<str>) {
        let _ = env.throw_new("java/lang/RuntimeException", message.as_ref());
    }

    fn init_platform_verifier(env: &mut JNIEnv, context: JObject, caller: &str) -> jboolean {
        match rustls_platform_verifier::android::init_with_env(env, context) {
            Ok(()) => 1,
            Err(error) => {
                throw_runtime_exception(
                    env,
                    format!("{caller} failed to initialize rustls platform verifier: {error}"),
                );
                0
            }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_eatsteak_rusaint_core_RusaintAndroid_nativeInitPlatformVerifier(
        mut env: JNIEnv,
        _class: JClass,
        context: JObject,
    ) -> jboolean {
        init_platform_verifier(&mut env, context, "RusaintAndroid")
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_eatsteak_rusaint_reactnative_RusaintReactNativeModule_nativeInitPlatformVerifier(
        mut env: JNIEnv,
        _this: JObject,
        context: JObject,
    ) -> jboolean {
        init_platform_verifier(&mut env, context, "RusaintReactNativeModule")
    }
}

/// rusaint의 오류 처리 모듈
pub mod error;

/// u-saint 세션을 제공
pub mod session;

/// 공통 USaintClientBuilder를 생성합니다.
pub fn client_builder() -> rusaint::client::USaintClientBuilder {
    rusaint::client::USaintClientBuilder::new()
}
