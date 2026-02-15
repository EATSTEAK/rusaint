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
    let mut table_body = parser.read(SapTableBodyCommand::new(table.clone()))?;
    let mut results: Vec<T> = Vec::with_capacity(row_count);
    while results.len() < row_count {
        let mut partial_results = table_body.try_table_into::<T>(&parser)?;
        if results.len() + partial_results.len() > row_count {
            let overflowed = results.len() + partial_results.len() - row_count;
            partial_results.drain(0..overflowed);
        }
        results.append(&mut partial_results);
        if results.len() < row_count {
            let event = parser.read(SapTableVerticalScrollEventCommand::new(
                table.clone(),
                results.len().try_into().unwrap(),
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
    }
    Ok(results)
}
