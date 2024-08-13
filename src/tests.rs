use bevy_ecs::{component::Component, query::ROQueryItem, world::World};

use crate::prelude::{Predicate, PredicateFilter};

#[test]
fn query_predicate_basic() {
    #[derive(Component, Debug, PartialEq)]
    struct FishName(String);

    #[derive(Component)]
    pub struct FishSize(usize);

    pub struct BigFish;

    impl PredicateFilter for BigFish {
        type Data = &'static FishSize;
        fn filter_predicate(item: ROQueryItem<Self::Data>) -> bool {
            item.0 > 5
        }
    }

    let mut world = World::new();
    world.spawn((FishName(String::from("Albert")), FishSize(3)));
    world.spawn((FishName(String::from("Freddy")), FishSize(8)));
    world.spawn((FishName(String::from("Ronny")), FishSize(10)));

    let values = world
        .query_filtered::<&FishName, Predicate<BigFish>>()
        .iter(&mut world)
        .collect::<Vec<_>>();
    assert_eq!(
        values,
        vec![
            &FishName(String::from("Freddy")),
            &FishName(String::from("Ronny"))
        ]
    );
}
