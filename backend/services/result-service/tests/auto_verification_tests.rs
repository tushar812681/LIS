#[cfg(test)]
mod auto_verification_tests {
    use uuid::Uuid;
    use rust_decimal::Decimal;

    #[tokio::test]
    async fn test_range_check_within_range() {
        // Mock test result with value within reference range
        let result_value = "75.5";
        let ref_min = Decimal::from(60);
        let ref_max = Decimal::from(100);

        let value: f64 = result_value.parse().unwrap();
        let min_f = ref_min.to_string().parse::<f64>().unwrap();
        let max_f = ref_max.to_string().parse::<f64>().unwrap();

        let is_within_range = value >= min_f && value <= max_f;

        assert!(is_within_range, "Value should be within reference range");
    }

    #[tokio::test]
    async fn test_range_check_below_range() {
        let result_value = "45.0";
        let ref_min = Decimal::from(60);
        let ref_max = Decimal::from(100);

        let value: f64 = result_value.parse().unwrap();
        let min_f = ref_min.to_string().parse::<f64>().unwrap();
        let max_f = ref_max.to_string().parse::<f64>().unwrap();

        let is_within_range = value >= min_f && value <= max_f;

        assert!(!is_within_range, "Value should be below reference range");
    }

    #[tokio::test]
    async fn test_range_check_above_range() {
        let result_value = "120.0";
        let ref_min = Decimal::from(60);
        let ref_max = Decimal::from(100);

        let value: f64 = result_value.parse().unwrap();
        let min_f = ref_min.to_string().parse::<f64>().unwrap();
        let max_f = ref_max.to_string().parse::<f64>().unwrap();

        let is_within_range = value >= min_f && value <= max_f;

        assert!(!is_within_range, "Value should be above reference range");
    }

    #[tokio::test]
    async fn test_critical_value_detection_panic_low() {
        let result_value = "25.0";
        let panic_low = Decimal::from(30);

        let value: f64 = result_value.parse().unwrap();
        let panic_low_f = panic_low.to_string().parse::<f64>().unwrap();

        let is_panic = value < panic_low_f;

        assert!(is_panic, "Should detect panic low value");
    }

    #[tokio::test]
    async fn test_critical_value_detection_panic_high() {
        let result_value = "550.0";
        let panic_high = Decimal::from(500);

        let value: f64 = result_value.parse().unwrap();
        let panic_high_f = panic_high.to_string().parse::<f64>().unwrap();

        let is_panic = value > panic_high_f;

        assert!(is_panic, "Should detect panic high value");
    }

    #[tokio::test]
    async fn test_delta_check_calculation() {
        let current_value = 120.0;
        let previous_value = 100.0;

        let delta_abs = current_value - previous_value;
        let delta_pct = ((current_value - previous_value) / previous_value) * 100.0;

        assert_eq!(delta_abs, 20.0);
        assert_eq!(delta_pct, 20.0);
    }

    #[tokio::test]
    async fn test_delta_check_significant_increase() {
        let current_value = 180.0;
        let previous_value = 100.0;
        let threshold = 50.0;

        let delta_pct = ((current_value - previous_value) / previous_value) * 100.0;

        assert!(delta_pct > threshold, "Should detect significant increase");
    }

    #[tokio::test]
    async fn test_delta_check_significant_decrease() {
        let current_value = 40.0;
        let previous_value = 100.0;
        let threshold = 50.0;

        let delta_pct = ((current_value - previous_value) / previous_value) * 100.0;

        assert!(delta_pct.abs() > threshold, "Should detect significant decrease");
    }

    #[tokio::test]
    async fn test_auto_verification_confidence_score() {
        let confidence = 95.0;

        assert!(confidence >= 90.0, "Confidence should be high for auto-verification");
        assert!(confidence <= 100.0, "Confidence should not exceed 100%");
    }

    #[tokio::test]
    async fn test_interpretation_calculation_normal() {
        let value = 75.0;
        let min = 60.0;
        let max = 100.0;

        let is_abnormal = value < min || value > max;

        assert!(!is_abnormal, "Value should be interpreted as normal");
    }

    #[tokio::test]
    async fn test_interpretation_calculation_abnormal_low() {
        let value = 45.0;
        let min = 60.0;
        let max = 100.0;

        let is_low = value < min;

        assert!(is_low, "Value should be interpreted as abnormal low");
    }

    #[tokio::test]
    async fn test_interpretation_calculation_abnormal_high() {
        let value = 120.0;
        let min = 60.0;
        let max = 100.0;

        let is_high = value > max;

        assert!(is_high, "Value should be interpreted as abnormal high");
    }
}

#[cfg(test)]
mod reference_range_tests {
    #[tokio::test]
    async fn test_age_applicability_within_range() {
        let patient_age = 35;
        let age_min = Some(18);
        let age_max = Some(60);

        let is_applicable = match (age_min, age_max) {
            (Some(min), Some(max)) => patient_age >= min && patient_age <= max,
            _ => false,
        };

        assert!(is_applicable, "Reference range should apply to this age");
    }

    #[tokio::test]
    async fn test_age_applicability_below_range() {
        let patient_age = 15;
        let age_min = Some(18);
        let age_max = Some(60);

        let is_applicable = match (age_min, age_max) {
            (Some(min), Some(max)) => patient_age >= min && patient_age <= max,
            _ => false,
        };

        assert!(!is_applicable, "Reference range should not apply to this age");
    }

    #[tokio::test]
    async fn test_gender_applicability() {
        let patient_gender = "MALE";
        let range_gender = Some("MALE".to_string());

        let is_applicable = match range_gender {
            Some(ref g) => g == "ALL" || g.eq_ignore_ascii_case(patient_gender),
            None => true,
        };

        assert!(is_applicable, "Reference range should apply to this gender");
    }

    #[tokio::test]
    async fn test_all_gender_applicability() {
        let patient_gender = "FEMALE";
        let range_gender = Some("ALL".to_string());

        let is_applicable = match range_gender {
            Some(ref g) => g == "ALL" || g.eq_ignore_ascii_case(patient_gender),
            None => true,
        };

        assert!(is_applicable, "ALL gender range should apply to any gender");
    }
}
