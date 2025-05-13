use crate::application::USaintClient;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::command::element::complex::{
    SapTableBodyCommand, SapTableLSDataCommand, SapTableVerticalScrollEventCommand,
};
use crate::webdynpro::element::complex::SapTableDef;
use crate::webdynpro::element::complex::sap_table::FromSapTable;
use crate::webdynpro::element::definition::ElementDefinition;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::error::{ElementError, WebDynproError};

pub(crate) async fn try_table_into_with_scroll<T: for<'body> FromSapTable<'body>>(
    client: &mut USaintClient,
    mut parser: ElementParser,
    table: SapTableDef,
) -> Result<Vec<T>, WebDynproError> {
    let row_count = parser
        .read(SapTableLSDataCommand::new(table.clone()))?
        .row_count()
        .map(|u| u.to_owned())
        .ok_or_else(|| {
            WebDynproError::Element(ElementError::NoSuchData {
                element: table.clone().id().to_string(),
                field: "row_count".to_string(),
            })
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
