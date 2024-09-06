use crate::webdynpro::error::WebDynproError;
use definition::SubElementDefinition;
use scraper::ElementRef;

/// [`SubElement`](SubElement)의 정의에 관련된 모듈
pub mod definition;

/// SubElement 생성에 관련된 매크로 모듈
pub mod macros;

/// 서브 엘리먼트의 기능
pub trait SubElement<'a>: Sized {
    /// WebDynpro 내부에서 사용하는 서브 엘리먼트의 Id
    const SUBCONTROL_ID: &'static str;
    /// WebDynpro 내부에서 사용하는 서브 엘리먼트의 이름
    const ELEMENT_NAME: &'static str;
    /// 서브 엘리먼트의 LSData
    type SubElementLSData;
    /// 서브 엘리먼트의 정의
    type Def: SubElementDefinition<'a>;

    /// 서브 엘리먼트 정의와[] `scraper::ElementRef`]로부터 서브 엘리먼트를 가져옵니다.
    fn from_ref(
        elem_def: &impl SubElementDefinition<'a>,
        element: ElementRef<'a>,
    ) -> Result<Self, WebDynproError>;

    /// 서브 엘리먼트의 LSData를 가져옵니다.
    fn lsdata(&self) -> &Self::SubElementLSData;

    /// 서브 엘리먼트의 Id를 가져옵니다.
    fn id(&self) -> &str;

    /// 서브 엘리먼트의 [`ElementRef`]를 가져옵니다.
    fn element_ref(&self) -> &ElementRef<'a>;
}
