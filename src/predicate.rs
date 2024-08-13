use std::marker::PhantomData;

use bevy_ecs::query::{ROQueryItem, ReadOnlyQueryData};

/// A trait to power the machinery behind the [`Predicate`] query filter.
///
/// ### Associated types
///
/// - `Data`: Can be any set of components to run the predicate over
///
/// ### Methods
///
/// - `filter_predicate`: This is the predicate that entities have to pass to
/// be selected by the [`Query`](bevy_ecs::system::Query)
///
/// # Examples
///
/// ```
/// # use bevy_ecs::component::Component;
/// # use bevy_predicate_filter::prelude::{Predicate, PredicateFilter};
/// # use bevy_ecs::query::ROQueryItem;
/// # use bevy_ecs::system::Query;
///
/// #[derive(Component, Debug)]
/// struct BookPages(usize);
///
/// pub struct EmptyBooks;
///
/// impl PredicateFilter for EmptyBooks {
///     type Data = &'static BookPages;
///     fn filter_predicate(item: ROQueryItem<Self::Data>) -> bool {
///         item.0 == 0
///     }
/// }
/// ```
pub trait PredicateFilter {
    /// Any set of components to run the predicate over
    type Data: ReadOnlyQueryData;

    /// This is the predicate that entities have to pass to be selected by the
    /// [`Query`](bevy_ecs::system::Query) using the [`Predicate`] filter
    fn filter_predicate(item: ROQueryItem<Self::Data>) -> bool;
}

/// Filter that selects entities satisfying a predicate.
///
/// This can be used in a [`Query`](bevy_ecs::system::Query) if entities are
/// required to pass some predicate but you don't actually care about components
/// value and you don't wan to extend the query's data section. It's easier to
/// manage a bunch of queries which all need the same kind of predicate this way
/// since there is only a single implementation and the query types don't have
/// to be updated.
///
/// # Examples
///
/// ```
/// # use bevy_ecs::component::Component;
/// # use bevy_predicate_filter::prelude::{Predicate, PredicateFilter};
/// # use bevy_ecs::query::ROQueryItem;
/// # use bevy_ecs::system::Query;
/// #
/// # #[derive(Component, Debug)]
/// # struct FishName(String);
/// #
/// # #[derive(Component)]
/// # pub struct FishSize(usize);
///
/// pub struct BigFish;
///
/// impl PredicateFilter for BigFish {
///     type Data = &'static FishSize;
///     fn filter_predicate(item: ROQueryItem<Self::Data>) -> bool {
///         item.0 > 5
///     }
/// }
///
/// fn name_big_fish(query: Query<&FishName, Predicate<BigFish>>) {
///     for name in &query {
///         println!("{name:?} is looking huuuge today!");
///     }
/// }
///
/// # bevy_ecs::system::assert_is_system(name_big_fish);
/// ```
///
/// # Note
///
/// Please note that there is another also the alternative of using
/// [`SystemParam`](bevy_ecs::system::SystemParam) instead. This might
/// be more efficient since it uses less indirection compared to the
/// [`Predicate`] machinery.
///
///
/// ## Example
///
/// ```
/// # use bevy_ecs::component::Component;
/// # use bevy_ecs::system::SystemParam;
/// # use bevy_ecs::system::Query;
/// #
/// # #[derive(Component, Debug)]
/// # struct FishName(String);
/// #
/// # #[derive(Component)]
/// # pub struct FishSize(usize);
///
/// #[derive(SystemParam)]
/// pub struct BigFishParam<'w, 's> {
///     fish: Query<'w, 's, (&'static FishName, &'static FishSize)>
/// }
///
/// impl BigFishParam<'_, '_> {
///     pub fn iter_big_fish(&self) -> impl Iterator<Item = &FishName> + '_ {
///         self.fish.iter().filter(|(_, size)| size.0 > 5).map(|(name, _)| name)
///     }
/// }
///
/// fn name_big_fish(param: BigFishParam) {
///     for name in param.iter_big_fish() {
///         println!("{name:?} is looking huuuge today!");
///     }
/// }
///
/// # bevy_ecs::system::assert_is_system(name_big_fish);
/// ```
pub struct Predicate<P>(PhantomData<P>);
