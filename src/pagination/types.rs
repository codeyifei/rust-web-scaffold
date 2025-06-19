use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PaginationQueryMeta {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_page_size")]
    page_size: u64,
}

impl PaginationQueryMeta {
    pub fn get_page(self) -> u64 {
        self.page
    }

    pub fn get_page_size(self) -> u64 {
        self.page_size
    }
}

pub fn default_page() -> u64 {
    1
}

pub fn default_page_size() -> u64 {
    20
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}