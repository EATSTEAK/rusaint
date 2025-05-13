use std::collections::HashMap;

use super::{SapTableHeader, SapTableRow, cell::SapTableCell};
use crate::webdynpro::element::ElementWrapper;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::error::{ElementError, WebDynproError};

/// [`SapTable`](super::SapTable)의 내부 데이터로 표현될 수 있는 형에 구현하는 트레이트
pub trait FromSapTable<'body>: Sized {
    /// [`SapTableRow`]를 해당 형으로 변환하고자 시도하는 함수
    fn from_table(
        header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError>;
}

impl<'body> FromSapTable<'body> for Vec<Option<String>> {
    fn from_table(
        _header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let iter = row.iter_value(parser);
        let vec = iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(wrapper) => Ok(ElementWrapper::from_def(&wrapper, parser)?.textise().ok()),
                    None => Ok(None),
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<Option<String>>, WebDynproError>>()?;
        Ok(vec)
    }
}

impl<'body> FromSapTable<'body> for Vec<String> {
    fn from_table(
        _header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let iter = row.iter_value(parser);
        iter.map(|val| match val {
            Ok(cell) => match cell.content() {
                Some(wrapper) => Ok(ElementWrapper::from_def(&wrapper, parser)?.textise()?),
                None => Err(ElementError::NoSuchContent {
                    element: "Cell Content".to_string(),
                    content: "No content provided".to_string(),
                })?,
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<String>, WebDynproError>>()
    }
}

impl<'body> FromSapTable<'body> for Vec<(String, Option<String>)> {
    fn from_table(
        header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let header_iter = header.iter_value(parser);
        let header_string = header_iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(wrapper) => Ok(ElementWrapper::from_def(&wrapper, parser)?.textise()?),
                    None => Ok(cell.id().to_owned()),
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<String>, WebDynproError>>()?;
        let iter = row.iter_value(parser);
        let row_string = iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(wrapper) => Ok(ElementWrapper::from_def(&wrapper, parser)?.textise().ok()),
                    None => Ok(None),
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<Option<String>>, WebDynproError>>()?;
        let zip = header_string
            .into_iter()
            .zip(row_string)
            .collect::<Vec<(String, Option<String>)>>();
        Ok(zip)
    }
}

impl<'body> FromSapTable<'body> for Vec<(String, String)> {
    fn from_table(
        header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let header_iter = header.iter_value(parser);
        let header_string = header_iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(wrapper) => Ok(ElementWrapper::from_def(&wrapper, parser)?.textise()?),
                    None => Ok(cell.id().to_owned()),
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<String>, WebDynproError>>()?;
        let iter = row.iter_value(parser);
        let row_string = iter
            .map(|val| match val {
                Ok(cell) => match cell.content() {
                    Some(wrapper) => Ok(ElementWrapper::from_def(&wrapper, parser)?
                        .textise()
                        .unwrap_or(wrapper.id().to_string())),
                    None => Ok("".to_owned()),
                },
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<String>, WebDynproError>>()?;
        let zip = header_string
            .into_iter()
            .zip(row_string)
            .collect::<Vec<(String, String)>>();
        Ok(zip)
    }
}

impl<'body> FromSapTable<'body> for HashMap<String, String> {
    fn from_table(
        header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let vec = row.try_row_into::<Vec<(String, String)>>(header, parser)?;
        Ok(vec.into_iter().collect())
    }
}

impl<'body> FromSapTable<'body> for HashMap<String, Option<String>> {
    fn from_table(
        header: &'body SapTableHeader,
        row: &'body SapTableRow,
        parser: &'body ElementParser,
    ) -> Result<Self, WebDynproError> {
        let vec = row.try_row_into::<Vec<(String, Option<String>)>>(header, parser)?;
        Ok(vec.into_iter().collect())
    }
}
