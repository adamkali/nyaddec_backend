use serde;

#[derive(serde::Serialize)]
pub struct DetailedResponse<T> {
    pub data: Option<T>,
    pub success: bool,
    pub message: String,
}

impl<T> DetailedResponse::<T> {
    pub fn new() -> DetailedResponse<T> {
        DetailedResponse {
            data: None,
            success: false,
            message: "".to_string(),
        }
    }
}


