use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub result: String,
    pub description: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            result: "Success".to_string(),
            description: "".to_string(),
        }
    }

    pub fn error(description: String) -> Self {
        Self {
            data: None,
            result: "Error".to_string(),
            description,
        }
    }
}
