use std::marker::PhantomData;

use bevy_ecs::query::{QueryItem, ReadOnlyQueryData};

pub trait PredicateFilter {
    type Data: ReadOnlyQueryData;
    fn filter_predicate(item: QueryItem<Self::Data>) -> bool;
}

pub struct Predicate<P>(PhantomData<P>);
