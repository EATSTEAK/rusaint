use scraper::Selector;
use wdpe::body::Body;
use wdpe::element::layout::PopupWindow;
use wdpe::element::parser::ElementParser;
use wdpe::element::{Element, ElementWrapper};
use wdpe::error::WebDynproError;
use wdpe::event::Event;

use crate::client::USaintClient;

/// 현재 페이지에 열려있는 모든 팝업 창을 닫습니다.
pub(crate) async fn close_popups(client: &mut USaintClient) -> Result<(), WebDynproError> {
    let popup_selector =
        Selector::parse(format!(r#"[ct="{}"]"#, PopupWindow::CONTROL_ID).as_str()).unwrap();
    fn make_close_event(body: &Body, selector: &Selector) -> Option<Event> {
        let parser = ElementParser::new(body);
        let mut popup_iter = parser.document().select(selector);
        popup_iter.next().and_then(|elem| {
            let elem_wrapped = ElementWrapper::from_ref(elem).ok()?;
            if let ElementWrapper::PopupWindow(popup) = elem_wrapped {
                popup.close().ok()
            } else {
                None
            }
        })
    }
    while let Some(event) = make_close_event(client.body(), &popup_selector) {
        client.process_event(false, event).await?;
    }
    Ok(())
}
