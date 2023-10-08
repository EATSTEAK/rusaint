use std::{borrow::Cow, cell::OnceCell, collections::HashMap};

use scraper::Selector;

use crate::webdynpro::{
    element::{define_element_interactable, ElementDef, Interactable, SubElement, SubElementDef},
    error::{BodyError, ElementError, WebDynproError},
    event::Event,
};

use self::cell::{
    SapTableHeaderCell, SapTableHierarchicalCell,
    SapTableMatrixCell, SapTableNormalCell,
    SapTableSelectionCell, SapTableCellWrapper,
};

pub type SapTableBody<'a> = Vec<Vec<SapTableCellWrapper<'a>>>;

define_element_interactable! {
    SapTable<"ST", "SapTable"> {
        table: OnceCell<Option<SapTableBody<'a>>>,
    },
    SapTableLSData {
        title_text: String => "0",
        accessibility_description: String => "1",
        row_count: u32 => "2",
        col_count: u32 => "3",
    }
}

pub enum AccessType {
    Invalid,
    Standard,
    Range,
    Toggle,
    SelectAll,
    DeselectAll,
}

impl ToString for AccessType {
    fn to_string(&self) -> String {
        match self {
            AccessType::Invalid => "INVALID",
            AccessType::Standard => "STANDARD",
            AccessType::Range => "RANGE",
            AccessType::Toggle => "TOGGLE",
            AccessType::SelectAll => "SELECT_ALL",
            AccessType::DeselectAll => "DESELECT_ALL",
        }
        .to_owned()
    }
}

impl<'a> SapTable<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
            table: OnceCell::new(),
        }
    }

    pub fn table(&self) -> Option<&SapTableBody<'a>> {
        self.table.get_or_init(|| self.parse_table().ok()).as_ref()
    }

    fn parse_table(&self) -> Result<SapTableBody<'a>, WebDynproError> {
        let def: ElementDef<'a, SapTable<'a>> = {
            if let Cow::Borrowed(id) = self.id {
                ElementDef::new(id)
            } else {
                ElementDef::new_dynamic((&self.id).to_string())
            }
        };
        let element = self.element_ref;
        let elem_value = element.value();
        let tbody_selector = Selector::parse(
            format!(
                r#"[id="{}-contentTBody"]"#,
                elem_value.id().ok_or(ElementError::NoSuchData {
                    element: self.id.clone().into_owned(),
                    field: "id".to_string()
                })?
            )
            .as_str(),
        )
        .or(Err(BodyError::InvalidSelector))?;
        let tbody = element
            .select(&tbody_selector)
            .next()
            .ok_or(ElementError::NoSuchContent {
                element: self.id.clone().into_owned(),
                content: "Table content".to_string(),
            })?;
        Ok(tbody
            .children()
            .filter_map(|node| scraper::ElementRef::wrap(node))
            .map(|row_ref| -> Vec<SapTableCellWrapper<'a>> {
                let subct_selector = Selector::parse("[subct]").unwrap();
                let subcts = row_ref.select(&subct_selector);
                subcts
                    .filter_map(|subct_ref| -> Option<SapTableCellWrapper<'a>> {
                        let subct_value = subct_ref.value();
                        match subct_value.attr("subct") {
                            Some(SapTableNormalCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableNormalCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableHeaderCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableHeaderCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableHierarchicalCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableHierarchicalCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableMatrixCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableMatrixCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            Some(SapTableSelectionCell::SUBCONTROL_ID) => Some(
                                SubElementDef::<_, SapTableSelectionCell>::new_dynamic(
                                    def.clone(),
                                    subct_value.id()?.to_owned(),
                                )
                                .from_elem(subct_ref)
                                .ok()?
                                .wrap(),
                            ),
                            _ => None,
                        }
                    })
                    .collect::<Vec<SapTableCellWrapper<'a>>>()
            })
            .collect())
    }

    pub fn row_select(
        &self,
        row_index: i32,
        row_user_data: &str,
        cell_user_data: &str,
        access_type: AccessType,
        trigger_cell_id: &str,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            ("RowIndex".to_string(), format!("{}", row_index)),
            ("RowUserData".to_string(), row_user_data.to_owned()),
            ("CellUserData".to_string(), cell_user_data.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
            ("TriggerCellId".to_string(), trigger_cell_id.to_owned()),
        ]);
        self.fire_event("RowSelect".to_string(), parameters)
    }

    pub fn cell_select(
        &self,
        cell_id: &str,
        cell_type: &str,
        row_index: i32,
        col_index: i32,
        row_user_data: &str,
        cell_user_data: &str,
        access_type: AccessType,
    ) -> Result<Event, WebDynproError> {
        let parameters: HashMap<String, String> = HashMap::from([
            ("Id".to_string(), self.id.clone().to_string()),
            ("CellId".to_string(), cell_id.to_owned()),
            ("CellType".to_string(), cell_type.to_owned()),
            ("RowIndex".to_string(), format!("{}", row_index)),
            ("ColIndex".to_string(), format!("{}", col_index)),
            ("RowUserData".to_string(), row_user_data.to_owned()),
            ("CellUserData".to_string(), cell_user_data.to_owned()),
            ("AccessType".to_string(), access_type.to_string()),
        ]);
        self.fire_event("CellSelect".to_string(), parameters)
    }
}

pub mod cell;
