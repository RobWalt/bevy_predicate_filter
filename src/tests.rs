use bevy_ecs::{component::Component, query::QueryItem, world::World};

use crate::prelude::{Predicate, PredicateFilter};

#[derive(Component, Debug, PartialEq)]
struct A(usize);

#[derive(Component)]
pub struct B(usize);

pub struct NonZeroB;

impl PredicateFilter for NonZeroB {
    type Data = &'static B;
    fn filter_predicate(item: QueryItem<Self::Data>) -> bool {
        item.0 != 0
    }
}

#[test]
fn query_predicate_basic() {
    let mut world = World::new();
    world.spawn((A(1), B(0)));
    world.spawn((A(2), B(1)));
    world.spawn((A(3), B(2)));

    let values = world
        .query_filtered::<&A, Predicate<NonZeroB>>()
        .iter(&mut world)
        .collect::<Vec<_>>();
    assert_eq!(values, vec![&A(2), &A(3)]);
}
