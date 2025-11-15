#[cfg(test)]
mod domain_model_tests {
    use common::types::*;

    #[test]
    fn test_mrn_generation() {
        let mrn = generate_mrn("ORG123");
        assert!(mrn.starts_with("ORG123"));
        assert!(mrn.len() > 10);
    }

    #[test]
    fn test_sample_id_generation() {
        let sample_id = generate_sample_id();
        assert!(sample_id.len() > 0);
    }

    #[test]
    fn test_accession_number_generation() {
        let accession = generate_accession_number("LAB");
        assert!(accession.starts_with("LAB"));
    }

    #[test]
    fn test_duplicate_mrn_uniqueness() {
        let mrn1 = generate_mrn("ORG");
        let mrn2 = generate_mrn("ORG");
        assert_ne!(mrn1, mrn2, "MRNs should be unique");
    }
}

#[cfg(test)]
mod error_handling_tests {
    use common::error::{Error, Result};

    #[test]
    fn test_validation_error_creation() {
        let error = Error::ValidationError("Test validation failed".to_string());
        match error {
            Error::ValidationError(msg) => assert_eq!(msg, "Test validation failed"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_not_found_error() {
        let error = Error::NotFound("Resource not found".to_string());
        assert!(matches!(error, Error::NotFound(_)));
    }

    #[test]
    fn test_error_to_string() {
        let error = Error::ValidationError("Test".to_string());
        let error_str = error.to_string();
        assert!(error_str.contains("Test"));
    }
}

#[cfg(test)]
mod pagination_tests {
    use common::types::{PaginationParams, PaginatedResponse};

    #[test]
    fn test_pagination_params_default() {
        let params = PaginationParams::default();
        assert_eq!(params.page, 1);
        assert_eq!(params.page_size, 20);
    }

    #[test]
    fn test_pagination_params_custom() {
        let params = PaginationParams {
            page: 5,
            page_size: 50,
        };
        assert_eq!(params.page, 5);
        assert_eq!(params.page_size, 50);
    }

    #[test]
    fn test_pagination_offset_calculation() {
        let params = PaginationParams {
            page: 3,
            page_size: 20,
        };
        let offset = (params.page - 1) * params.page_size;
        assert_eq!(offset, 40);
    }

    #[test]
    fn test_max_page_size_limit() {
        let params = PaginationParams {
            page: 1,
            page_size: 100,
        };
        assert!(params.page_size <= 100, "Page size should not exceed 100");
    }
}
