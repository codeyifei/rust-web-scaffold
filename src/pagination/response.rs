use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::pagination::types::{PaginationMeta, PaginationQueryMeta};
use crate::result::response::AppResponse;

#[derive(Serialize, Debug)]
pub struct PaginationResponse<T>
where
    T: Serialize,
{
    pub list: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T> From<(Vec<T>, u64, PaginationQueryMeta)> for PaginationResponse<T>
where
    T: Serialize,
{
    fn from((list, total, meta): (Vec<T>, u64, PaginationQueryMeta)) -> Self {
        (list, total, meta.get_page(), meta.get_page_size()).into()
    }
}

impl<T> From<(Vec<T>, u64, u64, u64)> for PaginationResponse<T>
where
    T: Serialize,
{
    fn from((list, total, page, page_size): (Vec<T>, u64, u64, u64)) -> Self {
        Self {
            list,
            meta: PaginationMeta {
                total,
                page,
                page_size,
            },
        }
    }
}

impl<T> IntoResponse for PaginationResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        AppResponse::data(PaginationResponse::from((
            self.list,
            self.meta.total,
            self.meta.page,
            self.meta.page_size,
        )))
            .into_response()
    }
}
