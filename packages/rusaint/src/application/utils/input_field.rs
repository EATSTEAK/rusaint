use wdpe::{
    element::{Element, text::InputField},
    error::{ElementError, WebDynproError},
};

pub(crate) trait InputFieldExt {
    /// Returns the value of the input field as a string.
    fn value_string(&self) -> Result<String, WebDynproError>;

    /// Converts the value of the input field to a u32.
    fn value_into_u32(&self) -> Result<u32, WebDynproError>;

    /// Converts the value of the input field to a f32.
    fn value_into_f32(&self) -> Result<f32, WebDynproError>;
}

impl InputFieldExt for InputField<'_> {
    fn value_string(&self) -> Result<String, WebDynproError> {
        Ok(self
            .value()
            .ok_or_else(|| ElementError::NoSuchContent {
                element: self.id().to_owned(),
                content: "value of InputField".to_string(),
            })?
            .to_owned())
    }

    fn value_into_u32(&self) -> Result<u32, WebDynproError> {
        self.value_string()?.trim().parse::<u32>().map_err(|e| {
            tracing::error!(?e, "failed to convert string to u32");
            ElementError::InvalidContent {
                element: self.id().to_owned(),
                content: "value is not correct u32".to_string(),
            }
            .into()
        })
    }

    fn value_into_f32(&self) -> Result<f32, WebDynproError> {
        self.value_string()?.trim().parse::<f32>().map_err(|_| {
            ElementError::InvalidContent {
                element: self.id().to_owned(),
                content: "value is not correct f32".to_string(),
            }
            .into()
        })
    }
}
