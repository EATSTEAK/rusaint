use super::{
    cell::SapTableCellDefWrapper, property::SapTableRowType, row::SapTableRow, FromSapTable,
    SapTableDef, SapTableHeader,
};
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::{
    element::definition::ElementDefinition,
    error::{ElementError, WebDynproError},
};
use std::{collections::HashMap, ops::Index};
use tl::Bytes;

/// [`SapTable`](super::SapTable) 내부 테이블
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct SapTableBody {
    table_def: SapTableDef,
    header: SapTableHeader,
    rows: Vec<SapTableRow>,
}

impl<'a> SapTableBody {
    pub(super) fn new(
        table_def: SapTableDef,
        tag: tl::HTMLTag<'a>,
        parser: &'a ElementParser,
    ) -> Result<SapTableBody, ElementError> {
        let children = tag.children();
        let tag_iter = children.top()
            .as_slice()
            .iter()
            .filter_map(|handle| handle.get(parser.dom().parser()))
            .filter_map(|node| node.as_tag());
        let mut header_iter = tag_iter.clone()
            .filter_map(|tag| SapTableHeader::new(table_def.clone(), tag.clone(), parser).ok());
        let Some(header) = header_iter.next() else {
            return Err(ElementError::NoSuchContent {
                element: table_def.id().to_owned(),
                content: "Header of table".to_owned(),
            });
        };
        if header_iter.next().is_some() {
            return Err(ElementError::InvalidContent {
                element: table_def.id().to_owned(),
                content: "Multiple header in table".to_owned(),
            });
        }
        let mut rows: Vec<SapTableRow> = Vec::new();
        // Def, rowsize, colsize
        type CellSpanInfo = (SapTableCellDefWrapper, u32, u32);
        let mut spans: HashMap<u32, CellSpanInfo> = HashMap::new();
        for row_tag in tag_iter.clone() {
            let row_type = row_tag
                .attributes()
                .get("rt")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| Some(s.into()))
                .unwrap_or(SapTableRowType::default());
            let row_count = row_tag
                .attributes()
                .get("rr")
                .flatten()
                .and_then(Bytes::try_as_utf8_str)
                .and_then(|s| s.parse::<u32>().ok());
            if matches!(row_type, SapTableRowType::Header) {
                continue;
            }

            // If it meets empty row(zero-index), instantly terminate repetition
            if row_count.is_some_and(|c| c == 0) {
                break;
            }
            let subcts = row_tag
                .query_selector(parser.dom().parser(), "[subct]")
                .into_iter()
                .flatten()
                .filter_map(|handle| handle.get(parser.dom().parser()))
                .filter_map(|node| node.as_tag());
            let mut cells: Vec<SapTableCellDefWrapper> = Vec::new();
            let mut col_counter: u32 = 0;
            for cell_tag in subcts {
                let cell = SapTableCellDefWrapper::from_tag(table_def.clone(), cell_tag);
                if let Some(cell) = cell {
                    if spans.contains_key(&col_counter) {
                        let spanned_cell = spans.remove(&col_counter).unwrap();
                        if spanned_cell.1 - 1 > 1 {
                            spans.insert(
                                col_counter,
                                (spanned_cell.0.clone(), spanned_cell.1 - 1, spanned_cell.2),
                            );
                        }
                        for _ in 0..spanned_cell.2 {
                            col_counter += 1;
                            cells.push(spanned_cell.0.clone());
                        }
                    }
                    // TODO: Remove boilerplate to get attribute str from tag
                    let rowspan = cell_tag
                        .attributes()
                        .get("rowspan")
                        .flatten()
                        .and_then(Bytes::try_as_utf8_str)
                        .and_then(|str| str.parse::<u32>().ok())
                        .unwrap_or(1);
                    let colspan = cell_tag
                        .attributes()
                        .get("colspan")
                        .flatten()
                        .and_then(Bytes::try_as_utf8_str)
                        .and_then(|str| str.parse::<u32>().ok())
                        .unwrap_or(1);
                    if rowspan > 1 {
                        spans.insert(col_counter, (cell.clone(), rowspan, colspan));
                    }
                    for _ in 0..rowspan {
                        cells.push(cell.clone());
                        col_counter += 1;
                    }
                }
            }
            // checks if spanning lasts after last cell
            if spans.contains_key(&col_counter) {
                let spanned_cell = spans.remove(&col_counter).unwrap();
                if spanned_cell.1 - 1 > 1 {
                    spans.insert(
                        col_counter,
                        (spanned_cell.0.clone(), spanned_cell.1 - 1, spanned_cell.2),
                    );
                }
                for _ in 0..spanned_cell.2 {
                    col_counter += 1;
                    cells.push(spanned_cell.0.clone());
                }
            }
            if let Ok(row) = SapTableRow::new(table_def.clone(), row_tag.clone(), cells) {
                rows.push(row);
            }
        }
        Ok(SapTableBody {
            table_def,
            header,
            rows,
        })
    }

    /// 헤더 행을 제외한 행의 갯수를 반환합니다.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// 내부 행의 Iterator를 반환합니다.
    pub fn iter(&'a self) -> impl Iterator<Item = &SapTableRow> + ExactSizeIterator {
        self.rows.iter()
    }

    /// 내부 행에 헤더 행을 포함한 튜플의 Iterator를 반환합니다.
    pub fn zip_header(
        &'a self,
    ) -> impl Iterator<Item = (&SapTableHeader, &SapTableRow)> + ExactSizeIterator {
        self.rows.iter().map(|row| (self.header(), row))
    }

    /// 이 테이블의 원본 [`SapTableDef`]를 반환합니다.
    pub fn table_def(&self) -> SapTableDef {
        self.table_def.clone()
    }

    /// 헤더 행을 반환합니다.
    pub fn header(&self) -> &SapTableHeader {
        &self.header
    }

    /// 테이블을 [`FromSapTable`]을 구현하는 형의 [`Vec`]으로 변환합니다.
    pub fn try_table_into<T: FromSapTable<'a>>(
        &'a self,
        parser: &'a ElementParser,
    ) -> Result<Vec<T>, WebDynproError> {
        self.iter()
            .map(|row| T::from_table(self.header(), row, parser))
            .collect::<Result<Vec<T>, WebDynproError>>()
    }
}

impl<'a> Index<usize> for SapTableBody {
    type Output = SapTableRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
