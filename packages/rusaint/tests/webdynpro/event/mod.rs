use rusaint::webdynpro::event::{
    EventBuilder,
    ucf_parameters::{UcfAction, UcfParametersBuilder, UcfResponseData},
};
use std::collections::HashMap;
use test_log::test;

#[test]
fn event_serialize() {
    let mut parameters = HashMap::new();
    parameters.insert("Id".to_string(), "WD01A8".to_string());
    let ucf_params = UcfParametersBuilder::default()
        .response(Some(UcfResponseData::Delta))
        .action(Some(UcfAction::Submit))
        .build()
        .unwrap();
    let event = EventBuilder::default()
        .control("Button".to_owned())
        .event("Press".to_owned())
        .parameters(parameters)
        .ucf_parameters(ucf_params)
        .build()
        .unwrap();
    assert_eq!(
        event.to_string(),
        "Button_Press~E002Id~E004WD01A8~E003~E002ClientAction~E004submit~E005ResponseData~E004delta~E003~E002~E003"
    );
}
