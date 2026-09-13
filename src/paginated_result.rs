use text_search_core::Indexable;

pub struct PaginatedResult<T: Indexable> {
    pub data: Vec<T>,
    pub page: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

impl<T: Indexable> PaginatedResult<T> {
    pub fn new(data: Vec<T>, page: i64, total_items: i64, total_pages: i64) -> Self {
        Self {
            data,
            page,
            total_items,
            total_pages,
        }
    }
}
