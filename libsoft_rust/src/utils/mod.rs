// Utility functions

pub fn is_valid_phone(phone: &str) -> bool {
    phone.chars().all(char::is_numeric) && (phone.len() == 10 || phone.is_empty())
}

pub fn is_valid_price(price_str: &str) -> bool {
    price_str.parse::<f64>().is_ok()
}
