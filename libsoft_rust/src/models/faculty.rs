#[derive(Clone, Default)]
pub struct Faculty {
    pub uid: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub joining_date: String,
    pub joining_under: String,
    pub branch: String,
    pub mobile_no: String,
    pub email: String,
    pub address: String,
    pub is_active: bool,
    pub total_due: f64,
}
