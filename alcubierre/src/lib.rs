pub use alcubierre_derive::*;
use inventory::iter;
pub use warp;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;
use futures::Future;
use std::net::SocketAddr;
use warp::reply::BoxedReply;

#[derive(Debug)]
pub struct Route {
    pub name: &'static str,
    pub path: &'static str,
    pub filter: BoxedFilter<(BoxedReply, )>,
}

inventory::collect!(Route);

pub fn routes() -> Vec<&'static Route> {
    iter::<Route>.into_iter().collect()
}

pub fn all_routes_filter() -> BoxedFilter<(BoxedReply, )> {
    let mut routes = routes();

    // TODO: allocation galore - clone, boxed

    let mut result = routes.pop().unwrap().filter.clone();
    for route in routes.into_iter().rev() {
        result = result.or(route.filter.clone()).unify().boxed();
    }

    result
}

pub fn engage(addr: impl Into<SocketAddr> + 'static) {
    warp::serve(all_routes_filter()).run(addr)
}

#[cfg(test)]
mod tests {
    use crate::Route;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
