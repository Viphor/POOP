pub struct CodegenError {
    message: String,
}

impl CodegenError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn error<T: Into<String>>(message: T) -> Self {
        Self::new(message)
    }
}
