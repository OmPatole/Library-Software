#[derive(Clone, Default)]
pub struct Transaction {
    pub id: i64,
    pub accession_no: String,
    pub user_id: String,
    pub user_name: String,
    pub issue_date: String,
    pub expected_return_date: String,
    pub actual_return_date: Option<String>,
    pub status: String, // Issued, Returned, Renewed
}
