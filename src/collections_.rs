use std::{
    alloc::Allocator,
    collections::{BTreeSet, LinkedList, VecDeque},
    vec::Vec,
};

use crate::{TrItemsMutView, TrItemsRefView};

// for BTreeSet

impl<T, A> TrItemsRefView for BTreeSet<T, A>
where
    A: Allocator + Clone,
{
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

// for LinkedList

impl<T, A> TrItemsRefView for LinkedList<T, A>
where
    A: Allocator,
{
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T, A> TrItemsMutView for LinkedList<T, A>
where
    A: Allocator,
{
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}

// for Vec

impl<T, A> TrItemsRefView for Vec<T, A>
where
    A: Allocator,
{
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T, A> TrItemsMutView for Vec<T, A>
where
    A: Allocator,
{
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}

// for VecDeque

impl<T, A> TrItemsRefView for VecDeque<T, A>
where
    A: Allocator,
{
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T, A> TrItemsMutView for VecDeque<T, A>
where
    A: Allocator,
{
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}
