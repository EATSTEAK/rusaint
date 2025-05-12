use crate::application::USaintClient;
use crate::model::SemesterType;
use crate::webdynpro::command::WebDynproCommandExecutor;
use crate::webdynpro::command::element::selection::ComboBoxLSDataCommand;
use crate::webdynpro::element::definition::ElementDefinition;
use crate::webdynpro::element::parser::ElementParser;
use crate::webdynpro::element::selection::ComboBoxDef;
use crate::webdynpro::error::{ElementError, WebDynproError};

pub(crate) fn get_selected_semester(
    client: &USaintClient,
    year_def: &ComboBoxDef,
    semester_def: &ComboBoxDef,
) -> Result<(u32, SemesterType), WebDynproError> {
    let parser = ElementParser::new(client.body());
    let year = parser
        .read(ComboBoxLSDataCommand::new(year_def.clone()))?
        .key()
        .ok_or_else(|| {
            WebDynproError::Element(ElementError::NoSuchContent {
                element: year_def.id().to_string(),
                content: "No data provided".to_string(),
            })
        })?
        .parse::<u32>()
        .or(Err(WebDynproError::Element(ElementError::InvalidContent {
            element: year_def.id().to_string(),
            content: "Year cannot be parsed as u32".to_string(),
        })))?;
    let semester = match parser
        .read(ComboBoxLSDataCommand::new(semester_def.clone()))?
        .key()
        .ok_or_else(|| {
            WebDynproError::Element(ElementError::NoSuchContent {
                element: semester_def.id().to_string(),
                content: "No data provided".to_string(),
            })
        })?
        .as_str()
    {
        "090" => SemesterType::One,
        "091" => SemesterType::Summer,
        "092" => SemesterType::Two,
        "093" => SemesterType::Winter,
        _ => unreachable!("Invalid semester key"),
    };
    Ok((year, semester))
}
