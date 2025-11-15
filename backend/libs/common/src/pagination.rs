use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInput {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PaginationInput {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl PaginationInput {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> u32 {
        self.page_size
    }

    pub fn validate(&self) -> bool {
        self.page >= 1 && self.page_size >= 1 && self.page_size <= 100
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub current_page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl PageInfo {
    pub fn new(pagination: &PaginationInput, total_count: u64) -> Self {
        let total_pages = ((total_count as f64) / (pagination.page_size as f64)).ceil() as u32;

        Self {
            current_page: pagination.page,
            page_size: pagination.page_size,
            total_count,
            total_pages,
            has_next_page: pagination.page < total_pages,
            has_previous_page: pagination.page > 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge<T> {
    pub node: T,
    pub cursor: String,
}

impl<T> Connection<T> {
    pub fn new(items: Vec<T>, pagination: &PaginationInput, total_count: u64) -> Self
    where
        T: Clone,
    {
        let edges = items
            .into_iter()
            .enumerate()
            .map(|(i, node)| Edge {
                cursor: general_purpose::STANDARD.encode(format!("{}:{}", pagination.page, i)),
                node,
            })
            .collect();

        let page_info = PageInfo::new(pagination, total_count);

        Self { edges, page_info }
    }
}

// Type aliases for backwards compatibility
pub type PaginationParams = PaginationInput;
pub type Paginated<T> = Connection<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let pagination = PaginationInput {
            page: 3,
            page_size: 20,
        };

        assert_eq!(pagination.offset(), 40);
        assert_eq!(pagination.limit(), 20);
    }

    #[test]
    fn test_page_info() {
        let pagination = PaginationInput {
            page: 2,
            page_size: 20,
        };

        let page_info = PageInfo::new(&pagination, 100);

        assert_eq!(page_info.total_pages, 5);
        assert!(page_info.has_next_page);
        assert!(page_info.has_previous_page);
    }
}
