pub struct ResponseWrapper<T> {
    pub message: String,
    pub status: u16,
    pub data: T,
}