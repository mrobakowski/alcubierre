pub use alcubierre_derive::*;
use std::any::Any;
use inventory::iter;

#[derive(Debug)]
pub struct Route {
    pub name: &'static str,
    pub path: &'static str,
    pub func: Box<Any>
}

impl Route {
    pub fn new<F: Any>(name: &'static str, path: &'static str, func: F) -> Route {
        Route {
            name,
            path,
            func: Box::new(func)
        }
    }
}

inventory::collect!(Route);

pub fn routes() -> Vec<&'static Route> {
    iter::<Route>.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::Route;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_construct_route_from_fn() {
        let rt = Route::new("foo", "/bar", it_works);
    }
}
