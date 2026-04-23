#[derive(Clone, Default)]
pub struct Book {
    pub accession_no: String,
    pub call_no: String,
    pub title: String,
    pub author: String,
    pub branch: String,
    pub publisher: String,
    pub price: f64,
    pub bill_no: String,
    pub status: String, // Available, Issued
}
