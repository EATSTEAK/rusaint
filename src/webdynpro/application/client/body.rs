use std::collections::HashMap;

use lol_html::{element, html_content::ContentType, rewrite_str, RewriteStrSettings};
use roxmltree::Node;
use scraper::{Html, Selector};
use thiserror::Error;

use super::SapSsrClient;

#[derive(Error, Debug)]
pub enum WDBodyUpdateError {
    #[error("Failed to parse update document")]
    Parse(#[from] roxmltree::Error),
    #[error("Cannot find a node from given document: {0}")]
    CannotFindNode(String),
    #[error("Cannot find an attribute {attribute:?} from a node {node:?}")]
    CannotFindAttribute { node: String, attribute: String },
    #[error("{0} has no content")]
    NoContent(String),
    #[error("Unknown element found: {0}")]
    UnknownElement(String),
    #[error("Failed to rewrite body document")]
    Rewrite(#[from] lol_html::errors::RewritingError),
}

#[derive(Error, Debug)]
pub enum WDBodyError {
    #[error("Failed to parse body document")]
    Parse,
    #[error("Given body document is invalid")]
    Invalid,
}

type WDBodyUpdateWindowId = String;
type WDBodyUpdateControlId = String;

pub enum WDBodyUpdateType {
    Full(WDBodyUpdateWindowId, String),
    Delta(WDBodyUpdateWindowId, HashMap<WDBodyUpdateControlId, String>),
}

pub struct WDBodyUpdate {
    update: Option<WDBodyUpdateType>,
    initialize_ids: Option<String>,
    script_calls: Option<Vec<String>>,
    model_updates: Option<Vec<String>>,
    animation_updates: Option<Vec<String>>,
}

impl WDBodyUpdate {
    pub fn new(response: &str) -> Result<WDBodyUpdate, WDBodyUpdateError> {
        let response_xml = roxmltree::Document::parse(response)?;
        let updates = response_xml
            .root()
            .first_child()
            .ok_or(WDBodyUpdateError::CannotFindNode("<updates>".to_string()))?;
        let update = updates
            .first_child()
            .ok_or(WDBodyUpdateError::CannotFindNode(
                "<full-update> or <delta-update>".to_string(),
            ))?;
        let update_type: Option<WDBodyUpdateType>;
        if update.tag_name().name() == "full-update" {
            let windowid =
                update
                    .attribute("windowid")
                    .ok_or(WDBodyUpdateError::CannotFindAttribute {
                        node: "full-update".to_string(),
                        attribute: "windowid".to_string(),
                    })?;
            let content = update
                .first_child()
                .ok_or(WDBodyUpdateError::NoContent("full-update".to_string()))?;
            if content.tag_name().name() != "content-update" {
                return Err(WDBodyUpdateError::UnknownElement(content.tag_name().name().to_owned()));
            }
            update_type = Some(WDBodyUpdateType::Full(
                windowid.to_owned(),
                content
                    .text()
                    .ok_or(WDBodyUpdateError::NoContent("full-content".to_string()))?
                    .to_owned(),
            ));
        } else if update.tag_name().name() == "delta-update" {
            let windowid = update
                .attribute("windowid")
                .ok_or(WDBodyUpdateError::CannotFindAttribute {
                    node: "delta-update".to_string(),
                    attribute: "windowid".to_string(),
                })?;
            let childrens = update.children().collect::<Vec<Node>>();
            let mut update_map: HashMap<WDBodyUpdateControlId, String> =
                HashMap::with_capacity(childrens.len());
            for children in childrens {
                let tag_name = children.tag_name().name();
                match tag_name {
                    "control-update" => {
                        let control_id =
                            children.attribute("id").ok_or(WDBodyUpdateError::CannotFindAttribute {
                                node: "control-update".to_string(),
                                attribute: "id".to_string(),
                            })?;
                        let content = children.first_child().ok_or(WDBodyUpdateError::NoContent("control-update".to_string()))?;
                        update_map.insert(
                            control_id.to_owned(),
                            content
                                .text()
                                .ok_or(WDBodyUpdateError::NoContent("content".to_string()))?
                                .to_owned(),
                        );
                    }
                    &_ => {
                        eprintln!("[WARN] Unknown body update {} is found, ignore.", tag_name);
                    }
                };
            }
            update_type = Some(WDBodyUpdateType::Delta(windowid.to_owned(), update_map));
        } else {
            return Err(WDBodyUpdateError::UnknownElement(update.tag_name().name().to_owned()));
        }
        // TODO: Apply additional updates to WDBodyUpdate struct.
        Ok(WDBodyUpdate {
            update: update_type,
            initialize_ids: None,
            script_calls: None,
            model_updates: None,
            animation_updates: None,
        })
    }
}

pub struct WDBody {
    raw_body: String,
}

impl WDBody {
    pub fn new(body: String) -> WDBody {
        WDBody { raw_body: body }
    }

    pub fn raw_body(&self) -> &str {
        &self.raw_body
    }

    pub fn document(&self) -> Html {
        Html::parse_document(&self.raw_body)
    }

    pub fn parse_sap_ssr_client(&self) -> Result<SapSsrClient, WDBodyError> {
        let document = &self.document();
        let selector = Selector::parse(r#"#sap\.client\.SsrClient\.form"#).unwrap();
        let client_form = document
            .select(&selector)
            .next()
            .ok_or(WDBodyError::Invalid)?;
        let mut data = HashMap::<String, String>::new();
        data.insert(
            "action".to_owned(),
            client_form
                .value()
                .attr("action")
                .expect("Attribute not found or malformed")
                .to_string(),
        );
        let children_iter = client_form.children();
        children_iter.for_each(|item| {
            let item_tag = item.value().as_element().expect("Not a valid tag");
            let id = item_tag
                .id()
                .expect("id Attribute not found or malformed")
                .to_string();
            let value = item_tag
                .attr("value")
                .expect("value Attribute not found or malformed")
                .to_string();
            data.insert(id, value);
        });
        Ok(SapSsrClient {
            action: html_escape::decode_html_entities(
                data.get("action").ok_or(WDBodyError::Invalid)?,
            )
            .to_string(),
            charset: data
                .get("sap-charset")
                .ok_or(WDBodyError::Invalid)?
                .to_owned(),
            wd_secure_id: data
                .get("sap-wd-secure-id")
                .ok_or(WDBodyError::Invalid)?
                .to_owned(),
            app_name: data
                .get("fesrAppName")
                .ok_or(WDBodyError::Invalid)?
                .to_owned(),
            use_beacon: (data
                .get("fesrUseBeacon")
                .ok_or(WDBodyError::Invalid)?
                .to_owned()
                .as_str()
                == "true"),
        })
    }

    pub fn apply(&mut self, updates: WDBodyUpdate) -> Result<(), WDBodyUpdateError> {
        if let Some(update) = updates.update {
            let output: String = match update {
                WDBodyUpdateType::Full(windowid, content) => {
                    let element_content_handlers =
                        vec![element!(format!(r#"[id="{}"]"#, windowid), |el| {
                            el.replace(&content, ContentType::Html);
                            Ok(())
                        })];
                    rewrite_str(
                        &self.raw_body,
                        RewriteStrSettings {
                            element_content_handlers,
                            ..RewriteStrSettings::default()
                        },
                    )?
                }
                WDBodyUpdateType::Delta(windowid, controls) => {
                    let element_content_handlers = controls.iter().map(|(control_id, content)| {
                            element!(format!(r#"[id="{}"] [id="{}"]"#, windowid, control_id), move |el| {
                                    el.replace(&content, ContentType::Html);
                                    Ok(())
                                }
                            )
                        })
                        .collect();
                    rewrite_str(
                        &self.raw_body,
                        RewriteStrSettings {
                            element_content_handlers,
                            ..RewriteStrSettings::default()
                        },
                    )?
                }
            };
            self.raw_body = output;
        }
        Ok(())
    }
}
