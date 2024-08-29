use std::borrow::Cow;

use super::Element;

/// 컴파일 타임에서도 생성할 수 있는 [`Element`] 정의
pub trait ElementDefinition<'body>: Sized {
    /// 해당 정의가 생성할 수 있는 엘리먼트
    type Element: Element<'body>;

    /// 런타임에서 엘리먼트 정의를 생성합니다. 엘리먼트의 Id 등을 컴파일 타임에서 미리 알 수 없는 경우 유용합니다.
    /// ### 예시
    /// ```
    /// # use rusaint::webdynpro::element::{ action::ButtonDef, definition::ElementDefinition };
    /// # fn get_dynamic_button() -> String { return "TEST.BUTTON1".to_string() }
    /// let runtime_string: String = get_dynamic_button();
    /// let button_def: ButtonDef = ButtonDef::new_dynamic(runtime_string);
    /// ```
    fn new_dynamic(id: String) -> Self;

    /// 엘리먼트의 Id를 반환합니다.
    fn id(&self) -> &str;

    /// [`Cow`]형태의 Id가 필요한 경우 사용합니다.
    fn id_cow(&self) -> Cow<'static, str>;
}
