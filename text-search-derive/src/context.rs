use std::cell::RefCell;

pub struct Ctxt {
    errors: RefCell<Option<Vec<syn::Error>>>,
}

impl Ctxt {
    pub fn new() -> Self {
        Ctxt {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn syn_error(&self, err: syn::Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(err);
    }

    // pub fn check(self) -> syn::Result<()> {
    //     let mut errors = self.errors.borrow_mut().take().unwrap().into_iter();

    //     let mut combined = match errors.next() {
    //         Some(first) => first,
    //         None => return Ok(()),
    //     };

    //     for rest in errors {
    //         combined.combine(rest);
    //     }

    //     Err(combined)
    // }
}