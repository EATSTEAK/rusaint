use wdpe::command::WebDynproCommandExecutor;
use wdpe::command::element::complex::{
    SapTableBodyCommand, SapTableLSDataCommand, SapTableVerticalScrollEventCommand,
};
use wdpe::element::ElementDefWrapper;
use wdpe::element::complex::SapTableDef;
use wdpe::element::complex::sap_table::FromSapTable;
use wdpe::element::complex::sap_table::cell::{SapTableCell, SapTableCellWrapper};
use wdpe::element::definition::ElementDefinition;
use wdpe::element::parser::ElementParser;
use wdpe::error::{ElementError, WebDynproError};

use crate::client::USaintClient;

/// SAP 테이블 본문이 실제로 비어있는 경우를 판별합니다.
/// 첫 행의 첫 셀이 텍스트를 포함하며 "없습니다."가 들어있으면 비어있는 것으로 간주합니다.
pub(crate) fn is_sap_table_empty(
    body: &wdpe::element::complex::sap_table::SapTableBody,
    parser: &ElementParser,
) -> bool {
    let Some(first_row) = body.iter().next() else {
        return true;
    };
    if let Some(Ok(SapTableCellWrapper::Normal(cell))) = first_row.iter_value(parser).next()
        && let Some(ElementDefWrapper::TextView(tv_def)) = cell.content()
        && let Ok(tv) = parser.element_from_def(&tv_def)
    {
        return tv.text().contains("없습니다.");
    }
    false
}

pub(crate) async fn try_table_into_with_scroll<T: for<'body> FromSapTable<'body>>(
    client: &mut USaintClient,
    mut parser: ElementParser,
    table: SapTableDef,
) -> Result<Vec<T>, WebDynproError> {
    let row_count = parser
        .read(SapTableLSDataCommand::new(table.clone()))?
        .row_count()
        .map(|u| u.to_owned())
        .ok_or_else(|| ElementError::NoSuchData {
            element: table.clone().id().to_string(),
            field: "row_count".to_string(),
        })?
        .try_into()
        .unwrap();
    // 각 행의 절대 인덱스(row_index = DOM "rr")를 키로 모은다. SAP 테이블을 스크롤하며 읽으면
    // 페이지가 보통 한 행씩 겹쳐 렌더되는데, 기존 로직은 그 겹침을 "맨 앞 drain"으로만 처리해
    // (행수가 딱 맞아떨어질 때) 겹친 행이 중복으로 남고 그만큼 마지막 행을 놓쳤다. 인덱스를 키로
    // 쓰면 겹침은 자동으로 접히고 누락이 생기지 않는다.
    let mut collected: std::collections::BTreeMap<u32, T> = std::collections::BTreeMap::new();
    let mut table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
    // 겹침·클램핑이 어떻든 반드시 끝나도록 반복 상한을 둔다.
    let mut forced_tail = false;
    for _ in 0..(row_count + 2) {
        let header = table_body.header();
        let before = collected.len();
        for row in table_body.iter() {
            let idx = match row.row_index() {
                Some(idx) => idx,
                None => continue,
            };
            if !collected.contains_key(&idx) {
                collected.insert(idx, row.try_row_into::<T>(header, &parser)?);
            }
        }
        if collected.len() >= row_count {
            break;
        }
        let made_progress = collected.len() > before;
        let next_offset: u32 = if made_progress {
            // 정상 전진 — 지금까지 모은 수(0-base)부터 다음 페이지.
            forced_tail = false;
            collected.len().try_into().unwrap()
        } else if !forced_tail {
            // 겹침으로 새 행이 안 늘면, 마지막 창을 한 번 강제로 훑어 꼬리 행을 확보한다.
            forced_tail = true;
            u32::try_from(row_count.saturating_sub(1)).unwrap()
        } else {
            // 꼬리까지 훑어도 더 없으면 끝.
            break;
        };
        let event = parser.read(SapTableVerticalScrollEventCommand::new(
            table.clone(),
            next_offset,
            "",
            "SCROLLBAR",
            false,
            false,
            false,
            false,
        ))?;
        client.process_event(false, event).await?;
        parser = ElementParser::new(client.body());
        table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
    }
    Ok(collected.into_values().collect())
}
