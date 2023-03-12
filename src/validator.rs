pub fn validate_date_for_birthday(day: u8, month: u8) -> Result<(), String> {
    if [1, 3, 5, 7, 8, 10, 12].contains(&month) && day > 0 && day <= 31 {
        return Ok(());
    } else if [4, 6, 9, 11].contains(&month) && day > 0 && day <= 30 {
        return Ok(());
    } else if month == 2 && day <= 29 && day > 0 {
        return Ok(());
    }

    Err("Invalid day and month".to_owned())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_date_validator() {
        assert_eq!(true, validate_date_for_birthday(12, 3).is_ok());
        assert_eq!(false, validate_date_for_birthday(31, 2).is_ok());
        assert_eq!(true, validate_date_for_birthday(30, 12).is_ok());
        assert_eq!(false, validate_date_for_birthday(0, 5).is_ok());
        assert_eq!(true, validate_date_for_birthday(1, 6).is_ok());
        assert_eq!(true, validate_date_for_birthday(29, 2).is_ok());
        assert_eq!(false, validate_date_for_birthday(0, 0).is_ok());
        assert_eq!(false, validate_date_for_birthday(34, 14).is_ok());
    }
}
