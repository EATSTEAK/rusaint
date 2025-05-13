use super::{EVENT_SPECTATOR, Event};
use std::collections::LinkedList;

#[derive(Debug)]
pub(crate) struct EventQueue(LinkedList<Event>);

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue(LinkedList::new())
    }

    pub fn serialize_and_clear(&mut self) -> String {
        let mut owned = "".to_owned();
        let events = &self.0;
        for (idx, event) in events.iter().enumerate() {
            owned.push_str(&event.serialize());
            if idx < events.len() - 1 {
                owned.push_str(EVENT_SPECTATOR);
            }
        }
        let _ = &self.0.clear();
        owned
    }

    pub fn add(&mut self, evt: Event) {
        self.0.push_back(evt)
    }

    #[allow(unused)]
    pub fn remove(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::webdynpro::event::{
        EventBuilder,
        event_queue::EventQueue,
        ucf_parameters::{UcfAction, UcfParametersBuilder, UcfResponseData},
    };

    #[test]
    fn event_queue_serialize() {
        let mut parameters = HashMap::new();
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
        let mut parameters_two = HashMap::new();
        parameters_two.insert("Id".to_string(), "sap.client.SsrClient.form".to_string());
        parameters_two.insert("Async".to_string(), "false".to_string());
        parameters_two.insert(
            "FocusInfo".to_string(),
            "@{\"sFocussedId\":\"WD0213\"}".to_string(),
        );
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
        assert_eq!(queue.serialize_and_clear().len(), "Button_Press~E002Id~E004WD0213~E003~E002ClientAction~E004submit~E005ResponseData~E004delta~E003~E002~E003~E001Form_Request~E002FocusInfo~E004~0040~007B~0022sFocussedId~0022~003A~0022WD0213~0022~007D~E005Id~E004sap.client.SsrClient.form~E005Async~E004false~E005Hash~E004~E005IsDirty~E004false~E005DomChanged~E004false~E003~E002ResponseData~E004delta~E003~E002~E003".len());
    }
}
