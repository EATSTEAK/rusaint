use std::collections::HashMap;

use scraper::{Html, Selector};
use thiserror::Error;

use super::SapSsrClient;

#[derive(Error, Debug)]
pub enum WDBodyUpdateError {
    #[error("Failed to parse update document")]
    Parse(#[from] roxmltree::Error),
    #[error("Given update document is invalid")]
    Invalid
}

#[derive(Error, Debug)]
pub enum WDBodyError {
    #[error("Failed to parse body document")]
    Parse,
    #[error("Given body document is invalid")]
    Invalid
}

type WDBodyUpdateWindowId = String;
type WDBodyUpdateControlId = String;

pub enum WDBodyUpdateType {
    Full(WDBodyUpdateWindowId, String),
    Delta(WDBodyUpdateWindowId, HashMap<WDBodyUpdateControlId, String>)
}

pub struct WDBodyUpdate {
    update: Option<WDBodyUpdateType>,
    initialize_ids: Option<String>,
    script_calls: Option<Vec<String>>,
    model_updates: Option<Vec<String>>,
    animation_updates: Option<Vec<String>>
}

impl WDBodyUpdate {
    pub fn new(response: &str) -> Result<WDBodyUpdate, WDBodyUpdateError> {
        let response_xml = roxmltree::Document::parse(response)?;
        let updates = response_xml.root().first_child().ok_or(WDBodyUpdateError::Invalid)?;
        let update = updates.first_child().ok_or(WDBodyUpdateError::Invalid)?;
        let update_type: Option<WDBodyUpdateType>;
        if update.tag_name().name() == "full-update" {
            let windowid = update.attribute("windowid").ok_or(WDBodyUpdateError::Invalid)?;
            let content = update.first_child().ok_or(WDBodyUpdateError::Invalid)?;
            if content.tag_name().name() != "content-update" { return Err(WDBodyUpdateError::Invalid) }
            println!("{:?}", content.text());
            update_type = Some(WDBodyUpdateType::Full(windowid.to_owned(), content.text().ok_or(WDBodyUpdateError::Invalid)?.to_owned()));
        } else if update.tag_name().name() == "delta-update" {
            todo!("implement delta update");
        } else { return Err(WDBodyUpdateError::Invalid) }
        Ok(
            WDBodyUpdate {
                update: update_type,
                initialize_ids: None,
                script_calls: None,
                model_updates: None,
                animation_updates: None
            }
        )
    }
}

pub struct WDBody {
    raw_body: String
}


impl WDBody {

    pub fn new(body: String) -> WDBody {
        WDBody {
            raw_body: body
        }
    }

    pub fn document(&self) -> Html {
        Html::parse_document(&self.raw_body)
    }

    pub fn parse_sap_ssr_client(&self) -> Result<SapSsrClient, WDBodyError>  {
        let document = &self.document();
        let selector = Selector::parse(r#"#sap\.client\.SsrClient\.form"#).unwrap();
        let client_form = document.select(&selector).next().ok_or(WDBodyError::Invalid)?;
        let mut data = HashMap::<String, String>::new();
        data.insert("action".to_owned(), 
        client_form.value().attr("action")
            .expect("Attribute not found or malformed").to_string()
        );
        let children_iter = client_form.children();
        children_iter.for_each(|item| {
            let item_tag = item.value().as_element().expect("Not a valid tag");
            let id = item_tag.id().expect("id Attribute not found or malformed").to_string();
            let value = item_tag.attr("value").expect("value Attribute not found or malformed").to_string();
            data.insert(id, value);
        });
        Ok(SapSsrClient {
            action: html_escape::decode_html_entities(data.get("action").ok_or(WDBodyError::Invalid)?).to_string(),
            charset: data.get("sap-charset").ok_or(WDBodyError::Invalid)?.to_owned(),
            wd_secure_id: data.get("sap-wd-secure-id").ok_or(WDBodyError::Invalid)?.to_owned(),
            app_name: data.get("fesrAppName").ok_or(WDBodyError::Invalid)?.to_owned(),
            use_beacon: (data.get("fesrUseBeacon").ok_or(WDBodyError::Invalid)?.to_owned().as_str() == "true")
        })
    }

    pub fn apply(&mut self, updates: WDBodyUpdate) {
        if let Some(update) = updates.update {
            if let WDBodyUpdateType::Full(windowid, content) = update {
                todo!("mutate body");
            }
        }
    }
}