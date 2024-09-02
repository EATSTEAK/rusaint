use crate::webdynpro::element::{normalize_lsjson, ElementWrapper, EventParameterMap};
use crate::webdynpro::error::{BodyError, ElementError, WebDynproError};
use crate::webdynpro::event::ucf_parameters::UcfParameters;
use scraper::ElementRef;
use serde_json::Value;
use std::collections::HashMap;
use std::iter::Map;

/// 엘리먼트의 lsdata 속성을 파싱합니다.
pub(super) fn parse_lsdata(raw_lsdata: &str) -> Result<Value, WebDynproError> {
    let normalized = normalize_lsjson(raw_lsdata);
    Ok(serde_json::from_str(&normalized)
        .or(Err(ElementError::InvalidLSData(raw_lsdata.to_string())))?)
}

/// 엘리먼트의 lsevents 속성을 파싱합니다.
pub(super) fn parse_lsevents(raw_lsevents: &str) -> Result<EventParameterMap, WebDynproError> {
    let normalized = normalize_lsjson(raw_lsevents);
    let json: Map<String, Value> = serde_json::from_str::<Map<String, Value>>(&normalized)
        .or(Err(BodyError::Invalid(
            "Cannot deserialize lsevents field".to_string(),
        )))?
        .to_owned();
    Ok(json.into_iter().flat_map(|(key, value)| -> Result<(String, (UcfParameters, HashMap<String, String>)), BodyError> {
        let mut parameters = value.as_array().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?.to_owned().into_iter();
        let raw_ucf = parameters.next().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?;
        let ucf: UcfParameters = serde_json::from_value(raw_ucf).or(Err(BodyError::Invalid("Cannot deserialize lsevents field".to_string())))?;
        let mut custom = parameters.next().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?.as_object().ok_or(BodyError::Invalid("Cannot deserialize lsevents field".to_string()))?.to_owned();
        let custom_map = custom.iter_mut().map(|(key, value)| {
            (key.to_owned(), value.to_string())
        }).collect::<HashMap<String, String>>();
        Ok((key, (ucf, custom_map)))
    }).collect::<EventParameterMap>())
}

/// 엘리먼트의 자식 엘리먼트를 가져옵니다.
pub(super) fn children_element<'body>(
    root: scraper::ElementRef<'body>,
) -> Vec<ElementWrapper<'body>> {
    let mut next_refs = vec![root];
    let mut cts: Vec<ElementRef<'_>> = vec![];
    while let Some(elem) = next_refs.pop() {
        for child in elem.children() {
            if let scraper::Node::Element(child_elem) = child.value() {
                let child_elem_ref = scraper::ElementRef::wrap(child).unwrap();
                if child_elem.attr("ct").is_some() {
                    cts.push(child_elem_ref);
                } else {
                    next_refs.push(child_elem_ref);
                }
            }
        }
    }
    cts.into_iter()
        .rev()
        .filter_map(|eref| ElementWrapper::from_ref(eref).ok())
        .collect()
}