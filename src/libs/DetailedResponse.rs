pub struct DetailedResponse<T> {
    pub data: T,
    pub success: bool,
    pub message: String,
}