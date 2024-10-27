//! # WebDynpro 모듈
//! WebDynpro 모듈은 rusaint의 u-saint 페이지 스크래핑과 입력 조작을 위한 기반 엔진입니다.
//! 이 모듈의 기능을 이용하여 아직 rusaint에 구현되지 않은 애플리케이션을 손쉽게 구현할 수 있습니다.
//! 또한 숭실대학교 u-saint 이외에도 WebDynpro 의 Lightspeed 라이브러리를 통해 구현되는 페이지에 대한 분석을 수행할 수 있습니다.
//!
//! ## WebDynpro 페이지 분석하기
//! 직접 WebDynpro 페이지를 분석하여 이에 대한 자동화된 애플리케이션을 구현하려면, 애플리케이션에서 조작하고자 하는 주요 요소들에 대해 정의하는 작업이 선행되어야 합니다.
//!
//! 대부분의 조작하고자 하는 요소는 WebDynpro 엘리먼트로 구성되어 있으므로, 조작/분석하고자 하는 엘리먼트의 종류, ID를 미리 컴파일 타임에 정의하여 파싱/조작 작업을 원활히 수행할 수 있습니다.
//!
//! ### 페이지 내 엘리먼트가 정적인 ID를 가지도록 하기
//! 기본적으로 WebDynpro 페이지의 엘리먼트 ID는 동적으로 구성되도록 되어 있습니다. 대략적으로 엘리먼트가 렌더링되는 순서대로 ID가 부여되며, 따라서 동일한 위치에 있는 동일한 엘리먼트임에도 불구하고 수행한 동작의 순서나 종류에 따라 다른 ID를 가질 수 있습니다.
//!
//! 이러한 현상을 방지하려면 WebDynpro 에서 자동 테스팅 목적으로 제공하는 파라메터 `SAP-WD-STABLEIDS`를 `X` 로 표시하면 정적인 ID를 가진 페이지를 확인할 수 있습니다.
//! [예시](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100?sap-wd-stableids=x)
//!
//! ### 엘리먼트의 종류 알아내기
//! 엘리먼트의 종류는 해당 엘리먼트 태그의 `ct` HTML 어트리뷰트 값을 rusaint WebDynpro 엘리먼트의 [`CONTROL_ID`]와 매칭시켜 알아낼 수 있습니다.
//! > **예시:** `ct="B"` 어트리뷰트의 경우 WebDynpro 엘리먼트의 [`Button`]과 매칭됩니다.
//!
//! 엘리먼트의 종류나 렌더링 방법에 따라 엘리먼트 자체를 감싸는 태그가 있을 수 있습니다. 이런 태그는 보통 해당 엘리먼트의 ID 뒤에 `-r` 접미사가 붙는것으로 확인할 수 있습니다.
//!
//! ## 애플리케이션 정의 예시
//! [`USaintClient`](crate::application::USaintClient)를 활용하여 [예시](https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMW2100?sap-wd-stableids=x&sap-language=ko) 애플리케이션 상단의 "담당자문의 정보" 텍스트를 파싱하는 예시 애플리케이션입니다.
//!
//! [`WebDynproClient`](client::WebDynproClient)을 활용하여 u-saint 외의 다른 사이트에 대한 정의를 만들 수 있으나, [`LoadingPlaceholder`] 로딩이나 [`ClientInspector`] 처리 등 추가적인 처리가 필요할 수 있습니다.
//! 추가 정보는 [`LoadingPlaceholder`], [`ClientInspector`] 와 [`Custom`] 엘리먼트를 참고하십시오.
//! ```ignore
//! use futures::executor::block_on;
//! use rusaint::application::{USaintClient, USaintClientBuilder, USaintApplication};
//! use rusaint::define_elements;
//! use rusaint::webdynpro::element::text::Caption;
//! use rusaint::RusaintError;
//!
//! // USaintClient을 Wrap한 애플리케이션 struct를 새로 정의합니다.
//! // WebDynproClient 등의 기본 애플리케이션 정의를 사용하면 u-saint 가 아닌 다른 사이트에 적용할 수 있으나, 기본 로딩과 같은 추가 작업이 필요할 수 있습니다.
//! pub struct ExampleApplication {
//!   client: USaintClient
//! };
//!
//! impl USaintApplication for ExampleApplication {
//!   const APP_NAME: &'static str = "ZCMW2100";
//!
//!   fn from_client(client: USaintClient) -> Result<Self, RusaintError> {
//!     if client.name() != Self::APP_NAME {
//!       Err(RusaintError::InvalidClientError)
//!     } else {
//!       Ok(Self { client })
//!     }
//!   }
//! }
//!
//! impl<'a> ExampleApplication {
//!     // 엘리먼트를 정의하는 매크로
//!     define_elements! {
//!         // 담당자문의 정보에 해당하는 캡션의 ID 정의
//!         CAPTION: Caption<'a> = "ZCMW_DEVINFO_RE.ID_D080C16F227F4D68751326DC40BB6BE0:MAIN.CAPTION";
//!     }
//!
//!
//!     // 캡션의 데이터를 읽는 함수
//!     pub fn read_caption(&self) -> Result<String, RusaintError> {
//!         // 엘리먼트 파서를 생성
//!         let parser = ElementParser::new(self.client.body());
//!         // 캡션 정의와 현재 애플리케이션 바디로부터 엘리먼트 객체 생성
//!         let caption = parser.element_from_def(&Self::CAPTION)?;
//!         // 캡션 엘리먼트로부터 텍스트를 반환
//!         Ok(caption.text().to_string())
//!     }
//! }
//!
//! async fn test_caption() {
//!     let app = USaintClientBuilder::new().build_into<ExampleApplication>().await.unwrap();
//!     let text = app.read_caption().unwrap();
//!     assert_eq!(text, "담당자문의 정보");
//! }
//!
//! fn main() {
//!     block_on(test_caption());
//! }
//! ```
//! [`CONTROL_ID`]: webdynpro::element::Element::CONTROL_ID
//! [`Button`]: webdynpro::element::action::Button
//! [`USaintApplication`]: application::USaintApplication
//! [`BasicApplication`]: webdynpro::application::BasicApplication
//! [`LoadingPlaceholder`]: webdynpro::element::system::LoadingPlaceholder
//! [`ClientInspector`]: webdynpro::element::system::ClientInspector
//! [`Custom`]: webdynpro::element::system::Custom

/// WebDynpro 페이지의 기본 클라이언트
pub mod client;
#[cfg(feature = "element")]
/// WebDynpro 페이지를 구성하는 엘리먼트
pub mod element;

#[cfg(feature = "element")]
/// WebDynpro 클라이언트를 조작하는 명령
pub mod command;

/// WebDynpro 클라이언트 작업 중 발생할 수 있는 오류
pub mod error;
/// WebDynpro 엘리먼트가 발생시키는 이벤트 처리
pub mod event;
