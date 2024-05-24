use std::collections::HashMap;

use crate::webdynpro::{
    client::body::Body,
    element::{definition::ElementDefinition, Element, ElementDefWrapper},
    error::{ElementError, WebDynproError},
};

use super::{cell::SapTableCell, SapTableHeader, SapTableRow};

/// [`SapTable`]의 내부 데이터로 표현될 수 있는 형에 구현하는 트레이트
pub trait FromSapTable<'body>: Sized {
    /// [`SapTableRow`]를 해당 형으로 변환하고자 시도하는 함수
    fn from_table(
        body: &'body Body,
        header: &'body SapTableHeader<'body>,
        row: &'body SapTableRow<'body>,
    ) -> Result<Self, WebDynproError>;
}

impl<'body> FromSapTable<'body> for Vec<Option<String>> {
    fn from_table(
        body: &'body Body,
        _header: &'body SapTableHeader<'body>,
        row: &'body SapTableRow<'body>,
    ) -> Result<Self, WebDynproError> {
        let iter = row.iter_value(body);
        let vec = iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(ElementDefWrapper::TextView(tv)) => {
                        Some(tv.from_body(body).ok()?.text().to_owned())
                    }
                    Some(ElementDefWrapper::Caption(cap)) => Some(
                        cap.from_body(body)
                            .ok()?
                            .lsdata()
                            .text()
                            .unwrap_or(&String::default())
                            .to_owned(),
                    ),
                    _ => None,
                },
                Err(_err) => None,
            })
            .collect::<Vec<Option<String>>>();
        Ok(vec)
    }
}

impl<'body> FromSapTable<'body> for Vec<String> {
    fn from_table(
        body: &'body Body,
        _header: &'body SapTableHeader<'body>,
        row: &'body SapTableRow<'body>,
    ) -> Result<Self, WebDynproError> {
        let iter = row.iter_value(body);
        iter.map(|val| match val {
            Ok(cell) => match cell.content() {
                Some(ElementDefWrapper::TextView(tv)) => Ok(tv.from_body(body)?.text().to_owned()),
                Some(ElementDefWrapper::Caption(cap)) => Ok(cap
                    .from_body(body)?
                    .lsdata()
                    .text()
                    .unwrap_or(&String::default())
                    .to_owned()),
                _ => Err(ElementError::InvalidContent {
                    element: "Cell Content".to_string(),
                    content: "Cannot convert to string".to_string(),
                })?,
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<String>, WebDynproError>>()
    }
}

impl<'body> FromSapTable<'body> for HashMap<String, String> {
    fn from_table(
        body: &'body Body,
        header: &'body SapTableHeader<'body>,
        row: &'body SapTableRow<'body>,
    ) -> Result<Self, WebDynproError> {
        let header_iter = header.iter_value(body);
        let header_string = header_iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(ElementDefWrapper::TextView(tv)) => {
                        Ok(tv.from_body(body)?.text().to_owned())
                    }
                    Some(ElementDefWrapper::Caption(cap)) => Ok(cap
                        .from_body(body)?
                        .lsdata()
                        .text()
                        .unwrap_or(&String::default())
                        .to_owned()),
                    None => Ok(cell.id().to_owned()),
                    _ => Err(ElementError::InvalidContent {
                        element: "Header Cell Content".to_string(),
                        content: "Cannot convert to string".to_string(),
                    })?,
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<String>, WebDynproError>>()?;
        let iter = row.iter_value(body);
        let row_string = iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(ElementDefWrapper::TextView(tv)) => {
                        Ok(tv.from_body(body)?.text().to_owned())
                    }
                    Some(ElementDefWrapper::Caption(cap)) => Ok(cap
                        .from_body(body)?
                        .lsdata()
                        .text()
                        .unwrap_or(&String::default())
                        .to_owned()),
                    Some(wrapper) => Ok(wrapper.id().to_owned()),
                    None => Ok("".to_owned()),
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<String>, WebDynproError>>()?;
        let zip = header_string
            .into_iter()
            .zip(row_string.into_iter())
            .collect::<HashMap<String, String>>();
        Ok(zip)
    }
}
