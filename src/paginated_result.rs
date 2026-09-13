use text_search_core::Indexable;

pub struct PaginatedResult<T: Indexable> {
    pub data: Vec<T>,
    pub page: usize,
    pub total_pages: usize,
}

impl<T: Indexable> PaginatedResult<T> {
    pub fn new(data: Vec<T>, page: usize, total_pages: usize) -> Self {
        Self {
            data,
            page,
            total_pages,
        }
    }
}
