use crate::webdynpro::{
    element::{Element, text::InputField},
    error::{ElementError, WebDynproError},
};

impl InputField<'_> {
    pub(crate) fn value_string(&self) -> Result<String, WebDynproError> {
        Ok(self
            .value()
            .ok_or_else(|| ElementError::NoSuchContent {
                element: self.id().to_owned(),
                content: "value of InputField".to_string(),
            })?
            .to_owned())
    }

    pub(crate) fn value_into_u32(&self) -> Result<u32, WebDynproError> {
        self.value_string()?.trim().parse::<u32>().map_err(|e| {
            log::error!(e:?; "failed to convert string to u32");
            ElementError::InvalidContent {
                element: self.id().to_owned(),
                content: "value is not correct u32".to_string(),
            }
            .into()
        })
    }

    pub(crate) fn value_into_f32(&self) -> Result<f32, WebDynproError> {
        self.value_string()?.trim().parse::<f32>().map_err(|_| {
            ElementError::InvalidContent {
                element: self.id().to_owned(),
                content: "value is not correct f32".to_string(),
            }
            .into()
        })
    }
}
