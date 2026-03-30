uniffi::setup_scaffolding!();

/// rusaint에서 제공하는 기본 u-saint 애플리케이션
pub mod application;

/// Android에서 webpki-roots 기반 TLS 설정
#[cfg(target_os = "android")]
mod android_tls {
    use once_cell::sync::OnceCell;
    use rustls::ClientConfig;
    use std::sync::Arc;

    static TLS_CONFIG: OnceCell<Arc<ClientConfig>> = OnceCell::new();

    /// webpki-roots 기반 ClientConfig를 생성합니다.
    fn create_tls_config() -> ClientConfig {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
    }

    /// webpki-roots 기반 TLS ClientConfig를 반환합니다.
    pub fn tls_config() -> Arc<ClientConfig> {
        TLS_CONFIG
            .get_or_init(|| Arc::new(create_tls_config()))
            .clone()
    }
}

/// Android TLS 설정을 노출
#[cfg(target_os = "android")]
pub use android_tls::tls_config as android_tls_config;

/// rusaint의 오류 처리 모듈
pub mod error;

/// u-saint 세션을 제공
pub mod session;

/// 플랫폼에 맞는 TLS 설정이 적용된 USaintClientBuilder를 생성합니다.
pub fn client_builder() -> rusaint::client::USaintClientBuilder {
    #[allow(unused_mut)]
    let mut builder = rusaint::client::USaintClientBuilder::new();
    #[cfg(target_os = "android")]
    {
        builder = builder.tls_client_config(android_tls_config());
    }
    builder
}
