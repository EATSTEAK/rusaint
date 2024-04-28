use std::collections::HashMap;
use std::hash::Hash;

use lol_html::{element, html_content::ContentType, rewrite_str, RewriteStrSettings};
use roxmltree::Node;
use scraper::{Html, Selector};

use crate::webdynpro::error::{BodyError, UpdateBodyError};

use super::SapSsrClient;

type BodyUpdateWindowId = String;
type BodyUpdateContentId = String;
type BodyUpdateControlId = String;

#[derive(Debug)]
pub(super) enum BodyUpdateType {
    Full(BodyUpdateWindowId, BodyUpdateContentId, String),
    Delta(BodyUpdateWindowId, HashMap<BodyUpdateControlId, String>),
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct BodyUpdate {
    update: Option<BodyUpdateType>,
    initialize_ids: Option<String>,
    script_calls: Option<Vec<String>>,
    model_updates: Option<Vec<String>>,
    animation_updates: Option<Vec<String>>,
}

impl BodyUpdate {
    pub(super) fn new(response: &str) -> Result<BodyUpdate, UpdateBodyError> {
        let response_xml = roxmltree::Document::parse(response)?;
        let updates = response_xml
            .root()
            .first_child()
            .ok_or(UpdateBodyError::NoSuchNode("<updates>".to_string()))?;
        let update = updates.first_child().ok_or(UpdateBodyError::NoSuchNode(
            "<full-update> or <delta-update>".to_string(),
        ))?;
        let update_type: Option<BodyUpdateType>;
        if update.tag_name().name() == "full-update" {
            let windowid =
                update
                    .attribute("windowid")
                    .ok_or(UpdateBodyError::NoSuchAttribute {
                        node: "full-update".to_string(),
                        attribute: "windowid".to_string(),
                    })?;
            let content = update
                .first_child()
                .ok_or(UpdateBodyError::NoSuchContent("full-update".to_string()))?;
            let contentid = content
                .attribute("id")
                .ok_or(UpdateBodyError::NoSuchAttribute {
                    node: "content-update".to_string(),
                    attribute: "id".to_string(),
                })?;
            if content.tag_name().name() != "content-update" {
                return Err(UpdateBodyError::UnknownElement(
                    content.tag_name().name().to_owned(),
                ))?;
            }
            update_type = Some(BodyUpdateType::Full(
                windowid.to_owned(),
                contentid.to_owned(),
                content
                    .text()
                    .ok_or(UpdateBodyError::NoSuchContent("full-content".to_string()))?
                    .to_owned(),
            ));
        } else if update.tag_name().name() == "delta-update" {
            let windowid =
                update
                    .attribute("windowid")
                    .ok_or(UpdateBodyError::NoSuchAttribute {
                        node: "delta-update".to_string(),
                        attribute: "windowid".to_string(),
                    })?;
            let childrens = update.children().collect::<Vec<Node>>();
            let mut update_map: HashMap<BodyUpdateControlId, String> =
                HashMap::with_capacity(childrens.len());
            for children in childrens {
                let tag_name = children.tag_name().name();
                match tag_name {
                    "control-update" => {
                        let control_id =
                            children
                                .attribute("id")
                                .ok_or(UpdateBodyError::NoSuchAttribute {
                                    node: "control-update".to_string(),
                                    attribute: "id".to_string(),
                                })?;
                        let content = children
                            .first_child()
                            .ok_or(UpdateBodyError::NoSuchContent("control-update".to_string()))?;
                        update_map.insert(
                            control_id.to_owned(),
                            content
                                .text()
                                .ok_or(UpdateBodyError::NoSuchContent("content".to_string()))?
                                .to_owned(),
                        );
                    }
                    &_ => {
                        eprintln!("[WARN] Unknown body update {} is found, ignore.", tag_name);
                    }
                };
            }
            update_type = Some(BodyUpdateType::Delta(windowid.to_owned(), update_map));
        } else {
            return Err(UpdateBodyError::UnknownElement(
                update.tag_name().name().to_owned(),
            ))?;
        }
        // TODO: Apply additional updates to BodyUpdate struct.
        Ok(BodyUpdate {
            update: update_type,
            initialize_ids: None,
            script_calls: None,
            model_updates: None,
            animation_updates: None,
        })
    }
}

/// WebDynpro 페이지의 상태를 관리하는 구조체
pub struct Body {
    raw_body: String,
    document: Html,
    sap_ssr_client: SapSsrClient,
}

impl Hash for Body {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_body.hash(state);
    }
}

impl Body {
    pub(crate) fn new(body: String) -> Result<Body, BodyError> {
        let document = Html::parse_document(&body);
        let sap_ssr_client = Self::parse_sap_ssr_client(&document)?;
        Ok(Body {
            raw_body: body,
            document,
            sap_ssr_client,
        })
    }

    /// 페이지 도큐먼트의 HTML 텍스트를 반환합니다.
    pub fn raw_body(&self) -> &str {
        &self.raw_body
    }

    /// 도큐먼트 파싱을 위한 `scraper::Html` 구조체를 반환합니다.
    pub fn document(&self) -> &Html {
        &self.document
    }

    pub(crate) fn ssr_client(&self) -> &SapSsrClient {
        &self.sap_ssr_client
    }

    fn parse_sap_ssr_client(document: &Html) -> Result<SapSsrClient, BodyError> {
        let selector = Selector::parse(r#"#sap\.client\.SsrClient\.form"#).unwrap();
        let client_form = document
            .select(&selector)
            .next()
            .ok_or(BodyError::Invalid)?;
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
            action: html_escape::decode_html_entities(data.get("action").ok_or(
                BodyError::NoSuchAttribute("'action' field of SSR Form".to_string()),
            )?)
            .to_string(),
            charset: data
                .get("sap-charset")
                .ok_or(BodyError::NoSuchAttribute(
                    "'sap-charset' field of SSR Form".to_string(),
                ))?
                .to_owned(),
            wd_secure_id: data
                .get("sap-wd-secure-id")
                .ok_or(BodyError::NoSuchAttribute(
                    "'sap-wd-secure-id' field of SSR Form".to_string(),
                ))?
                .to_owned(),
            app_name: data
                .get("fesrAppName")
                .ok_or(BodyError::NoSuchAttribute(
                    "'fesrAppName' field of SSR Form".to_string(),
                ))?
                .to_owned(),
            use_beacon: (data
                .get("fesrUseBeacon")
                .ok_or(BodyError::NoSuchAttribute(
                    "'fesrUseBeacon' field of SSR Form".to_string(),
                ))?
                .to_owned()
                .as_str()
                == "true"),
        })
    }

    pub(super) fn apply(&mut self, updates: BodyUpdate) -> Result<(), UpdateBodyError> {
        if let Some(update) = updates.update {
            let output: String = match update {
                BodyUpdateType::Full(_, contentid, content) => {
                    let element_content_handlers =
                        vec![element!(format!(r#"[id="{}"]"#, contentid), |el| {
                            el.set_inner_content(&content, ContentType::Html);
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
                BodyUpdateType::Delta(windowid, controls) => {
                    let element_content_handlers = controls
                        .iter()
                        .map(|(control_id, content)| {
                            element!(
                                format!(r#"[id="{}_root_"] [id="{}"]"#, windowid, control_id),
                                move |el| {
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
            self.document = Html::parse_document(&output);
            self.raw_body = output;
        }
        Ok(())
    }
}
