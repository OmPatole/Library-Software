#[derive(Clone, Default)]
pub struct Member {
    pub id: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub admission_year: String,
    pub course: String,
    pub current_year: String,
    pub branch: String,
    pub mobile_no: String,
    pub email: String,
    pub address: String,
    pub is_active: bool,
    pub total_due: f64,
}
