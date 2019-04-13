pub use alcubierre_derive::*;
use inventory::iter;
pub use warp;
pub use inventory;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;
use std::net::SocketAddr;
use warp::reply::BoxedReply;

#[derive(Debug)]
pub struct Route {
    pub name: &'static str,
    pub mod_path: &'static str,
    pub filter: BoxedFilter<(BoxedReply, )>,
}

impl Route {
    fn clone_filter_rooted_at(&self, root_module: &str) -> Option<BoxedFilter<(BoxedReply, )>> {
        if !self.mod_path.starts_with(root_module) { return None; };

        // TODO: I kinda hate all those allocations

        let rooted = self.mod_path[root_module.len()..].split("::")
            .filter(|segment| !segment.is_empty())
            .map(|segment| warp::path(segment))
            .fold(warp::any().boxed(), |prefix, segment| prefix.and(segment).boxed());

        Some(rooted.and(self.filter.clone()).boxed())
    }
}

inventory::collect!(Route);

pub fn routes() -> Vec<&'static Route> {
    iter::<Route>.into_iter().collect()
}

pub fn all_routes_filter(root_module: &str) -> impl Filter<Extract = impl Reply> {
    // TODO: allocation galore - clone, boxed

    let mut result = warp::any().and_then(|| Err(warp::reject::not_found())).boxed();

    for filter in routes().into_iter().filter_map(|x| x.clone_filter_rooted_at(root_module)) {
        result = result.or(filter).unify().boxed()
    }

    result.with(warp::cors().allow_any_origin()) // TODO: allow for customizing that
}

pub fn engage(addr: impl Into<SocketAddr> + 'static) {
    warp::serve(all_routes_filter("")).run(addr)
}

pub fn engage_rooted(root_module: &str, addr: impl Into<SocketAddr> + 'static) {
    warp::serve(all_routes_filter(root_module)).run(addr)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
