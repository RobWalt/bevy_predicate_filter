mod predicate;
mod query_filter_impl;
mod world_query_impl;

pub mod prelude {
    pub use super::predicate::{Predicate, PredicateFilter};
}

#[cfg(test)]
mod tests;
