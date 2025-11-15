use chrono::{Datelike, NaiveDate, Utc};

/// Calculate age from date of birth
pub fn calculate_age(date_of_birth: NaiveDate) -> i32 {
    let today = Utc::now().naive_utc().date();
    let age = today.year() - date_of_birth.year();

    if today.month() < date_of_birth.month()
        || (today.month() == date_of_birth.month() && today.day() < date_of_birth.day())
    {
        age - 1
    } else {
        age
    }
}

/// Generate check digit using Luhn algorithm
pub fn calculate_luhn_check_digit(input: &str) -> char {
    let sum: u32 = input
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
        .map(|(i, digit)| {
            if i % 2 == 0 {
                let doubled = digit * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                digit
            }
        })
        .sum();

    let check = (10 - (sum % 10)) % 10;
    char::from_digit(check, 10).unwrap()
}

/// Validate Luhn check digit
pub fn validate_luhn(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    let (code, check_digit) = input.split_at(input.len() - 1);
    let calculated_check = calculate_luhn_check_digit(code);

    check_digit.chars().next() == Some(calculated_check)
}

/// Mask sensitive data
pub fn mask_phone_number(phone: &str) -> String {
    if phone.len() <= 4 {
        return phone.to_string();
    }

    let visible = &phone[phone.len() - 4..];
    format!("XXXXXX{}", visible)
}

pub fn mask_aadhaar(aadhaar: &str) -> String {
    if aadhaar.len() != 12 {
        return aadhaar.to_string();
    }

    format!(
        "XXXX-XXXX-{}",
        &aadhaar[aadhaar.len() - 4..]
    )
}

pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let (username, domain) = email.split_at(at_pos);
        if username.len() <= 2 {
            return email.to_string();
        }

        let first = &username[..1];
        let last = &username[username.len() - 1..];
        format!("{}****{}{}", first, last, domain)
    } else {
        email.to_string()
    }
}

/// Phone number formatting
pub fn format_indian_phone(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() == 10 {
        format!("+91{}", digits)
    } else if digits.len() == 12 && digits.starts_with("91") {
        format!("+{}", digits)
    } else {
        digits
    }
}

/// Validate Indian phone number
pub fn is_valid_indian_phone(phone: &str) -> bool {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() == 10 {
        // Must start with 6, 7, 8, or 9
        digits.starts_with('6')
            || digits.starts_with('7')
            || digits.starts_with('8')
            || digits.starts_with('9')
    } else if digits.len() == 12 {
        digits.starts_with("91")
            && (digits.chars().nth(2) == Some('6')
                || digits.chars().nth(2) == Some('7')
                || digits.chars().nth(2) == Some('8')
                || digits.chars().nth(2) == Some('9'))
    } else {
        false
    }
}

/// Validate Aadhaar number
pub fn is_valid_aadhaar(aadhaar: &str) -> bool {
    let digits: String = aadhaar.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.len() == 12 && !digits.starts_with('0') && !digits.starts_with('1')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_age() {
        let dob = NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
        let age = calculate_age(dob);
        assert!(age >= 33 && age <= 35); // Depends on current date
    }

    #[test]
    fn test_luhn_algorithm() {
        let code = "202511050000";
        let check_digit = calculate_luhn_check_digit(code);
        let full_code = format!("{}{}", code, check_digit);

        assert!(validate_luhn(&full_code));
        assert!(!validate_luhn("202511050000X"));
    }

    #[test]
    fn test_mask_phone() {
        assert_eq!(mask_phone_number("9876543210"), "XXXXXX3210");
    }

    #[test]
    fn test_mask_aadhaar() {
        assert_eq!(mask_aadhaar("123456789012"), "XXXX-XXXX-9012");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("rajesh@example.com"), "r****h@example.com");
    }

    #[test]
    fn test_indian_phone_validation() {
        assert!(is_valid_indian_phone("9876543210"));
        assert!(is_valid_indian_phone("919876543210"));
        assert!(!is_valid_indian_phone("1234567890"));
        assert!(!is_valid_indian_phone("12345"));
    }

    #[test]
    fn test_aadhaar_validation() {
        assert!(is_valid_aadhaar("234567890123"));
        assert!(!is_valid_aadhaar("123456789012")); // Starts with 0 or 1
        assert!(!is_valid_aadhaar("12345")); // Too short
    }
}
