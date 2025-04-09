use chrono::Utc;  // Add this import

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values: (field_name.to_string(), field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),  // Use Utc::now() here
            err: err.to_string(),
        }
    }
}
