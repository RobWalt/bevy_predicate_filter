use bevy_ecs::{
    prelude::*,
    query::{QueryFilter, ReadOnlyQueryData, WorldQuery},
    storage::TableRow,
};

use crate::prelude::{Predicate, PredicateFilter};

impl<D: ReadOnlyQueryData, P: PredicateFilter<Data = D>> QueryFilter for Predicate<P> {
    const IS_ARCHETYPAL: bool = false;

    unsafe fn filter_fetch(
        fetch: &mut Self::Fetch<'_>,
        entity: Entity,
        table_row: TableRow,
    ) -> bool {
        P::filter_predicate(Self::fetch(fetch, entity, table_row))
    }
}
