use core::{
    borrow::{Borrow, BorrowMut},
    iter::Iterator,
};

/// Collections that can provide iterator accessing to the view in the form of
/// `Borrow`) of the its items.
///
/// Arrays `[T; N]`, slices `[T]` and `&[T]`, `Option<T>` and `Result<T>` 
/// implements this trait.
pub trait TrItemsRefView {
    type Item: ?Sized;
    type View<'view>: Borrow<Self::Item>
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>>;
}

/// Collections that can provide iterator accessing to the view in the form of
/// `BorrowMut`) of the its items.
///
/// Arrays `[T; N]`, slices `[T]` and `&[T]`, `Option<T>` and `Result<T>` 
/// implements this trait.
pub trait TrItemsMutView {
    type Item: ?Sized;
    type View<'view>: BorrowMut<Self::Item>
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>>;
}

// for array [T; N]

impl<T, const N: usize> TrItemsRefView for [T; N] {
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T, const N: usize> TrItemsMutView for [T; N] {
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}

// --- for slice [T]

impl<T> TrItemsRefView for [T] {
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T> TrItemsMutView for [T] {
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}

// --- for slice &[T]

impl<T> TrItemsRefView for &[T] {
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T> TrItemsMutView for &mut [T] {
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}

// --- for Option<T>

impl<T> TrItemsRefView for Option<T> {
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T> TrItemsMutView for Option<T> {
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}

// --- for Result<T, E>

impl<T, E> TrItemsRefView for Result<T, E> {
    type Item = T;
    type View<'view> = &'view Self::Item
    where
        Self: 'view;

    fn items_ref_view(&self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter()
    }
}

impl<T, E> TrItemsMutView for Result<T, E> {
    type Item = T;
    type View<'view> = &'view mut Self::Item
    where
        Self: 'view;

    fn items_mut_view(&mut self) -> impl Iterator<Item = Self::View<'_>> {
        self.iter_mut()
    }
}
