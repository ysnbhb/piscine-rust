pub use chrono::Utc;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.to_string(),
                "Username is empty",
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.to_string(),
                "Password should be at least 8 characters long",
            ));
        }
        let mut is_nurec = false;
        let mut is_lettr = false;
        let mut is_symbols = false;
        for i in self.password.chars() {
            if i.is_ascii_alphabetic() {
                is_lettr = true
            } else if i.is_numeric() {
                is_nurec = true
            } else if !i.is_ascii_alphanumeric() {
                is_symbols = true
            }
        }
        if !is_nurec || !is_lettr || !is_symbols {
            return Err(FormError::new(
                "password",
                self.password.to_string(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }
        Ok(())
    }
}
