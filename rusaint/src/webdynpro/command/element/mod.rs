/// 액션 분류의 엘리먼트([`Button`](crate::webdynpro::element::action::Button), [`Link`](crate::webdynpro::element::action::Link))를 위한 명령
pub mod action;

/// complex 분류의 엘리먼트([`SapTable`](crate::webdynpro::element::complex::SapTable))를 위한 명령
pub mod complex;

/// 선택 분류의 엘리먼트([`ListBox`](crate::webdynpro::element::selection::list_box::ListBox), [`ComboBox`](crate::webdynpro::element::selection::ComboBox))를 위한 명령
pub mod selection;

/// 레이아웃 분류의 엘리먼트를 위한 명령
pub mod layout;

/// 텍스트 분류의 엘리먼트([`Caption`](crate::webdynpro::element::text::Caption), [`InputField`](crate::webdynpro::element::text::InputField), [`Label`](crate::webdynpro::element::text::Label), [`TextView`](crate::webdynpro::element::text::TextView)를 위한 명령)
pub mod text;

/// 시스템 분류의 엘리먼트([`ClientInspector`](crate::webdynpro::element::system::ClientInspector), [`Custom`](system::Custom), [`LoadingPlaceholder`](crate::webdynpro::element::system::LoadingPlaceholder))를 위한 명령
pub mod system;
