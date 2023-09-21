use indexmap::IndexMap;
use rusaint::webdynpro::event::{EventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}, event_queue::EventQueue};

#[test]
fn event_serialize() {
    let mut parameters = IndexMap::new();
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
    assert_eq!(event.to_string(), "Button_Press~E002Id~E004WD01A8~E003~E002ClientAction~E004submit~E005ResponseData~E004delta~E003~E002~E003");
}

#[test]
fn event_queue_serialize() {
    let mut parameters = IndexMap::new();
        parameters.insert("Id".to_string(), "WD0213".to_string());
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
    let mut parameters_two = IndexMap::new();
        parameters_two.insert("Id".to_string(), "sap.client.SsrClient.form".to_string());
        parameters_two.insert("Async".to_string(), "false".to_string());
        parameters_two.insert("FocusInfo".to_string(), "@{\"sFocussedId\":\"WD0213\"}".to_string());
        parameters_two.insert("Hash".to_string(), "".to_string());
        parameters_two.insert("DomChanged".to_string(), "false".to_string());
        parameters_two.insert("IsDirty".to_string(), "false".to_string());
    let ucf_params_two = UcfParametersBuilder::default()
        .response(Some(UcfResponseData::Delta))
        .build()
        .unwrap();
    let event_two = EventBuilder::default()
            .control("Form".to_owned())
            .event("Request".to_owned())
            .parameters(parameters_two)
            .ucf_parameters(ucf_params_two)
            .build()
            .unwrap();
    let mut queue = EventQueue::new();
    queue.add(event);
    queue.add(event_two);
    assert_eq!(queue.serialize_and_clear(), "Button_Press~E002Id~E004WD0213~E003~E002ClientAction~E004submit~E005ResponseData~E004delta~E003~E002~E003~E001Form_Request~E002Id~E004sap.client.SsrClient.form~E005Async~E004false~E005FocusInfo~E004~0040~007B~0022sFocussedId~0022~003A~0022WD0213~0022~007D~E005Hash~E004~E005DomChanged~E004false~E005IsDirty~E004false~E003~E002ResponseData~E004delta~E003~E002~E003");
}